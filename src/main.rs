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

    // yes this is a if else else statement, i know its not gonna be good. I only made it like this for the sake of my sanity. Anyway-
    // this is where more of the magic happens.
    //Thanks to the Message enums, you can have different messages like text, binary, closing, pings or pongs!
    // As you can see here, it will filter out if its a closing or a ping message. If its neither, it will try to display the message anyway.
    
    while let Some(message) = read.next().await {
        let message = message.expect("Failed to read :<");
        if message.is_ping(){
            write.send(Message::Pong("Pong!".into())).await.expect("Failed to reply to ping");
            println!("Received ping! Replying with pong!");
        }
        else if message.is_close(){
            println!("Received closing connection!");
        }
        else {
            println!("Received message: {}", message);
        }
    }
}