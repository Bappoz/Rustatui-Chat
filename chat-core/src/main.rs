use chat_core::server::server_config::ServerConfig;
use chat_core::server::server::ChatServer;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inicializa o sistema de logs
    // Logs vão para stderr para não interferir com TUIs
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_target(false)
        .init();

    let config = ServerConfig::default();
    let server = ChatServer::new(config);
    
    server.run().await
}

