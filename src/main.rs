use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use tokio_util::codec::{Framed, LinesCodec};

/// Tagentacle 核心协议消息结构 (最小集/MVP)
#[derive(Debug, Deserialize, Serialize)]
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
    // 将来扩展: AdvertiseService, CallService 等
}

/// 内部路由表结构
struct Router {
    // topic -> Vec<(node_id, tx)>
    subscriptions: HashMap<String, Vec<(String, mpsc::UnboundedSender<Value>)>>,
}

type SharedRouter = Arc<Mutex<Router>>;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:19999";
    let listener = TcpListener::bind(addr).await?;
    println!("Tagentacle Core Daemon (Rust) listening on: {}", addr);

    let router = Arc::new(Mutex::new(Router {
        subscriptions: HashMap::new(),
    }));

    loop {
        let (socket, _) = listener.accept().await?;
        let router = Arc::clone(&router);

        tokio::spawn(async move {
            match handle_connection(socket, router).await {
                Ok(_) => { /* 正常断开 */ }
                Err(e) => eprintln!("Error handling connection: {}", e),
            }
        });
    }
}

async fn handle_connection(socket: TcpStream, router: SharedRouter) -> Result<()> {
    let codec = LinesCodec::new();
    let mut framed = Framed::new(socket, codec);

    // 为当前连接创建一个消息队列，用于推送订阅消息
    let (tx, mut rx) = mpsc::unbounded_channel::<Value>();
    
    // 跟踪本连接关联的节点 ID (MVP 模型暂为 1 Link -> 1 Node 映射)
    let mut current_node_id: Option<String> = None;

    loop {
        tokio::select! {
            // 1. 监听来自此 Client 的 JSON 命令 (Raw TCP -> Framed Lines)
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
                                println!("Node '{}' subscribed to '{}'", node_id, topic);
                                current_node_id = Some(node_id.clone());
                                
                                let mut r = router.lock().await;
                                let entry = r.subscriptions.entry(topic).or_insert_with(Vec::new);
                                // 无论是否已订阅，加入 Sender
                                entry.push((node_id, tx.clone()));
                            }
                            Action::Publish { topic, sender, payload } => {
                                // 转发逻辑 (Fan-out)
                                let r = router.lock().await;
                                if let Some(subs) = r.subscriptions.get(&topic) {
                                    let push_msg = json!({
                                        "op": "message",
                                        "topic": topic,
                                        "sender": sender,
                                        "payload": payload
                                    });
                                    
                                    for (_, sub_tx) in subs {
                                        let _ = sub_tx.send(push_msg.clone());
                                    }
                                }
                            }
                        }
                    }
                    _ => break, // EOF 或读取出错，退出 Loop 并后续清理
                }
            }

            // 2. 监听需要推送给此 Client 的异步消息
            msg_to_send = rx.recv() => {
                if let Some(msg) = msg_to_send {
                    let json_line = serde_json::to_string(&msg)?;
                    framed.send(json_line).await?;
                } else {
                    break;
                }
            }
        }
    }

    // 清理逻辑 (MVP 简化: 遍历路由表移除断开的 Senders)
    if let Some(node_id) = current_node_id {
        println!("Node '{}' disconnected. Cleaning up subscriptions...", node_id);
    }
    
    Ok(())
}

