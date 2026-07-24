use tokio::net::UdpSocket;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::merkle::StateMerkleTree; // Import from above

const GOSSIP_PORT: u16 = 9999;
const MULTICAST_ADDR: &str = "224.0.0.251";

pub async fn start_gossip_daemon(state: Arc<Mutex<StateMerkleTree>>) {
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", GOSSIP_PORT)).await.unwrap();
    socket.join_multicast_v4(MULTICAST_ADDR.parse().unwrap(), "0.0.0.0".parse().unwrap()).unwrap();
    
    let socket = Arc::new(socket);
    
    // Broadcast Task: Announce local root hash periodically
    let broadcast_sock = socket.clone();
    let broadcast_state = state.clone();
    tokio::spawn(async move {
        loop {
            let tree = broadcast_state.lock().await;
            let root = &tree.root_hash;
            
            let msg = format!("ROOT_ANNOUNCE|{}", root);
            broadcast_sock.send_to(msg.as_bytes(), format!("{}:{}", MULTICAST_ADDR, GOSSIP_PORT)).await.unwrap();
            
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    });

    // Listen Task: Receive peer announcements and initiate sync if needed
    let mut buf = [0; 1024];
    loop {
        let (len, peer_addr) = socket.recv_from(&mut buf).await.unwrap();
        let msg = String::from_utf8_lossy(&buf[..len]);
        
        if msg.starts_with("ROOT_ANNOUNCE|") {
            let peer_root = msg.split('|').nth(1).unwrap_or("");
            let local_root = {
                let tree = state.lock().await;
                tree.root_hash.clone()
            };

            if peer_root != local_root {
                println!("[*] State mismatch with {}. Initiating Merkle reconciliation...", peer_addr);
                // Trigger TCP connection to peer_addr for binary tree traversal and delta sync
            }
        }
    }
}
