use crate::{MessageResult, SharedState};
use chrono::Local;
use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::broadcast::Sender;

pub async fn handle_client(mut stream: TcpStream, state: SharedState, client_id: usize) -> MessageResult {
    let peer_addr = match stream.peer_addr() {
        Ok(addr) => addr,
        Err(_) => "127.0.0.1:0".parse().unwrap(),
    };
    tracing::info!("Client {} connected from {}", client_id, peer_addr);

    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader).lines();
    let sender = state.lock().await.message_sender.clone();

    let username = format!("user_{}", client_id);
    {
        let mut s = state.lock().await;
        s.add_client(username.clone(), sender.clone()).await;
        s.update_username(client_id, username.clone()).await;
    }

    sender.send(format!("[{}] {} joined the chat", Local::now().format("%Y-%m-%d %H:%M:%S"), username)).ok();

    loop {
        let line_result = reader.next_line().await;
        
        match line_result {
            Ok(Some(line)) => {
                let trimmed = line.trim();
                if trimmed.is_empty() { continue; }
                
                if let Err(e) = process_message(trimmed, client_id, &state, &sender).await {
                    let _ = writer.write_all(format!("[ERROR] {}\n", e).as_bytes()).await;
                }
            }
            Ok(None) => {
                tracing::info!("Client {} disconnected (EOF)", client_id);
                sender.send(format!("[{}] {} left the chat", Local::now().format("%Y-%m-%d %H:%M:%S"), username)).ok();
                break;
            }
            Err(e) => {
                tracing::error!("Stream error for client {}: {}", client_id, e);
                break;
            }
        }
    }

    state.lock().await.remove_client(client_id).await;
    tracing::info!("Client {} cleanup complete", client_id);
    Ok(format!("Client {} disconnected", client_id))
}

async fn process_message(
    command: &str,
    client_id: usize,
    state: &SharedState,
    sender: &Sender<String>,
) -> MessageResult {
    if command.starts_with('/') {
        let parts: Vec<&str> = command.splitn(2, ' ').collect();
        let cmd = parts[0];
        let arg = parts.get(1).map(|s| s.to_string()).unwrap_or_default();

        match cmd {
            "/name" => {
                if arg.is_empty() {
                    return Err("Usage: /name <new_username>".to_string());
                }
                if arg.len() > 32 {
                    return Err("Username too long (max 32 chars)".to_string());
                }
                let old = state.lock().await.get_username(client_id).await.unwrap_or_default();
                state.lock().await.update_username(client_id, arg.clone()).await;
                sender.send(format!("[{}] {} is now known as {}", Local::now().format("%Y-%m-%d %H:%M:%S"), old, arg)).ok();
            }
            "/quit" => {
                return Err("DISCONNECT".to_string());
            }
            "/users" => {
                let users = state.lock().await.get_all_usernames().await;
                let list = users.join(", ");
                sender.send(format!("[{}] Online users: {}", Local::now().format("%Y-%m-%d %H:%M:%S"), list)).ok();
            }
            _ => {
                return Err(format!("Unknown command: {}", cmd));
            }
        }
    } else {
        let username = state.lock().await.get_username(client_id).await.unwrap_or_else(|| format!("user_{}", client_id));
        sender.send(format!("[{}] {}: {}", Local::now().format("%Y-%m-%d %H:%M:%S"), username, command)).ok();
    }
    Ok(String::new())
}