use async_std::io::BufRead;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::{task,io,net};
use std::ops::RemAssign;
use std::sync::Arc;

use chat::utils::{self, ChatResult};
use chat::{Client, Server};

fn get_value(mut input: &str)-> Option<(&str, &str)> {
    input = input.trim_start();
    if input.is_empty(){
        return None;
    }

    match input.find(char::is_whitespace) {
        Some(whitespace)=> Some((&input[0..whitespace], &input[whitespace..])),
        None => Some((input, "")),   
    }
}

fn parse_input(line: &str) -> Option<Client> {
    let(input, remainder) = get_value(line)?;
    if input.to_lowercase() == "join" {
        let (chat, remainder) = get_value(remainder)?;
        if !remainder.trim_start().is_empty(){
            return None;
        }
        return Some(Client::Join { chat_name: Arc::new(chat.to_string()) });
    }
    else if input.to_lowercase() == "post" {
        let(chat, remainder) = get_value(remainder)?;
        let message = remainder.trim_start().to_string();
        return Some(Client::Post { chat_name: Arc::new(chat.to_string()), message: Arc::new(message)});

    }
    else {
        println!("unrecognised input{:?}", line);
        return None;
    }
}

async fn send(mut send:TcpStream) -> ChatResult<()>{
    println!("Options:\n   Join CHATNAME\n   Post CHATNAME MESSAGE\n");
    let mut options = io::BufReader::new(io::stdin()).lines();

    while let Some(option_result) = options.next().await {
        let opt = option_result?;
        let req = match parse_input(&opt){
            Some(req)=> req,
            None => continue,
        };
        utils::send_json(&mut send, &req).await?;
        send.flush().await?;
    }
    Ok(())
}

fn main(){

}