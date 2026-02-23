use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
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
    /// Run a single Tagentacle package by its directory
    Run {
        /// Path to the package directory (must contain tagentacle.toml)
        #[arg(long)]
        pkg: String,
        /// Daemon address
        #[arg(short, long, default_value = "127.0.0.1:19999")]
        addr: String,
    },
    /// Launch a system topology from a TOML config file
    Launch {
        /// Path to the launch TOML config file
        config: String,
        /// Daemon address
        #[arg(short, long, default_value = "127.0.0.1:19999")]
        addr: String,
    },
    /// Install dependencies for a package or the entire workspace
    Setup {
        #[command(subcommand)]
        action: SetupAction,
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

#[derive(Subcommand)]
enum SetupAction {
    /// Install dependencies via `uv sync` (requires uv)
    Dep {
        /// Path to a single package directory (must contain pyproject.toml)
        #[arg(long)]
        pkg: Option<String>,
        /// Scan a workspace directory for all packages and install deps
        #[arg(long)]
        all: Option<String>,
    },
    /// Remove install/ workspace structure (reverse of dep)
    Clean {
        /// Workspace root directory
        #[arg(long, default_value = ".")]
        workspace: String,
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
        Some(Commands::Run { pkg, addr }) => {
            run_package(pkg, addr).await?;
        }
        Some(Commands::Launch { config, addr }) => {
            run_launch(config, addr).await?;
        }
        Some(Commands::Setup { action }) => match action {
            SetupAction::Dep { pkg, all } => {
                if let Some(ws) = all {
                    run_setup_all(ws).await?;
                } else {
                    let p = pkg.unwrap_or_else(|| ".".to_string());
                    run_setup_dep(p).await?;
                }
            }
            SetupAction::Clean { workspace } => {
                run_setup_clean(workspace).await?;
            }
        },
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

// --- CLI Tool: run ---

/// Parse a minimal tagentacle.toml to extract package info
fn parse_pkg_toml(path: &Path) -> Result<Value> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Cannot read {}", path.display()))?;
    // Minimal TOML parser: extract key = "value" pairs
    // For a real implementation, use the `toml` crate
    let mut map = serde_json::Map::new();
    let mut section = String::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') || trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            section = trimmed[1..trimmed.len()-1].to_string();
            continue;
        }
        if let Some(eq_pos) = trimmed.find('=') {
            let key = trimmed[..eq_pos].trim();
            let val = trimmed[eq_pos+1..].trim().trim_matches('"');
            let full_key = if section.is_empty() {
                key.to_string()
            } else {
                format!("{}.{}", section, key)
            };
            map.insert(full_key, Value::String(val.to_string()));
        }
    }
    Ok(Value::Object(map))
}

async fn run_package(pkg_path: String, addr: String) -> Result<()> {
    let pkg_dir = PathBuf::from(&pkg_path).canonicalize()
        .with_context(|| format!("Package directory not found: {}", pkg_path))?;
    let toml_path = pkg_dir.join("tagentacle.toml");

    if !toml_path.exists() {
        anyhow::bail!("No tagentacle.toml found in {}", pkg_dir.display());
    }

    let pkg_info = parse_pkg_toml(&toml_path)?;
    let pkg_name = pkg_info.get("package.name")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");
    let entry = pkg_info.get("entry_points.node")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    println!("Running package: {} ({})", pkg_name, pkg_dir.display());

    // Build the python command for the entry point
    let py_command = if !entry.is_empty() {
        let parts: Vec<&str> = entry.split(':').collect();
        if parts.len() == 2 {
            format!("python -c \"from {} import {}; import asyncio; asyncio.run({}())\"",
                parts[0], parts[1], parts[1])
        } else {
            format!("python {}", entry)
        }
    } else {
        // Fallback: look for common entry files
        if pkg_dir.join("main.py").exists() {
            "python main.py".to_string()
        } else if pkg_dir.join("server.py").exists() {
            "python server.py".to_string()
        } else if pkg_dir.join("client.py").exists() {
            "python client.py".to_string()
        } else {
            anyhow::bail!("No entry point found in {} (set entry_points.node in tagentacle.toml)", pkg_dir.display());
        }
    };

    // Check if the package has a .venv (uv project)
    // If so, source it before running — this uses the pkg's own python
    let venv_dir = pkg_dir.join(".venv");
    let shell_command = if venv_dir.join("bin/activate").exists() {
        println!("  Activating venv: {}", venv_dir.display());
        format!("source {}/bin/activate && {}", venv_dir.display(), py_command)
    } else {
        // No local venv — use system python, warn user
        println!("  ⚠ No .venv found. Using system Python.");
        println!("    Hint: cd {} && uv sync", pkg_dir.display());
        // Fall back to python3 if no venv
        py_command.replace("python ", "python3 ")
    };

    println!("  Command: {}", shell_command);

    // Find SDK path for PYTHONPATH
    let sdk_path = find_sdk_path(&pkg_dir);

    let mut child = Command::new("bash")
        .args(["-c", &shell_command])
        .current_dir(&pkg_dir)
        .env("TAGENTACLE_DAEMON_URL", format!("tcp://{}", addr))
        .env("PYTHONPATH", sdk_path.unwrap_or_default())
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .context("Failed to start package process")?;

    let status = child.wait().await?;
    if !status.success() {
        eprintln!("Package exited with status: {}", status);
    }
    Ok(())
}

