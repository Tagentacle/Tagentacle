use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::process::Command;
use tokio::sync::{mpsc, Mutex};
use tokio_util::codec::{Framed, LinesCodec};
use uuid::Uuid;

#[derive(Parser)]
#[command(name = "tagentacle")]
#[command(about = "Tagentacle: ROS for the AI Era", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Starts the local Tagentacle message bus daemon
    Daemon {
        #[arg(short, long, default_value = "127.0.0.1:19999")]
        addr: String,
    },
    /// Bridges an external MCP Server (stdio) to the Tagentacle bus
    Bridge {
        /// Command to start the MCP server (e.g., 'npx -y @modelcontextprotocol/server-sqlite')
        #[arg(long)]
        mcp: String,
        /// Optional node ID for this bridge (default: pseudo-random)
        #[arg(short, long)]
        node_id: Option<String>,
    },
    /// Topic introspection commands
    Topic {
        #[command(subcommand)]
        action: TopicAction,
    },
    /// Service introspection commands
    Service {
        #[command(subcommand)]
        action: ServiceAction,
    },
    /// Health check: verify daemon connectivity
    Doctor {
        #[arg(short, long, default_value = "127.0.0.1:19999")]
        addr: String,
    },
}

#[derive(Subcommand)]
enum TopicAction {
    /// Subscribe to a topic and print messages in real-time
    Echo {
        /// Topic to subscribe to (e.g., /chat/global)
        topic: String,
        #[arg(short, long, default_value = "127.0.0.1:19999")]
        addr: String,
    },
}

#[derive(Subcommand)]
enum ServiceAction {
    /// Call a service with a JSON payload
    Call {
        /// Service name (e.g., /math/add)
        service: String,
        /// JSON payload (e.g., '{"a": 1, "b": 2}')
        payload: String,
        #[arg(short, long, default_value = "127.0.0.1:19999")]
        addr: String,
    },
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "op")]
#[serde(rename_all = "snake_case")]
enum Action {
    Subscribe {
        topic: String,
        node_id: String,
    },
    Publish {
        topic: String,
        sender: String,
        payload: Value,
    },
    AdvertiseService {
        service: String,
        node_id: String,
    },
    CallService {
        service: String,
        request_id: String,
        payload: Value,
        caller_id: String,
    },
    ServiceResponse {
        service: String,
        request_id: String,
        payload: Value,
        caller_id: String,
    },
    Message {
        topic: String,
        sender: String,
        payload: Value,
    },
}

// --- Daemon Logic ---

struct Router {
    // Topic subscriptions: topic -> Vec<(node_id, tx)>
    subscriptions: HashMap<String, Vec<(String, mpsc::UnboundedSender<Value>)>>,
    // Service registry: service_name -> (node_id, tx)
    services: HashMap<String, (String, mpsc::UnboundedSender<Value>)>,
    // Node registry: node_id -> tx (used for direct routing of Service Responses and Callbacks)
    nodes: HashMap<String, mpsc::UnboundedSender<Value>>,
    // TODO: Global Parameter Server (ROS-like /param/get and /param/set)
    // parameters: HashMap<String, Value>,
}

// TODO: Built-in Command Tasks for Tagentacle CLI:
// 1. tagentacle node list: List all connected nodes.
// 2. tagentacle topic echo <topic>: CLI tool to peek into bus traffic (similar to ros2 topic echo).
// 3. tagentacle service call <srv> <json_args>: CLI tool to test services.
// 4. tagentacle doctor: Health check (daemon status, node connectivity).

// TODO: Implement Node Lifecycle Tracking (Heartbeats / Liveliness)
// Tagentacle nodes should be monitored so that if a node crashes, 
// the bus can cleanup its services and notify subscribers.

// TODO: Built-in 'launch' Command Logic
// This should parse a launch.yaml file containing:
// - List of nodes to run
// - Environment variables (e.g., TAGENTACLE_DAEMON_URL)
// - Remapping rules (e.g., map /camera/image to /sys/camera/image)
// - Logging redirection (stdout -> log file/bus topic).

