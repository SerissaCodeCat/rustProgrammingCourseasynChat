use async_std::{task, net};
use chat::utils::ChatResult;
use std::sync::Arc;
use async_std::prelude::*;

mod connections;
mod chats;
mod chats_map;

use connections::handle;

use crate::chats_map::ChatTracker;

fn log_error(result: ChatResult<()>) {
    if let Err(error) = result {
        println!("Error {}", error)
    }
}

fn main()-> ChatResult<()>{
    let address = std::env::args().nth(1).expect("server ADDRESS");
    let chat_table = Arc::new(chats_map::ChatTracker::new());

    async_std::task::block_on(async {
        let listener = net::TcpListener::bind(address).await?;
        let mut new_connections = listener.incoming();
        while let Some(socket_result) = new_connections.next().await{
            let socket = socket_result?;
            let chat = chat_table.clone();

            task::spawn(async { 
                log_error(handle(socket, chats).await);
            });
        }    
        Ok(())
    })
}