use std::net::SocketAddr;

use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::TcpListener, select };

const LOCAL: &str = "localhost:4556";
const _BUFF_SIZE: usize = 1024;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = TcpListener::bind(LOCAL)
        .await?;

    let (sender, _) = tokio::sync::broadcast::channel::<(String, SocketAddr)>(32);
    println!("\n Welcome to Rusty Chat ");
    
    loop {
        let (mut stream, client_addr) = server.accept().await?;
        let client_sender = sender.clone();
        let mut client_receiver = sender.subscribe();

        tokio::spawn(async move {
            let (streamer_reader, mut streamer_writer) = stream.split();

            let mut stream_buff_reader = BufReader::new(streamer_reader);
            let mut client_input = String::new();

            loop{
                select! {
                    _ = stream_buff_reader.read_line(&mut client_input) => {
                        _ = client_sender.send((client_input.clone(), client_addr));
                        client_input.clear();
                    },
                    Ok((message, message_client)) = client_receiver.recv() => {
                        if message_client != client_addr {
                            let fmt_message = format!("{}: {}", message_client, message);
                            streamer_writer.write_all(fmt_message.as_bytes()).await.unwrap();
                        }
                    },
                }
            }
        });
    }
}