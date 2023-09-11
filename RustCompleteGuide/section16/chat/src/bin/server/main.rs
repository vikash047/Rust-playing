use std::net;
use async_std::task;
use chat::utils::ChatResult;
use std::sync::Arc;
use async_std::prelude::*;

mod connection;
mod chats;
mod chats_map;

use connection::handle;
fn main() -> ChatResult<()> {
    let adder = std::env::args().nth(1).expect("server ADDRESS");

    let chat_table = Arc::new(chats_map::ChatTracker::new());

    async_std::task::block_on(async {
        let listener = net::TcpStream::bind(adder).await?;
        let mut new_connections = listener.incoming();

        while let Some(socket_result) = new_connections.next().await {
            let socket = socket_result?;
            let chats = chat_table.clone();

            task::spawn(async {
                log_error(handle(socket, chats).await);
            });
        }
        Ok(())
    })
}

fn log_error(result: ChatResult<()>) {
    if let Err(error) = result {
        println!("Error {}", error);
    }
}