fn find_sdk_path(from: &Path) -> Option<String> {
    let mut dir = from.to_path_buf();
    for _ in 0..10 {
        let candidate = dir.join("tagentacle-py");
        if candidate.is_dir() {
            return Some(candidate.to_string_lossy().to_string());
        }
        if !dir.pop() { break; }
    }
    None
}

// --- CLI Tool: launch ---

async fn run_launch(config_path: String, daemon_addr: String) -> Result<()> {
    let config_file = PathBuf::from(&config_path).canonicalize()
        .with_context(|| format!("Launch config not found: {}", config_path))?;
    let config_dir = config_file.parent().unwrap();

    println!("Tagentacle Launch");
    println!("=================");
    println!("Config: {}", config_file.display());

    // Parse the TOML launch config (minimal parser)
    let content = std::fs::read_to_string(&config_file)?;
    let config = parse_launch_toml(&content)?;

    // Check daemon connectivity
    let addr = config.daemon_addr.as_deref().unwrap_or(&daemon_addr);
    print!("Checking daemon at {}... ", addr);
    match TcpStream::connect(addr).await {
        Ok(_) => println!("OK ✓"),
        Err(_) => {
            println!("NOT RUNNING");
            println!("Starting daemon...");
            let _daemon = Command::new("sh")
                .args(["-c", &format!("{} daemon --addr {}",
                    std::env::current_exe()?.display(), addr)])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .context("Failed to start daemon")?;
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
    }

    // Find SDK path from config directory
    let sdk_path = find_sdk_path(config_dir).unwrap_or_default();

    // Build env vars from parameters
    let mut env_vars: HashMap<String, String> = HashMap::new();
    env_vars.insert("TAGENTACLE_DAEMON_URL".to_string(), format!("tcp://{}", addr));
    env_vars.insert("PYTHONPATH".to_string(), sdk_path);
    for (k, v) in &config.parameters {
        env_vars.insert(k.clone(), v.clone());
    }

    // Load secrets file if configured
    if let Some(ref secrets_file) = config.secrets_file {
        // Resolve relative to the bringup pkg directory (2 levels up from launch/)
        let bringup_dir = config_dir.parent().unwrap_or(config_dir);
        let secrets_path = bringup_dir.join(secrets_file);
        if secrets_path.exists() {
            env_vars.insert("TAGENTACLE_SECRETS_FILE".to_string(),
                secrets_path.to_string_lossy().to_string());
            println!("Secrets: {} ✓", secrets_path.display());
        } else {
            println!("Secrets: {} (not found, skipped)", secrets_path.display());
        }
    }

    // Launch nodes in order
    let mut processes: Vec<(String, tokio::process::Child)> = Vec::new();
    let nodes_dir = config_dir.parent().unwrap_or(config_dir)
        .parent().unwrap_or(config_dir); // Go up from launch/ to bringup_pkg/ to src/

    for node in &config.nodes {
        // Wait for dependencies
        for dep in &node.depends_on {
            if !processes.iter().any(|(n, _)| n == dep) {
                println!("[{}] Warning: dependency '{}' not found", node.name, dep);
            }
        }

        if node.startup_delay > 0 {
            println!("[{}] Waiting {}s...", node.name, node.startup_delay);
            tokio::time::sleep(std::time::Duration::from_secs(node.startup_delay)).await;
        }

        let pkg_dir = nodes_dir.join(&node.package);
        if !pkg_dir.exists() {
            eprintln!("[{}] Package dir not found: {}", node.name, pkg_dir.display());
            continue;
        }

        println!("[{}] Starting: {} (in {})", node.name, node.command, pkg_dir.display());

        // Source the package's .venv if it exists (per-node isolation)
        let venv_activate = pkg_dir.join(".venv/bin/activate");
        let shell_cmd = if venv_activate.exists() {
            println!("[{}]   venv: {}", node.name, pkg_dir.join(".venv").display());
            format!("source {} && {}", venv_activate.display(), node.command)
        } else {
            node.command.clone()
        };

        let mut cmd = Command::new("bash");
        cmd.args(["-c", &shell_cmd])
            .current_dir(&pkg_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        for (k, v) in &env_vars {
            cmd.env(k, v);
        }

        match cmd.spawn() {
            Ok(child) => {
                processes.push((node.name.clone(), child));
            }
            Err(e) => {
                eprintln!("[{}] Failed to start: {}", node.name, e);
            }
        }
    }

    if processes.is_empty() {
        println!("No nodes launched.");
        return Ok(());
    }

    // Wait for the last process (typically the agent)
    let last_name = processes.last().map(|(n, _)| n.clone()).unwrap_or_default();
    println!("\nWaiting for '{}' to complete...", last_name);

    if let Some((_, proc)) = processes.last_mut() {
        let _ = proc.wait().await;
    }

    // Graceful shutdown
    println!("\n--- Shutting down ---");
    for (name, mut proc) in processes.into_iter().rev() {
        if let Ok(None) = proc.try_wait() {
            let _ = proc.kill().await;
            println!("[{}] terminated.", name);
        }
    }

    println!("Launch complete.");
    Ok(())
}

struct LaunchConfig {
    daemon_addr: Option<String>,
    nodes: Vec<LaunchNode>,
    parameters: HashMap<String, String>,
    secrets_file: Option<String>,
}

struct LaunchNode {
    name: String,
    package: String,
    command: String,
    depends_on: Vec<String>,
    startup_delay: u64,
}

fn parse_launch_toml(content: &str) -> Result<LaunchConfig> {
    let mut config = LaunchConfig {
        daemon_addr: None,
        nodes: Vec::new(),
        parameters: HashMap::new(),
        secrets_file: None,
    };

    let mut current_section = String::new();
    let mut current_node: Option<LaunchNode> = None;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Section headers
        if trimmed == "[[nodes]]" {
            if let Some(node) = current_node.take() {
                config.nodes.push(node);
            }
            current_node = Some(LaunchNode {
                name: String::new(),
                package: String::new(),
                command: String::new(),
                depends_on: Vec::new(),
                startup_delay: 0,
            });
            current_section = "nodes".to_string();
            continue;
        }
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            if let Some(node) = current_node.take() {
                config.nodes.push(node);
            }
            current_section = trimmed[1..trimmed.len()-1].to_string();
            continue;
        }

        // Key-value pairs
        if let Some(eq_pos) = trimmed.find('=') {
            let key = trimmed[..eq_pos].trim();
            let raw_val = trimmed[eq_pos+1..].trim();

            match current_section.as_str() {
                "daemon" => {
                    if key == "addr" {
                        config.daemon_addr = Some(raw_val.trim_matches('"').to_string());
                    }
                }
                "nodes" => {
                    if let Some(ref mut node) = current_node {
                        match key {
                            "name" => node.name = raw_val.trim_matches('"').to_string(),
                            "package" => node.package = raw_val.trim_matches('"').to_string(),
                            "command" => node.command = raw_val.trim_matches('"').to_string(),
                            "startup_delay" => {
                                node.startup_delay = raw_val.parse().unwrap_or(0)
                            }
                            "depends_on" => {
                                // Parse ["dep1", "dep2"]
                                let inner = raw_val.trim_matches(|c| c == '[' || c == ']');
                                node.depends_on = inner.split(',')
                                    .map(|s| s.trim().trim_matches('"').to_string())
                                    .filter(|s| !s.is_empty())
                                    .collect();
                            }
                            _ => {}
                        }
                    }
                }
                "parameters" => {
                    config.parameters.insert(
                        key.to_string(),
                        raw_val.trim_matches('"').to_string(),
                    );
                }
                "secrets" => {
                    if key == "secrets_file" {
                        config.secrets_file = Some(raw_val.trim_matches('"').to_string());
                    }
                }
                _ => {}
            }
        }
    }

    // Don't forget the last node
    if let Some(node) = current_node {
        config.nodes.push(node);
    }

    Ok(config)
}

