use anyhow::{Context, Result};
use clap::{CommandFactory, Parser, Subcommand};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use std::time::Instant;
use tokio::net::{TcpListener, TcpStream};
use tokio::process::Command;
use tokio::sync::{Mutex, mpsc};
use tokio_util::codec::{Framed, LinesCodec};
use uuid::Uuid;

#[derive(Parser)]
#[command(name = "tagentacle")]
#[command(about = "Tagentacle: The ROS of AI Agents", long_about = None)]
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
    /// Run tests for a package or the entire workspace (starts daemon automatically)
    Test {
        /// Path to a single package directory containing tests/
        #[arg(long)]
        pkg: Option<String>,
        /// Scan workspace src/ directory and run tests for all packages
        #[arg(long)]
        all: Option<String>,
        /// Daemon address (auto-started if not already running)
        #[arg(short, long, default_value = "127.0.0.1:19999")]
        addr: String,
        /// Extra arguments forwarded to pytest (e.g. "-v --timeout=30 -k pubsub")
        #[arg(last = true)]
        pytest_args: Vec<String>,
    },
    /// Lint a package or the entire workspace (Rust: cargo clippy+fmt, Python: ruff check+format)
    Lint {
        /// Path to a single package directory
        #[arg(long)]
        pkg: Option<String>,
        /// Scan workspace src/ directory and lint all packages
        #[arg(long)]
        all: Option<String>,
        /// Auto-fix lint issues where possible
        #[arg(long)]
        fix: bool,
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
    Register {
        node_id: String,
    },
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
    Pong {
        node_id: String,
    },
}

// --- Daemon Logic ---

/// Metadata for a connected node.
struct NodeEntry {
    tx: mpsc::UnboundedSender<Value>,
    connected_at: String, // ISO 8601 timestamp
    registered: bool,     // true if node sent a Register message
    last_pong: Instant,   // last heartbeat response
}

struct Router {
    // Topic subscriptions: topic -> Vec<(node_id, tx)>
    subscriptions: HashMap<String, Vec<(String, mpsc::UnboundedSender<Value>)>>,
    // Service registry: service_name -> (node_id, tx)
    services: HashMap<String, (String, mpsc::UnboundedSender<Value>)>,
    // Node registry: node_id -> NodeEntry (extended metadata)
    nodes: HashMap<String, NodeEntry>,
    // Daemon start time for uptime calculation
    started_at: Instant,
}

impl Router {
    fn new() -> Self {
        Self {
            subscriptions: HashMap::new(),
            services: HashMap::new(),
            nodes: HashMap::new(),
            started_at: Instant::now(),
        }
    }

    /// Insert or update a node entry (preserving connected_at if already exists).
    fn upsert_node(&mut self, node_id: &str, tx: mpsc::UnboundedSender<Value>) {
        self.nodes
            .entry(node_id.to_string())
            .and_modify(|e| {
                e.tx = tx.clone();
                e.last_pong = Instant::now();
            })
            .or_insert_with(|| NodeEntry {
                tx,
                connected_at: chrono_now(),
                registered: false,
                last_pong: Instant::now(),
            });
    }

    /// Remove a node and clean up its subscriptions and services.
    fn remove_node(&mut self, node_id: &str) {
        self.nodes.remove(node_id);
        // Clean subscriptions
        for subs in self.subscriptions.values_mut() {
            subs.retain(|(id, _)| id != node_id);
        }
        // Remove empty topics
        self.subscriptions.retain(|_, subs| !subs.is_empty());
        // Clean services
        self.services.retain(|_, (id, _)| id != node_id);
    }

    /// Publish a message to a topic (used for internal node_events publishing).
    fn publish_internal(&self, topic: &str, sender: &str, payload: Value) {
        if let Some(subs) = self.subscriptions.get(topic) {
            let push =
                json!({"op": "message", "topic": topic, "sender": sender, "payload": payload});
            for (_, sub_tx) in subs {
                let _ = sub_tx.send(push.clone());
            }
        }
    }

    // --- Built-in system service handlers (/tagentacle/* interception) ---
    // These are the Daemon's equivalent of Linux /proc — read-only introspection
    // of the Router's internal state. They are NOT published via advertise_service;
    // the Daemon intercepts call_service requests to /tagentacle/* directly.
    // This is an intentional asymmetry: system services are provided by the
    // Daemon itself, not by any Node.