type SharedRouter = Arc<Mutex<Router>>;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Daemon { addr }) => {
            run_daemon(addr).await?;
        }
        Some(Commands::Bridge { mcp, node_id }) => {
            run_bridge(mcp, node_id).await?;
        }
        Some(Commands::Topic { action }) => match action {
            TopicAction::Echo { topic, addr } => {
                run_topic_echo(addr, topic).await?;
            }
        },
        Some(Commands::Service { action }) => match action {
            ServiceAction::Call { service, payload, addr } => {
                run_service_call(addr, service, payload).await?;
            }
        },
        Some(Commands::Doctor { addr }) => {
            run_doctor(addr).await?;
        }
        None => {
            // Default to daemon for backward compatibility
            run_daemon("127.0.0.1:19999".to_string()).await?;
        }
    }

    Ok(())
}

async fn run_daemon(addr: String) -> Result<()> {
    let listener = TcpListener::bind(&addr).await?;
    println!("Tagentacle Daemon listening on: {}", addr);

    let router = Arc::new(Mutex::new(Router {
        subscriptions: HashMap::new(),
        services: HashMap::new(),
        nodes: HashMap::new(),
    }));

    loop {
        let (socket, _) = listener.accept().await?;
        let router = Arc::clone(&router);
        tokio::spawn(async move {
            if let Err(e) = handle_daemon_connection(socket, router).await {
                eprintln!("Daemon connection error: {}", e);
            }
        });
    }
}