// --- CLI Tool: setup dep (uv sync) ---

/// Run `uv sync` in a single package directory.
/// The package must have a pyproject.toml (i.e. be a uv project).
async fn run_setup_dep(pkg_path: String) -> Result<()> {
    let pkg_dir = PathBuf::from(&pkg_path).canonicalize()
        .with_context(|| format!("Package directory not found: {}", pkg_path))?;

    let pyproject = pkg_dir.join("pyproject.toml");
    let toml_path = pkg_dir.join("tagentacle.toml");

    let pkg_name = if toml_path.exists() {
        let info = parse_pkg_toml(&toml_path)?;
        info.get("package.name")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string()
    } else {
        pkg_dir.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string())
    };

    if !pyproject.exists() {
        println!("[{}] ⚠ No pyproject.toml found — not a uv project.", pkg_name);
        println!("    Please configure the environment manually with uv.");
        println!("    Hint: cd {} && uv init --vcs none", pkg_dir.display());
        return Ok(());
    }

    println!("[{}] Running uv sync in {}...", pkg_name, pkg_dir.display());

    let mut child = Command::new("uv")
        .arg("sync")
        .current_dir(&pkg_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .context("Failed to run `uv sync`. Is uv installed?")?;

    let status = child.wait().await?;
    if status.success() {
        println!("[{}] ✓ Dependencies synced.", pkg_name);
    } else {
        eprintln!("[{}] ✗ uv sync failed (exit {}).", pkg_name, status);
    }

    Ok(())
}