    /// Handle a /tagentacle/* system service call.
    /// Returns Some(response_payload) if handled, None if not a system service.
    fn handle_system_service(&self, service: &str, payload: &Value) -> Option<Value> {
        match service {
            "/tagentacle/ping" => {
                let uptime = self.started_at.elapsed().as_secs();
                Some(json!({
                    "status": "ok",
                    "uptime_s": uptime,
                    "version": env!("CARGO_PKG_VERSION"),
                    "node_count": self.nodes.len(),
                    "topic_count": self.subscriptions.len(),
                    "service_count": self.services.len()
                }))
            }
            "/tagentacle/list_nodes" => {
                let nodes: Vec<Value> = self
                    .nodes
                    .iter()
                    .map(|(id, entry)| {
                        json!({
                            "node_id": id,
                            "connected_at": entry.connected_at,
                            "registered": entry.registered
                        })
                    })
                    .collect();
                Some(json!({ "nodes": nodes }))
            }
            "/tagentacle/list_topics" => {
                let topics: Vec<Value> = self
                    .subscriptions
                    .iter()
                    .map(|(name, subs)| {
                        let subscriber_ids: Vec<&str> =
                            subs.iter().map(|(id, _)| id.as_str()).collect();
                        json!({
                            "name": name,
                            "subscriber_count": subs.len(),
                            "subscribers": subscriber_ids
                        })
                    })
                    .collect();
                Some(json!({ "topics": topics }))
            }
            "/tagentacle/list_services" => {
                let services: Vec<Value> = self
                    .services
                    .iter()
                    .map(|(name, (provider, _))| {
                        json!({
                            "name": name,
                            "provider": provider
                        })
                    })
                    .collect();
                Some(json!({ "services": services }))
            }
            "/tagentacle/get_node_info" => {
                let target = payload
                    .get("node_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                if let Some(entry) = self.nodes.get(target) {
                    let subs: Vec<&str> = self
                        .subscriptions
                        .iter()
                        .filter(|(_, s)| s.iter().any(|(id, _)| id == target))
                        .map(|(topic, _)| topic.as_str())
                        .collect();
                    let svcs: Vec<&str> = self
                        .services
                        .iter()
                        .filter(|(_, (id, _))| id == target)
                        .map(|(name, _)| name.as_str())
                        .collect();
                    Some(json!({
                        "node_id": target,
                        "connected_at": entry.connected_at,
                        "registered": entry.registered,
                        "subscriptions": subs,
                        "services": svcs
                    }))
                } else {
                    Some(json!({ "error": format!("Node '{}' not found", target) }))
                }
            }
            _ => None,
        }
    }
}

/// Simple ISO 8601 timestamp (no external chrono dependency).
fn chrono_now() -> String {
    // Use std::time for a rough UTC timestamp
    let dur = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = dur.as_secs();
    // Basic formatting: seconds since epoch (nodes can interpret)
    format!("{}", secs)
}

type SharedRouter = Arc<Mutex<Router>>;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Daemon { addr }) => {
            run_daemon(addr).await?;
        }
        Some(Commands::Topic { action }) => match action {
            TopicAction::Echo { topic, addr } => {
                run_topic_echo(addr, topic).await?;
            }
        },
        Some(Commands::Service { action }) => match action {
            ServiceAction::Call {
                service,
                payload,
                addr,
            } => {
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
        Some(Commands::Test {
            pkg,
            all,
            addr,
            pytest_args,
        }) => {
            run_test(pkg, all, addr, pytest_args).await?;
        }
        Some(Commands::Lint { pkg, all, fix }) => {
            run_lint(pkg, all, fix).await?;
        }
        None => {
            Cli::command().print_help()?;
            println!();
        }
    }

    Ok(())
}

