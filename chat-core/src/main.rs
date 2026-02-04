use chat_core::server::server_config::ServerConfig;
use chat_core::server::server::ChatServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ServerConfig::default();
    let server = ChatServer::new(config);
    
    server.run().await
}