/// Scan a workspace directory for all tagentacle packages and run `uv sync` in each.
/// Then generate the install/ workspace structure with symlinks.
async fn run_setup_all(workspace_path: String) -> Result<()> {
    let ws = PathBuf::from(&workspace_path).canonicalize()
        .with_context(|| format!("Workspace not found: {}", workspace_path))?;

    println!("Tagentacle Setup — Workspace: {}", ws.display());
    println!("==========================================");

    // Recursively find directories containing tagentacle.toml
    let pkgs = find_all_packages(&ws)?;
    if pkgs.is_empty() {
        println!("No tagentacle packages found in {}", ws.display());
        return Ok(());
    }

    println!("Found {} package(s):\n", pkgs.len());
    for (name, path) in &pkgs {
        println!("  • {} ({})", name, path.display());
    }
    println!();

    // Run uv sync in each package that has pyproject.toml
    for (name, path) in &pkgs {
        let pyproject = path.join("pyproject.toml");
        if pyproject.exists() {
            run_setup_dep(path.to_string_lossy().to_string()).await?;
        } else {
            println!("[{}] ⚠ Skipped — no pyproject.toml (not a uv project).", name);
        }
    }

    // Generate the install/ structure
    println!("\nGenerating install/ workspace structure...");
    generate_install_structure(&ws, &pkgs)?;

    println!("\n✓ Setup complete!");
    println!("  Source the environment: source {}/install/setup_env.bash", ws.display());

    Ok(())
}

/// Find all directories containing tagentacle.toml under a workspace root.
fn find_all_packages(root: &Path) -> Result<Vec<(String, PathBuf)>> {
    let mut pkgs = Vec::new();
    find_packages_recursive(root, &mut pkgs, 0)?;
    Ok(pkgs)
}

fn find_packages_recursive(dir: &Path, pkgs: &mut Vec<(String, PathBuf)>, depth: usize) -> Result<()> {
    if depth > 5 { return Ok(()); } // Limit recursion depth

    let toml_path = dir.join("tagentacle.toml");
    if toml_path.exists() {
        let info = parse_pkg_toml(&toml_path)?;
        let name = info.get("package.name")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        pkgs.push((name, dir.to_path_buf()));
        return Ok(()); // Don't recurse into packages
    }

    // Skip hidden dirs, .venv, __pycache__, target, node_modules, install
    if let Some(dirname) = dir.file_name().and_then(|n| n.to_str()) {
        if dirname.starts_with('.') || dirname == "__pycache__"
            || dirname == "target" || dirname == "node_modules"
            || dirname == "install" {
            return Ok(());
        }
    }

    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                find_packages_recursive(&entry.path(), pkgs, depth + 1)?;
            }
        }
    }

    Ok(())
}