async fn run_daemon(addr: String) -> Result<()> {
    let listener = TcpListener::bind(&addr).await?;
    println!("Tagentacle Daemon listening on: {}", addr);

    let router = Arc::new(Mutex::new(Router::new()));

    // Spawn heartbeat task: send ping to all nodes every 30s, clean stale ones
    let heartbeat_router = Arc::clone(&router);
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
        loop {
            interval.tick().await;
            let mut r = heartbeat_router.lock().await;
            let ping_msg = json!({"op": "ping"});

            // Collect stale nodes (no pong for > 90s)
            let stale: Vec<String> = r
                .nodes
                .iter()
                .filter(|(_, entry)| entry.last_pong.elapsed().as_secs() > 90)
                .map(|(id, _)| id.clone())
                .collect();

            // Remove stale nodes and publish events
            for node_id in &stale {
                println!("Node '{}' timed out (no heartbeat), removing.", node_id);
                r.remove_node(node_id);
                r.publish_internal(
                    "/tagentacle/node_events",
                    "_daemon_",
                    json!({"event": "disconnected", "node_id": node_id, "reason": "heartbeat_timeout"}),
                );
            }

            // Send ping to remaining nodes
            for (_, entry) in r.nodes.iter() {
                let _ = entry.tx.send(ping_msg.clone());
            }
        }
    });

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
                            Action::Register { node_id } => {
                                current_node_id = Some(node_id.clone());
                                let mut r = router.lock().await;
                                r.upsert_node(&node_id, tx.clone());
                                if let Some(entry) = r.nodes.get_mut(&node_id) {
                                    entry.registered = true;
                                }
                                println!("Node '{}' registered.", node_id);
                                // Publish node_events
                                r.publish_internal(
                                    "/tagentacle/node_events",
                                    "_daemon_",
                                    json!({"event": "connected", "node_id": node_id}),
                                );
                                // Send ack back to node
                                let _ = tx.send(json!({"op": "register_ack", "node_id": node_id, "status": "ok"}));
                            }
                            Action::Subscribe { topic, node_id } => {
                                current_node_id = Some(node_id.clone());
                                let mut r = router.lock().await;
                                r.upsert_node(&node_id, tx.clone());
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
                                r.upsert_node(&node_id, tx.clone());
                                r.services.insert(service, (node_id, tx.clone()));
                            }
                            Action::CallService { service, request_id, payload, caller_id } => {
                                let mut r = router.lock().await;
                                r.upsert_node(&caller_id, tx.clone());

                                // Intercept /tagentacle/* system services (Daemon's /proc equivalent)
                                if service.starts_with("/tagentacle/") {
                                    if let Some(response_payload) = r.handle_system_service(&service, &payload) {
                                        let _ = tx.send(json!({
                                            "op": "service_response",
                                            "service": service,
                                            "request_id": request_id,
                                            "payload": response_payload,
                                            "caller_id": caller_id
                                        }));
                                    }
                                } else if let Some((_, srv_tx)) = r.services.get(&service) {
                                    let _ = srv_tx.send(json!({"op": "call_service", "service": service, "request_id": request_id, "payload": payload, "caller_id": caller_id}));
                                }
                            }
                            Action::ServiceResponse { service, request_id, payload, caller_id } => {
                                let r = router.lock().await;
                                if let Some(entry) = r.nodes.get(&caller_id) {
                                    let _ = entry.tx.send(json!({"op": "service_response", "service": service, "request_id": request_id, "payload": payload, "caller_id": caller_id}));
                                }
                            }
                            Action::Pong { node_id } => {
                                let mut r = router.lock().await;
                                if let Some(entry) = r.nodes.get_mut(&node_id) {
                                    entry.last_pong = Instant::now();
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
    // Clean up on disconnect
    if let Some(ref node_id) = current_node_id {
        println!("Node '{}' disconnected", node_id);
        let mut r = router.lock().await;
        r.remove_node(node_id);
        r.publish_internal(
            "/tagentacle/node_events",
            "_daemon_",
            json!({"event": "disconnected", "node_id": node_id, "reason": "connection_closed"}),
        );
    }
    Ok(())
}

// --- CLI Tool: topic echo ---

async fn run_topic_echo(addr: String, topic: String) -> Result<()> {
    let node_id = format!("cli_echo_{}", &Uuid::new_v4().to_string()[..8]);
    println!("Subscribing to '{}' as node '{}'...", topic, node_id);

    let stream = TcpStream::connect(&addr)
        .await
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
        if let Ok(msg) = serde_json::from_str::<Value>(&line)
            && msg.get("op").and_then(|v| v.as_str()) == Some("message")
        {
            let sender = msg.get("sender").and_then(|v| v.as_str()).unwrap_or("?");
            let payload = msg.get("payload").unwrap_or(&Value::Null);
            println!("[{}] {}", sender, serde_json::to_string_pretty(payload)?);
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

    let stream = TcpStream::connect(&addr)
        .await
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
        if let Ok(msg) = serde_json::from_str::<Value>(&line)
            && msg.get("op").and_then(|v| v.as_str()) == Some("service_response")
            && msg.get("request_id").and_then(|v| v.as_str()) == Some(&request_id)
        {
            let result = msg.get("payload").unwrap_or(&Value::Null);
            println!("Response:\n{}", serde_json::to_string_pretty(result)?);
            return Ok(());
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
    let content =
        std::fs::read_to_string(path).with_context(|| format!("Cannot read {}", path.display()))?;
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
            section = trimmed[1..trimmed.len() - 1].to_string();
            continue;
        }
        if let Some(eq_pos) = trimmed.find('=') {
            let key = trimmed[..eq_pos].trim();
            let val = trimmed[eq_pos + 1..].trim().trim_matches('"');
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
    let pkg_dir = PathBuf::from(&pkg_path)
        .canonicalize()
        .with_context(|| format!("Package directory not found: {}", pkg_path))?;
    let toml_path = pkg_dir.join("tagentacle.toml");

    if !toml_path.exists() {
        anyhow::bail!("No tagentacle.toml found in {}", pkg_dir.display());
    }

    let pkg_info = parse_pkg_toml(&toml_path)?;
    let pkg_name = pkg_info
        .get("package.name")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");
    let entry = pkg_info
        .get("entry_points.node")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    println!("Running package: {} ({})", pkg_name, pkg_dir.display());

    // Build the python command for the entry point
    let py_command = if !entry.is_empty() {
        let parts: Vec<&str> = entry.split(':').collect();
        if parts.len() == 2 {
            format!(
                "python -c \"from {} import {}; import asyncio; asyncio.run({}())\"",
                parts[0], parts[1], parts[1]
            )
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
            anyhow::bail!(
                "No entry point found in {} (set entry_points.node in tagentacle.toml)",
                pkg_dir.display()
            );
        }
    };

    // Check if the package has a .venv (uv project)
    // If so, source it before running — this uses the pkg's own python
    let venv_dir = pkg_dir.join(".venv");
    let shell_command = if venv_dir.join("bin/activate").exists() {
        println!("  Activating venv: {}", venv_dir.display());
        format!(
            "source {}/bin/activate && {}",
            venv_dir.display(),
            py_command
        )
    } else {
        // No local venv — use system python, warn user
        println!("  ⚠ No .venv found. Using system Python.");
        println!("    Hint: cd {} && uv sync", pkg_dir.display());
        // Fall back to python3 if no venv
        py_command.replace("python ", "python3 ")
    };

    println!("  Command: {}", shell_command);

    let mut child = Command::new("bash")
        .args(["-c", &shell_command])
        .current_dir(&pkg_dir)
        .env("TAGENTACLE_DAEMON_URL", format!("tcp://{}", addr))
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

// --- CLI Tool: launch ---

async fn run_launch(config_path: String, daemon_addr: String) -> Result<()> {
    let config_file = PathBuf::from(&config_path)
        .canonicalize()
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
                .args([
                    "-c",
                    &format!(
                        "{} daemon --addr {}",
                        std::env::current_exe()?.display(),
                        addr
                    ),
                ])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .context("Failed to start daemon")?;
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
    }

    // Build env vars from parameters
    let mut env_vars: HashMap<String, String> = HashMap::new();
    env_vars.insert(
        "TAGENTACLE_DAEMON_URL".to_string(),
        format!("tcp://{}", addr),
    );
    for (k, v) in &config.parameters {
        env_vars.insert(k.clone(), v.clone());
    }

    // Load secrets file if configured
    if let Some(ref secrets_file) = config.secrets_file {
        // Resolve relative to the bringup pkg directory (2 levels up from launch/)
        let bringup_dir = config_dir.parent().unwrap_or(config_dir);
        let secrets_path = bringup_dir.join(secrets_file);
        if secrets_path.exists() {
            env_vars.insert(
                "TAGENTACLE_SECRETS_FILE".to_string(),
                secrets_path.to_string_lossy().to_string(),
            );
            println!("Secrets: {} ✓", secrets_path.display());
        } else {
            println!("Secrets: {} (not found, skipped)", secrets_path.display());
        }
    }

    // Launch nodes in order
    let mut processes: Vec<(String, tokio::process::Child)> = Vec::new();
    let nodes_dir = config_dir
        .parent()
        .unwrap_or(config_dir)
        .parent()
        .unwrap_or(config_dir); // Go up from launch/ to bringup_pkg/ to src/

    // Build a lookup table: tagentacle.toml package name → directory path
    let known_pkgs = find_all_packages(nodes_dir).unwrap_or_default();

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

        // Resolve package directory: first by tagentacle.toml name, then by dir name,
        // then try kebab-case conversion (snake_case → kebab-case).
        let pkg_dir = known_pkgs
            .iter()
            .find(|(name, _)| name == &node.package)
            .map(|(_, p)| p.clone())
            .unwrap_or_else(|| {
                let direct = nodes_dir.join(&node.package);
                if direct.exists() {
                    direct
                } else {
                    // Try kebab-case: example_mcp_server → example-mcp-server
                    nodes_dir.join(node.package.replace('_', "-"))
                }
            });
        if !pkg_dir.exists() {
            eprintln!(
                "[{}] Package dir not found: {} (looked for '{}' in {})",
                node.name,
                pkg_dir.display(),
                node.package,
                nodes_dir.display()
            );
            continue;
        }

        println!(
            "[{}] Starting: {} (in {})",
            node.name,
            node.command,
            pkg_dir.display()
        );

        // Source the package's .venv if it exists (per-node isolation)
        let venv_activate = pkg_dir.join(".venv/bin/activate");
        let shell_cmd = if venv_activate.exists() {
            println!(
                "[{}]   venv: {}",
                node.name,
                pkg_dir.join(".venv").display()
            );
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

/// Raw TOML structures for serde deserialization
#[derive(Deserialize)]
struct LaunchToml {
    daemon: Option<DaemonToml>,
    nodes: Option<Vec<LaunchNodeToml>>,
    parameters: Option<HashMap<String, String>>,
    secrets: Option<SecretsToml>,
}

#[derive(Deserialize)]
struct DaemonToml {
    addr: Option<String>,
}

#[derive(Deserialize)]
struct LaunchNodeToml {
    name: String,
    package: String,
    command: String,
    #[serde(default)]
    depends_on: Vec<String>,
    #[serde(default)]
    startup_delay: u64,
    #[allow(dead_code)]
    #[serde(default)]
    description: Option<String>,
}

#[derive(Deserialize)]
struct SecretsToml {
    secrets_file: Option<String>,
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
    let raw: LaunchToml = toml::from_str(content).with_context(|| "Failed to parse launch TOML")?;

    let nodes = raw
        .nodes
        .unwrap_or_default()
        .into_iter()
        .map(|n| LaunchNode {
            name: n.name,
            package: n.package,
            command: n.command,
            depends_on: n.depends_on,
            startup_delay: n.startup_delay,
        })
        .collect();

    Ok(LaunchConfig {
        daemon_addr: raw.daemon.and_then(|d| d.addr),
        nodes,
        parameters: raw.parameters.unwrap_or_default(),
        secrets_file: raw.secrets.and_then(|s| s.secrets_file),
    })
}

// --- CLI Tool: setup dep (uv sync) ---

/// Run `uv sync` in a single package directory.
/// The package must have a pyproject.toml (i.e. be a uv project).
async fn run_setup_dep(pkg_path: String) -> Result<()> {
    let pkg_dir = PathBuf::from(&pkg_path)
        .canonicalize()
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
        pkg_dir
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string())
    };

    if !pyproject.exists() {
        println!(
            "[{}] ⚠ No pyproject.toml found — not a uv project.",
            pkg_name
        );
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

/// Scan workspace for [workspace.repos] sections in any tagentacle.toml and
/// auto-clone missing repositories into the workspace root.
async fn clone_workspace_repos(ws: &Path) -> Result<()> {
    // Collect all (repo_name, git_url) pairs from every tagentacle.toml
    let mut repos_to_clone: Vec<(String, String)> = Vec::new();

    let existing_pkgs = find_all_packages(ws)?;
    for (_name, pkg_path) in &existing_pkgs {
        let toml_path = pkg_path.join("tagentacle.toml");
        if !toml_path.exists() {
            continue;
        }

        let content = std::fs::read_to_string(&toml_path)
            .with_context(|| format!("Cannot read {}", toml_path.display()))?;
        let doc: toml::Value = content
            .parse::<toml::Value>()
            .with_context(|| format!("Invalid TOML in {}", toml_path.display()))?;

        // Navigate: workspace -> repos -> { repo_name: { git: "url" } }
        if let Some(workspace) = doc.get("workspace").and_then(|v| v.as_table())
            && let Some(repos) = workspace.get("repos").and_then(|v| v.as_table())
        {
            for (repo_name, repo_val) in repos {
                if let Some(git_url) = repo_val.get("git").and_then(|v| v.as_str()) {
                    repos_to_clone.push((repo_name.clone(), git_url.to_string()));
                }
            }
        }
    }

    if repos_to_clone.is_empty() {
        return Ok(());
    }

    println!("\n--- Workspace Dependency Repos ---");
    let mut cloned = 0usize;
    let mut skipped = 0usize;

    for (repo_name, git_url) in &repos_to_clone {
        // Derive expected directory name from git URL (last segment without .git)
        let dir_name = git_url
            .rsplit('/')
            .next()
            .unwrap_or(repo_name)
            .trim_end_matches(".git");
        let target_dir = ws.join(dir_name);

        if target_dir.exists() {
            println!("  {} — already exists, skipping.", dir_name);
            skipped += 1;
            continue;
        }

        println!("  Cloning {} → {} ...", git_url, target_dir.display());
        let status = Command::new("git")
            .args(["clone", git_url, &target_dir.to_string_lossy()])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .await
            .context("Failed to run `git clone`. Is git installed?")?;

        if status.success() {
            println!("  ✓ Cloned {}.", dir_name);
            cloned += 1;
        } else {
            eprintln!(
                "  ✗ Failed to clone {} (exit {}). Continuing...",
                dir_name, status
            );
        }
    }

    println!("--- Repos: {} cloned, {} skipped ---\n", cloned, skipped);
    Ok(())
}

/// Scan a workspace src directory for all tagentacle packages and run `uv sync` in each.
/// Then generate the install/ workspace structure with symlinks.
///
/// Directory layout (ROS-style):
///   ws/src/pkg1/.venv          ← actual venvs (created by uv sync)
///   ws/src/pkg2/.venv
///   ws/install/pkg1/.venv      ← symlinks into src
///   ws/install/pkg2/.venv
///   ws/install/setup_env.bash  ← sourceable env script
///
/// The `workspace_path` argument points to the `src/` directory.
async fn run_setup_all(workspace_path: String) -> Result<()> {
    let src_dir = PathBuf::from(&workspace_path)
        .canonicalize()
        .with_context(|| format!("Workspace src dir not found: {}", workspace_path))?;

    // Workspace root is the parent of src/
    let ws_root = src_dir
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| src_dir.clone());

    println!("Tagentacle Setup — Workspace: {}", ws_root.display());
    println!("  src: {}", src_dir.display());
    println!("==========================================");

    // Phase 0: Auto-clone missing repos declared in [workspace.repos]
    clone_workspace_repos(&src_dir).await?;

    // Phase 1: Recursively find directories containing tagentacle.toml
    // (re-scan after cloning to pick up newly cloned packages)
    let pkgs = find_all_packages(&src_dir)?;
    if pkgs.is_empty() {
        println!("No tagentacle packages found in {}", src_dir.display());
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
            println!(
                "[{}] ⚠ Skipped — no pyproject.toml (not a uv project).",
                name
            );
        }
    }

    // Generate the install/ structure at ws_root (sibling of src/)
    println!("\nGenerating install/ workspace structure...");
    generate_install_structure(&ws_root, &pkgs)?;

    println!("\n✓ Setup complete!");
    println!(
        "  Source the environment: source {}/install/setup_env.bash",
        ws_root.display()
    );

    Ok(())
}

/// Find all directories containing tagentacle.toml under a workspace root.
fn find_all_packages(root: &Path) -> Result<Vec<(String, PathBuf)>> {
    let mut pkgs = Vec::new();
    find_packages_recursive(root, &mut pkgs, 0)?;
    Ok(pkgs)
}

fn find_packages_recursive(
    dir: &Path,
    pkgs: &mut Vec<(String, PathBuf)>,
    depth: usize,
) -> Result<()> {
    if depth > 5 {
        return Ok(());
    } // Limit recursion depth

    let toml_path = dir.join("tagentacle.toml");
    if toml_path.exists() {
        let info = parse_pkg_toml(&toml_path)?;
        let name = info
            .get("package.name")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        pkgs.push((name, dir.to_path_buf()));
        return Ok(()); // Don't recurse into packages
    }

    // Skip hidden dirs, .venv, __pycache__, target, node_modules, install
    if let Some(dirname) = dir.file_name().and_then(|n| n.to_str())
        && (dirname.starts_with('.')
            || dirname == "__pycache__"
            || dirname == "target"
            || dirname == "node_modules"
            || dirname == "install")
    {
        return Ok(());
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
///   install/<pkg_name>/.venv → symlink to actual pkg's .venv
///   install/setup_env.bash   → sourceable env script
fn generate_install_structure(ws_root: &Path, pkgs: &[(String, PathBuf)]) -> Result<()> {
    let install_dir = ws_root.join("install");

    // Create install/ directory
    std::fs::create_dir_all(&install_dir).context("Failed to create install/ directory")?;

    let mut activated_paths: Vec<(String, PathBuf)> = Vec::new();

    for (name, pkg_path) in pkgs {
        let venv_dir = pkg_path.join(".venv");
        let install_pkg_dir = install_dir.join(name);

        // Create install/<pkg_name>/
        std::fs::create_dir_all(&install_pkg_dir)?;

        // Create or update symlink: install/<pkg_name>/.venv → <pkg_path>/.venv
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
    script.push_str(
        "_TAGENTACLE_INSTALL_DIR=\"$(cd \"$(dirname \"${BASH_SOURCE[0]}\")\" && pwd)\"\n\n",
    );

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

    std::fs::write(&bash_path, &script).context("Failed to write setup_env.bash")?;

    // Add install/ to .gitignore if not already present
    let gitignore = ws_root.join(".gitignore");
    let needs_entry = if gitignore.exists() {
        let content = std::fs::read_to_string(&gitignore).unwrap_or_default();
        !content
            .lines()
            .any(|l| l.trim() == "install/" || l.trim() == "/install/")
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
    let ws = PathBuf::from(&workspace_path)
        .canonicalize()
        .with_context(|| format!("Workspace not found: {}", workspace_path))?;

    let install_dir = ws.join("install");
    if !install_dir.exists() {
        println!(
            "No install/ directory found in {}. Nothing to clean.",
            ws.display()
        );
        return Ok(());
    }

    println!("Tagentacle Clean — removing install/ structure...");

    // Remove symlinks in install/<pkg>/
    if let Ok(entries) = std::fs::read_dir(&install_dir) {
        for entry in entries.flatten() {
            let venv_link = entry.path().join(".venv");
            if venv_link.symlink_metadata().is_ok() {
                std::fs::remove_file(&venv_link)?;
                println!("  Removed symlink: {}", venv_link.display());
            }
        }
    }

    // Remove the install/ directory
    std::fs::remove_dir_all(&install_dir).context("Failed to remove install/ directory")?;

    println!("✓ install/ directory removed.");
    Ok(())
}

// --- CLI Tool: test ---

/// Read [tool.tagentacle.test] from a pyproject.toml, returning (requires, extras).
fn read_test_config(pkg_dir: &Path) -> Result<(Vec<String>, Vec<String>)> {
    let pyproject_path = pkg_dir.join("pyproject.toml");
    if !pyproject_path.exists() {
        return Ok((vec![], vec![]));
    }
    let content = std::fs::read_to_string(&pyproject_path)
        .with_context(|| format!("Cannot read {}", pyproject_path.display()))?;
    let doc: toml::Value = content
        .parse::<toml::Value>()
        .with_context(|| format!("Invalid TOML in {}", pyproject_path.display()))?;

    let test_section = doc
        .get("tool")
        .and_then(|t| t.get("tagentacle"))
        .and_then(|t| t.get("test"));

    let requires = test_section
        .and_then(|s| s.get("requires"))
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    let extras = test_section
        .and_then(|s| s.get("extras"))
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    Ok((requires, extras))
}

/// Ensure test dependencies are synced for a package (reads [tool.tagentacle.test]).
async fn sync_test_deps(pkg_dir: &Path, pkg_name: &str) -> Result<()> {
    let (requires, _extras) = read_test_config(pkg_dir)?;

    if !requires.is_empty() {
        println!("[{}] Test requires: {}", pkg_name, requires.join(", "));
        // Find workspace root: walk up from pkg_dir to find sibling packages
        let ws_root = pkg_dir.parent().unwrap_or(pkg_dir);

        for dep_name in &requires {
            let dep_dir = ws_root.join(dep_name);
            if !dep_dir.exists() {
                println!(
                    "[{}] ⚠ Required dependency '{}' not found at {}",
                    pkg_name,
                    dep_name,
                    dep_dir.display()
                );
                continue;
            }
            if !dep_dir.join(".venv").exists() {
                println!("[{}] Syncing dependency '{}'...", pkg_name, dep_name);
                let status = Command::new("uv")
                    .arg("sync")
                    .current_dir(&dep_dir)
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .status()
                    .await
                    .with_context(|| format!("Failed to sync {}", dep_name))?;
                if !status.success() {
                    println!("[{}] ⚠ Failed to sync dependency '{}'", pkg_name, dep_name);
                }
            }
        }
    }

    // Sync the test package itself
    let pyproject = pkg_dir.join("pyproject.toml");
    if pyproject.exists() && !pkg_dir.join(".venv").exists() {
        println!("[{}] Syncing package deps...", pkg_name);
        let status = Command::new("uv")
            .arg("sync")
            .current_dir(pkg_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .await
            .context("Failed to run uv sync")?;
        if !status.success() {
            println!("[{}] ⚠ uv sync failed", pkg_name);
        }
    }

    Ok(())
}

/// Run pytest for one or more packages, auto-starting daemon as needed.
async fn run_test(
    pkg: Option<String>,
    all: Option<String>,
    addr: String,
    pytest_args: Vec<String>,
) -> Result<()> {
    println!("Tagentacle Test");
    println!("===============");

    // 1. Check if daemon is already running; if not, start one.
    let daemon_child = ensure_daemon(&addr).await?;

    let daemon_url = format!("tcp://{}", addr);

    // 2. Collect packages to test
    let packages: Vec<PathBuf> = if let Some(ws_src) = all {
        let src_dir = PathBuf::from(&ws_src)
            .canonicalize()
            .with_context(|| format!("src directory not found: {}", ws_src))?;
        find_testable_packages(&src_dir)?
    } else if let Some(p) = pkg {
        let d = PathBuf::from(&p)
            .canonicalize()
            .with_context(|| format!("Package directory not found: {}", p))?;
        vec![d]
    } else {
        // Default: current directory
        vec![std::env::current_dir()?]
    };

    if packages.is_empty() {
        println!("No testable packages found.");
        // Clean up daemon if we started it
        cleanup_daemon(daemon_child).await;
        return Ok(());
    }

    println!("Packages to test: {}", packages.len());
    for p in &packages {
        println!("  - {}", p.display());
    }
    println!();

    // 3. Run pytest for each package
    let mut total_pass = 0u32;
    let mut total_fail = 0u32;

    for pkg_dir in &packages {
        let pkg_name = pkg_dir
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let tests_dir = pkg_dir.join("tests");

        if !tests_dir.exists() {
            println!("[{}] No tests/ directory, skipping.", pkg_name);
            continue;
        }

        // Package health checks
        if !pkg_dir.join("pyproject.toml").exists() {
            println!(
                "[{}] ⚠ Warning: no pyproject.toml — not a standard Tagentacle package",
                pkg_name
            );
        }
        if !pkg_dir.join("uv.lock").exists() {
            println!(
                "[{}] ⚠ Warning: no uv.lock — run 'uv sync' or 'tagentacle setup dep' \
                 to generate a lockfile (Tagentacle convention: uv.lock is the \
                 reproducible deploy standard, pyproject.toml alone is not sufficient)",
                pkg_name
            );
        }

        // Sync test dependencies declared in [tool.tagentacle.test]
        sync_test_deps(pkg_dir, &pkg_name).await?;

        println!("[{}] Running tests...", pkg_name);

        // Source the package .venv if available
        let venv_activate = pkg_dir.join(".venv/bin/activate");
        let mut pytest_cmd_parts: Vec<String> = Vec::new();

        if venv_activate.exists() {
            pytest_cmd_parts.push(format!("source {}", venv_activate.display()));
            pytest_cmd_parts.push("&&".to_string());
        }

        pytest_cmd_parts.push("python -m pytest tests/".to_string());
        pytest_cmd_parts.push("-v".to_string());

        // Append user-provided pytest args (shell-quote args containing spaces)
        for arg in &pytest_args {
            if arg.contains(' ') || arg.contains('\'') || arg.contains('"') {
                // Wrap in single quotes, escaping any embedded single quotes
                let escaped = arg.replace('\'', "'\\''");
                pytest_cmd_parts.push(format!("'{}'", escaped));
            } else {
                pytest_cmd_parts.push(arg.clone());
            }
        }

        let shell_cmd = pytest_cmd_parts.join(" ");

        let status = Command::new("bash")
            .args(["-c", &shell_cmd])
            .current_dir(pkg_dir)
            .env("TAGENTACLE_DAEMON_URL", &daemon_url)
            .env(
                "TAGENTACLE_BIN",
                std::env::current_exe()?.to_string_lossy().to_string(),
            )
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .await
            .with_context(|| format!("[{}] Failed to run pytest", pkg_name))?;

        if status.success() {
            total_pass += 1;
            println!("[{}] ✓ PASSED\n", pkg_name);
        } else {
            total_fail += 1;
            println!("[{}] ✗ FAILED (exit {})\n", pkg_name, status);
        }
    }

    // 4. Summary
    println!("==========================");
    println!("Test Summary: {} passed, {} failed", total_pass, total_fail);
    println!("==========================");

    // 5. Cleanup daemon if we started it
    cleanup_daemon(daemon_child).await;

    if total_fail > 0 {
        std::process::exit(1);
    }
    Ok(())
}

/// Check if daemon is running at `addr`; if not, start one in background.
/// Returns `Some(Child)` if we started it (caller must clean up), or `None`.
async fn ensure_daemon(addr: &str) -> Result<Option<tokio::process::Child>> {
    use tokio::net::TcpStream;

    print!("Checking daemon at {}... ", addr);
    match TcpStream::connect(addr).await {
        Ok(_) => {
            println!("RUNNING ✓ (using existing daemon)");
            Ok(None)
        }
        Err(_) => {
            println!("NOT RUNNING");
            println!("Starting daemon...");
            let exe = std::env::current_exe()?;
            let child = Command::new(exe)
                .args(["daemon", "--addr", addr])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .context("Failed to start daemon")?;

            // Wait for it to be ready
            let deadline = tokio::time::Instant::now() + tokio::time::Duration::from_secs(10);
            loop {
                if tokio::time::Instant::now() > deadline {
                    anyhow::bail!("Daemon failed to start within 10s");
                }
                if TcpStream::connect(addr).await.is_ok() {
                    break;
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
            println!("Daemon started ✓");
            Ok(Some(child))
        }
    }
}

/// If we spawned a daemon child, kill it gracefully.
async fn cleanup_daemon(child: Option<tokio::process::Child>) {
    if let Some(mut c) = child {
        println!("Stopping daemon...");
        let _ = c.kill().await;
        let _ = c.wait().await;
        println!("Daemon stopped.");
    }
}

/// Find all packages under `src_dir` that contain a `tests/` directory.
fn find_testable_packages(src_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut packages = Vec::new();
    if !src_dir.is_dir() {
        anyhow::bail!("{} is not a directory", src_dir.display());
    }
    for entry in std::fs::read_dir(src_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && path.join("tests").is_dir() {
            packages.push(path);
        }
    }
    packages.sort();
    Ok(packages)
}

// --- CLI Tool: lint ---

/// Detect package type from directory contents.
enum PkgType {
    Rust,
    Python,
    Unknown,
}

fn detect_pkg_type(dir: &Path) -> PkgType {
    if dir.join("Cargo.toml").exists() {
        PkgType::Rust
    } else if dir.join("pyproject.toml").exists() {
        PkgType::Python
    } else {
        PkgType::Unknown
    }
}

/// Find all lintable packages under `src_dir` (any dir with Cargo.toml or pyproject.toml).
fn find_lintable_packages(src_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut packages = Vec::new();
    if !src_dir.is_dir() {
        anyhow::bail!("{} is not a directory", src_dir.display());
    }
    for entry in std::fs::read_dir(src_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && !matches!(detect_pkg_type(&path), PkgType::Unknown) {
            packages.push(path);
        }
    }
    packages.sort();
    Ok(packages)
}

/// Lint one or more packages.
async fn run_lint(pkg: Option<String>, all: Option<String>, fix: bool) -> Result<()> {
    println!("Tagentacle Lint");
    println!("===============");

    let packages: Vec<PathBuf> = if let Some(ws_src) = all {
        let src_dir = PathBuf::from(&ws_src)
            .canonicalize()
            .with_context(|| format!("Directory not found: {}", ws_src))?;
        find_lintable_packages(&src_dir)?
    } else if let Some(p) = pkg {
        let d = PathBuf::from(&p)
            .canonicalize()
            .with_context(|| format!("Package directory not found: {}", p))?;
        vec![d]
    } else {
        vec![std::env::current_dir()?]
    };

    if packages.is_empty() {
        println!("No lintable packages found.");
        return Ok(());
    }

    println!("Packages to lint: {}", packages.len());
    for p in &packages {
        println!("  - {}", p.display());
    }
    println!();

    let mut total_pass = 0u32;
    let mut total_fail = 0u32;

    for pkg_dir in &packages {
        let pkg_name = pkg_dir
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let success = match detect_pkg_type(pkg_dir) {
            PkgType::Rust => lint_rust(pkg_dir, &pkg_name, fix).await?,
            PkgType::Python => lint_python(pkg_dir, &pkg_name, fix).await?,
            PkgType::Unknown => {
                println!("[{}] ⚠ Unknown package type, skipping.", pkg_name);
                continue;
            }
        };

        if success {
            total_pass += 1;
        } else {
            total_fail += 1;
        }
    }

    println!("==========================");
    println!("Lint Summary: {} passed, {} failed", total_pass, total_fail);
    println!("==========================");

    if total_fail > 0 {
        std::process::exit(1);
    }
    Ok(())
}

/// Lint a Rust package: cargo clippy + cargo fmt --check (or --fix).
async fn lint_rust(pkg_dir: &Path, pkg_name: &str, fix: bool) -> Result<bool> {
    println!("[{}] Rust lint...", pkg_name);
    let mut ok = true;

    // clippy
    {
        let mut cmd = Command::new("cargo");
        if fix {
            cmd.args(["clippy", "--fix", "--allow-dirty", "--allow-staged"]);
        } else {
            cmd.args(["clippy", "--", "-D", "warnings"]);
        }
        let status = cmd
            .current_dir(pkg_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .await
            .context("Failed to run cargo clippy")?;
        if !status.success() {
            println!("[{}] ✗ clippy failed", pkg_name);
            ok = false;
        }
    }

    // fmt
    {
        let mut cmd = Command::new("cargo");
        if fix {
            cmd.arg("fmt");
        } else {
            cmd.args(["fmt", "--check"]);
        }
        let status = cmd
            .current_dir(pkg_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .await
            .context("Failed to run cargo fmt")?;
        if !status.success() {
            println!("[{}] ✗ fmt failed", pkg_name);
            ok = false;
        }
    }

    if ok {
        println!("[{}] ✓ PASSED\n", pkg_name);
    } else {
        println!("[{}] ✗ FAILED\n", pkg_name);
    }
    Ok(ok)
}

/// Lint a Python package: ruff check + ruff format --check (or --fix).
async fn lint_python(pkg_dir: &Path, pkg_name: &str, fix: bool) -> Result<bool> {
    println!("[{}] Python lint...", pkg_name);
    let mut ok = true;

    // ruff check
    {
        let mut args = vec!["tool", "run", "ruff", "check"];
        if fix {
            args.push("--fix");
        }
        args.push(".");
        let status = Command::new("uv")
            .args(&args)
            .current_dir(pkg_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .await
            .context("Failed to run ruff check. Is uv installed?")?;
        if !status.success() {
            println!("[{}] ✗ ruff check failed", pkg_name);
            ok = false;
        }
    }

    // ruff format
    {
        let args = if fix {
            vec!["tool", "run", "ruff", "format", "."]
        } else {
            vec!["tool", "run", "ruff", "format", "--check", "."]
        };
        let status = Command::new("uv")
            .args(&args)
            .current_dir(pkg_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .await
            .context("Failed to run ruff format. Is uv installed?")?;
        if !status.success() {
            println!("[{}] ✗ ruff format failed", pkg_name);
            ok = false;
        }
    }

    if ok {
        println!("[{}] ✓ PASSED\n", pkg_name);
    } else {
        println!("[{}] ✗ FAILED\n", pkg_name);
    }
    Ok(ok)
}
