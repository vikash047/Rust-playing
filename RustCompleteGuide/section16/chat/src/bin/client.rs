use async_std::io::BufReader;
use async_std::prelude::*;
use async_std::{task,io,net};
use std::sync::Arc;

use chat::utils::{self, ChatResult};
use chat::{Client, Server};

fn get_value(mut line: &str) -> Option<(&str, &str)> {
    line = line.trim_start();
    if line.is_empty() {
        return None;
    }
    match line.find(char::is_whitespace) {
        Some(whitespace ) => Some((&line[0..whitespace], &line[whitespace..])),
        None => Some((line, "")),
    }
}

fn parse_input(line: &str) -> Option<Client> {
    let(input, remainder) = get_value(line)?;
    if input == "join" {
        let (chat, remainder) = get_value(remainder)?;
        if !remainder.trim_start().is_empty() {
            return None;
        }
        return Some(Client::Join{chat_name: Arc::new(chat.to_string()),});
    }

    else if input == "post" {
        let (chat, remainder) = get_value(remainder)?;
        let message = remainder.trim_start().to_string();
        return Some(Client::Post{chat_name: Arc::new(chat.to_string()), message: Arc::new(message)});
    }
    else {
        println!("Unrecognized input {:?}", line);
        return None;
    }
}
async fn send(mut send: net::TcpStream) -> ChatResult<()> {
    println!("Option:\nJoin CHAT\n post CHAT MESSAGE");

    let mut options = io::BufReader::new(io::stdin()).lines();

    while let Some(option_result) = options.next().await {
        let opt = option_result?;
        let req = match parse_input(&opt) {
            Some(req) => req,
            None => continue,
        };
        utils::send_json(&mut send, &req).await?;
        send.flush().await?;
    }
    Ok(())
}


async  fn messages(server: net::TcpStream) -> ChatResult<()> {
    let buf = io::BufReader::new(server);
    let mut stream = utils::receive(buf);
    while let Some(msg) = stream.next().await {
        match msg? {
            Server::Message {chat_name, message} => {
                println!("Chat Name: {} \n, Messsage: {}\n", chat_name, message);
            }
            Server::Error(message) => {
                println!("Error happen while getting message: {:?}", message);
            }
        }
    }
    Ok(())
}


fn main() -> ChatResult<()> {
    let addr = std::env::args().nth(1).expect("Address:PORT");
    task::block_on(async {
        let socket = net::TcpStream::connect(addr).await?;
        socket.set_nodelay(true)?;
        let send = send(socket.clone());
        let replies = messages(socket);
        Ok(())
    })
}