/// Generate the install/ workspace structure:
///   install/src/<pkg_name>/.venv → symlink to actual pkg's .venv
///   install/setup_env.bash → sourceable env script
fn generate_install_structure(ws_root: &Path, pkgs: &[(String, PathBuf)]) -> Result<()> {
    let install_dir = ws_root.join("install");
    let install_src = install_dir.join("src");

    // Create install/src/ directory
    std::fs::create_dir_all(&install_src)
        .context("Failed to create install/src/ directory")?;

    let mut activated_paths: Vec<(String, PathBuf)> = Vec::new();

    for (name, pkg_path) in pkgs {
        let venv_dir = pkg_path.join(".venv");
        let install_pkg_dir = install_src.join(name);

        // Create install/src/<pkg_name>/
        std::fs::create_dir_all(&install_pkg_dir)?;

        // Create or update symlink: install/src/<pkg_name>/.venv → <pkg_path>/.venv
        let symlink_path = install_pkg_dir.join(".venv");
        if symlink_path.exists() || symlink_path.symlink_metadata().is_ok() {
            std::fs::remove_file(&symlink_path).ok();
        }

        if venv_dir.exists() {
            std::os::unix::fs::symlink(&venv_dir, &symlink_path)
                .with_context(|| format!("Failed to create symlink for {}", name))?;
            println!("  {} → {}", symlink_path.display(), venv_dir.display());
            activated_paths.push((name.clone(), venv_dir));
        } else {
            println!("  {} — .venv not found (run uv sync first)", name);
        }
    }

    // Generate setup_env.bash
    let bash_path = install_dir.join("setup_env.bash");
    let mut script = String::new();
    script.push_str("#!/usr/bin/env bash\n");
    script.push_str("# Tagentacle workspace environment setup\n");
    script.push_str("# Auto-generated by `tagentacle setup dep --all`\n");
    script.push_str("# Usage: source install/setup_env.bash\n\n");
    script.push_str("_TAGENTACLE_INSTALL_DIR=\"$(cd \"$(dirname \"${BASH_SOURCE[0]}\")\" && pwd)\"\n\n");

    for (name, venv_path) in &activated_paths {
        script.push_str(&format!(
            "# Package: {}\n\
             if [ -d \"{}/bin\" ]; then\n\
             \texport PATH=\"{}/bin:$PATH\"\n\
             fi\n\n",
            name,
            venv_path.display(),
            venv_path.display(),
        ));
    }

    script.push_str("echo \"Tagentacle environment loaded (");
    script.push_str(&format!("{} packages).\"\n", activated_paths.len()));

    std::fs::write(&bash_path, &script)
        .context("Failed to write setup_env.bash")?;

    // Add install/ to .gitignore if not already present
    let gitignore = ws_root.join(".gitignore");
    let needs_entry = if gitignore.exists() {
        let content = std::fs::read_to_string(&gitignore).unwrap_or_default();
        !content.lines().any(|l| l.trim() == "install/" || l.trim() == "/install/")
    } else {
        true
    };
    if needs_entry {
        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&gitignore)?;
        use std::io::Write;
        writeln!(f, "\n# Tagentacle install workspace\ninstall/")?;
        println!("  Added install/ to .gitignore");
    }

    println!("  Generated {}", bash_path.display());
    Ok(())
}

// --- CLI Tool: setup clean ---

async fn run_setup_clean(workspace_path: String) -> Result<()> {
    let ws = PathBuf::from(&workspace_path).canonicalize()
        .with_context(|| format!("Workspace not found: {}", workspace_path))?;

    let install_dir = ws.join("install");
    if !install_dir.exists() {
        println!("No install/ directory found in {}. Nothing to clean.", ws.display());
        return Ok(());
    }

    println!("Tagentacle Clean — removing install/ structure...");

    // Remove symlinks in install/src/<pkg>/
    let install_src = install_dir.join("src");
    if install_src.exists() {
        if let Ok(entries) = std::fs::read_dir(&install_src) {
            for entry in entries.flatten() {
                let venv_link = entry.path().join(".venv");
                if venv_link.symlink_metadata().is_ok() {
                    std::fs::remove_file(&venv_link)?;
                    println!("  Removed symlink: {}", venv_link.display());
                }
            }
        }
    }

    // Remove the install/ directory
    std::fs::remove_dir_all(&install_dir)
        .context("Failed to remove install/ directory")?;

    println!("✓ install/ directory removed.");
    Ok(())
}