async fn handle_daemon_connection(socket: TcpStream, router: SharedRouter) -> Result<()> {
    let codec = LinesCodec::new();
    let mut framed = Framed::new(socket, codec);
    let (tx, mut rx) = mpsc::unbounded_channel::<Value>();
    let mut current_node_id: Option<String> = None;

    loop {
        tokio::select! {
            line = framed.next() => {
                match line {
                    Some(Ok(msg)) => {
                        let action: Action = match serde_json::from_str(&msg) {
                            Ok(a) => a,
                            Err(e) => {
                                eprintln!("Failed to parse JSON: {} | Original: {}", e, msg);
                                continue;
                            }
                        };
                        match action {
                            Action::Subscribe { topic, node_id } => {
                                current_node_id = Some(node_id.clone());
                                let mut r = router.lock().await;
                                r.nodes.insert(node_id.clone(), tx.clone());
                                r.subscriptions.entry(topic).or_default().push((node_id, tx.clone()));
                            }
                            Action::Publish { topic, sender, payload } => {
                                let r = router.lock().await;
                                if let Some(subs) = r.subscriptions.get(&topic) {
                                    let push = json!({"op": "message", "topic": topic, "sender": sender, "payload": payload});
                                    for (_, sub_tx) in subs { let _ = sub_tx.send(push.clone()); }
                                }
                            }
                            Action::AdvertiseService { service, node_id } => {
                                current_node_id = Some(node_id.clone());
                                let mut r = router.lock().await;
                                r.nodes.insert(node_id.clone(), tx.clone());
                                r.services.insert(service, (node_id, tx.clone()));
                            }
                            Action::CallService { service, request_id, payload, caller_id } => {
                                let mut r = router.lock().await;
                                r.nodes.insert(caller_id.clone(), tx.clone());
                                if let Some((_, srv_tx)) = r.services.get(&service) {
                                    let _ = srv_tx.send(json!({"op": "call_service", "service": service, "request_id": request_id, "payload": payload, "caller_id": caller_id}));
                                }
                            }
                            Action::ServiceResponse { service, request_id, payload, caller_id } => {
                                let r = router.lock().await;
                                if let Some(caller_tx) = r.nodes.get(&caller_id) {
                                    let _ = caller_tx.send(json!({"op": "service_response", "service": service, "request_id": request_id, "payload": payload, "caller_id": caller_id}));
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => break,
                }
            }
            msg = rx.recv() => {
                if let Some(msg) = msg {
                    if let Ok(json_line) = serde_json::to_string(&msg) {
                        let _ = framed.send(json_line).await;
                    }
                } else { break; }
            }
        }
    }
    if let Some(node_id) = current_node_id { println!("Node '{}' disconnected", node_id); }
    Ok(())
}

// --- Bridge Logic ---

async fn run_bridge(mcp_cmd: String, node_id_opt: Option<String>) -> Result<()> {
    let node_id = node_id_opt.unwrap_or_else(|| format!("bridge_{}", &Uuid::new_v4().to_string()[..8]));
    let rpc_service = format!("/mcp/{}/rpc", node_id);
    let audit_topic = "/mcp/traffic";

    println!("Starting Bridge Node: {}", node_id);
    println!("MCP Command: {}", mcp_cmd);

    // 1. Connect to Daemon
    let stream = TcpStream::connect("127.0.0.1:19999").await
        .context("Failed to connect to Tagentacle Daemon. Is it running?")?;
    let mut framed = Framed::new(stream, LinesCodec::new());

    // 2. Advertise Tunnel Service
    let advertise = json!({
        "op": "advertise_service",
        "service": rpc_service,
        "node_id": node_id
    });
    framed.send(advertise.to_string()).await?;

    // 3. Start Subprocess
    let mut child = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", &mcp_cmd]).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?
    } else {
        Command::new("sh").args(["-c", &mcp_cmd]).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?
    };

    let mut child_stdin = child.stdin.take().unwrap();
    let child_stdout = child.stdout.take().unwrap();
    let mut child_reader = BufReader::new(child_stdout).lines();

    println!("Bridge ready. Listening for Tagentacle calls on {}...", rpc_service);

    // Track active requests to map responses back
    // Mapping: JSON-RPC ID (from client req) -> (Service Request ID, Caller ID)
    let mut pending_calls: HashMap<Value, (String, String)> = HashMap::new();

    loop {
        tokio::select! {
            // A. From Tagentacle Bus -> To MCP Subprocess (stdin)
            bus_msg = framed.next() => {
                if let Some(Ok(line)) = bus_msg {
                    let action: Action = serde_json::from_str(&line)?;
                    if let Action::CallService { payload, request_id, caller_id, .. } = action {
                        // Extract JSON-RPC ID to track it
                        if let Some(mcp_id) = payload.get("id") {
                            pending_calls.insert(mcp_id.clone(), (request_id, caller_id));
                        }

                        // Forward JSON-RPC payload to stdin
                        let mcp_raw = serde_json::to_string(&payload)?;
                        child_stdin.write_all(mcp_raw.as_bytes()).await?;
                        child_stdin.write_all(b"\n").await?;
                        child_stdin.flush().await?;
                        
                        // Mirrror to audit topic
                        let audit = json!({
                            "op": "publish", "topic": audit_topic, "sender": node_id,
                            "payload": {"direction": "bus_to_mcp", "data": payload}
                        });
                        let _ = framed.send(audit.to_string()).await;
                    }
                } else { break; }
            }

            // B. From MCP Subprocess (stdout) -> To Tagentacle Bus (ServiceResponse / CallService)
            mcp_line = child_reader.next_line() => {
                if let Ok(Some(line)) = mcp_line {
                    let mcp_val: Value = match serde_json::from_str(&line) {
                        Ok(v) => v,
                        Err(_) => continue,
                    };
                    
                    // Mirroring
                    let audit = json!({
                        "op": "publish", "topic": audit_topic, "sender": node_id,
                        "payload": {"direction": "mcp_to_bus", "data": mcp_val}
                    });
                    let _ = framed.send(audit.to_string()).await;

                    // If it's a Response (has 'id')
                    if let Some(mcp_id) = mcp_val.get("id") {
                        if let Some((req_id, caller_id)) = pending_calls.remove(mcp_id) {
                            let resp = json!({
                                "op": "service_response",
                                "service": rpc_service,
                                "request_id": req_id,
                                "payload": mcp_val,
                                "caller_id": caller_id
                            });
                            let _ = framed.send(resp.to_string()).await;
                        }
                    } else if mcp_val.get("method").is_some() {
                        // It's a Notification or Request from Server (Sampling)
                        // TODO: Implement session-aware reverse routing
                    }
                } else { break; }
            }
        }
    }

    Ok(())
}

// --- CLI Tool: topic echo ---

async fn run_topic_echo(addr: String, topic: String) -> Result<()> {
    let node_id = format!("cli_echo_{}", &Uuid::new_v4().to_string()[..8]);
    println!("Subscribing to '{}' as node '{}'...", topic, node_id);

    let stream = TcpStream::connect(&addr).await
        .context("Failed to connect to Tagentacle Daemon. Is it running?")?;
    let mut framed = Framed::new(stream, LinesCodec::new());

    // Subscribe
    let sub = json!({
        "op": "subscribe",
        "topic": topic,
        "node_id": node_id
    });
    framed.send(sub.to_string()).await?;

    println!("Listening on '{}'... (Ctrl+C to stop)", topic);

    while let Some(Ok(line)) = framed.next().await {
        if let Ok(msg) = serde_json::from_str::<Value>(&line) {
            if msg.get("op").and_then(|v| v.as_str()) == Some("message") {
                let sender = msg.get("sender").and_then(|v| v.as_str()).unwrap_or("?");
                let payload = msg.get("payload").unwrap_or(&Value::Null);
                println!("[{}] {}", sender, serde_json::to_string_pretty(payload)?);
            }
        }
    }

    Ok(())
}

// --- CLI Tool: service call ---

async fn run_service_call(addr: String, service: String, payload_str: String) -> Result<()> {
    let node_id = format!("cli_caller_{}", &Uuid::new_v4().to_string()[..8]);
    let request_id = Uuid::new_v4().to_string();

    let payload: Value = serde_json::from_str(&payload_str)
        .context("Invalid JSON payload. Use single quotes around the JSON string.")?;

    println!("Calling service '{}' as node '{}'...", service, node_id);

    let stream = TcpStream::connect(&addr).await
        .context("Failed to connect to Tagentacle Daemon. Is it running?")?;
    let mut framed = Framed::new(stream, LinesCodec::new());

    // Send service call
    let call = json!({
        "op": "call_service",
        "service": service,
        "request_id": request_id,
        "payload": payload,
        "caller_id": node_id
    });
    framed.send(call.to_string()).await?;

    // Wait for response
    println!("Waiting for response...");
    while let Some(Ok(line)) = framed.next().await {
        if let Ok(msg) = serde_json::from_str::<Value>(&line) {
            if msg.get("op").and_then(|v| v.as_str()) == Some("service_response") {
                if msg.get("request_id").and_then(|v| v.as_str()) == Some(&request_id) {
                    let result = msg.get("payload").unwrap_or(&Value::Null);
                    println!("Response:\n{}", serde_json::to_string_pretty(result)?);
                    return Ok(());
                }
            }
        }
    }

    println!("No response received (connection closed).");
    Ok(())
}

// --- CLI Tool: doctor ---

async fn run_doctor(addr: String) -> Result<()> {
    println!("Tagentacle Doctor");
    println!("=================");
    print!("Checking daemon at {}... ", addr);

    match TcpStream::connect(&addr).await {
        Ok(_) => {
            println!("OK ✓");
            println!("Daemon is running and accepting connections.");
        }
        Err(e) => {
            println!("FAILED ✗");
            println!("Could not connect to daemon: {}", e);
            println!("Try: tagentacle daemon --addr {}", addr);
        }
    }

    Ok(())
}
