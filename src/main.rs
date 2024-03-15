use std::{env, io::Error};

use futures_util::{ SinkExt, StreamExt, };
use log::info;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // our logger!
    let _ = env_logger::try_init();
    // it takes in a address and port number. if nothing is added, it will use localhost:8080
    let addr: String = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    // create an event loop and TCP listener we'll accept connections on!
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    info!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during Handshake. --> ");
    // print info msg for the new websocket message!
    info!("New WebSocket connection: {}", addr);

    let (mut write, mut read) = ws_stream.split();

    // this is where more of the magic happens.
    //Thanks to the Message enums, you can have different messages like text, binary, closing, pings or pongs!
    
    while let Some(message) = read.next().await {
        let message = message.expect("Failed to read :<");
        match message {
            Message::Close(_) => {
                println!("Closing connection to: {}", addr)
            }
            Message::Text(_) => {
                println!("Received Message from {addr}: {}", message)
            }
            Message::Binary(_) => {
                println!("Received Binary from {addr}: {}", message)
            }
            Message::Ping(_) => {
                println!("Received ping from: {}", addr);
                write
                    .send(Message::Pong("Pong!".into()))
                    .await
                    .expect("Failed to send pong :(");
            }
            Message::Pong(_) => {
                println!("Received pong from: {}?", addr);
                write
                    .send(Message::Ping("Ping".into()))
                    .await
                    .expect("Failed to send ping :<");
            }
            // I still don't know what frames are, so this may be thrown out unless i find something
            Message::Frame(_) => {
                println!("Received frame: {}", message)
            }
        }
    }
}