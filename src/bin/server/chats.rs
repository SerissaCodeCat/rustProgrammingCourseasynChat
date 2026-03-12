use async_std::task;
use crate::connection::Leaving;
use std::sync::Arc;
use tokio::sync::broardcast::*;

use Chat::Server;
use tokio::sync::broadcast::error::RecvError;

pub struct Chats {
    name: Arc<String>,
    publisher: broardcast::Sender<Arc<String>>,
}

impl Chats {
    pub fn new(name: Arc<String>)-> Chats {
        let (publisher, _)= broardcast::channel(1000);
        Chats{name, publisher}
    }

    pub fn join(&self, leaving: Arc<Leaving>) {
        let reciever = self.publisher.subscribe();
        task::spawn(sub(self.name.clone(), reciever, leaving));
    }

    pub fn post(&self, message: Arc<string>) {
        let _ = self.publisher.send(message);
    }
}
async fn sub(chatname: Arc<string>, mut reciever: broardcast::Reciever<Arc<String>>, leaving: Arc<Leaving> ) {
    loop{
        let packet = match reciever.recv().await {
            Ok(message) => Server::Message{
                chat_name: chat_name.clone(),
                message: message.clone()
            }, 
            Err(RecvError::Lagged(n))=> {
                Server::Error(format!("Dropped {} Messages from chat named: {}", n, chat_name))
            },
            Err(RecvError::Closed)=> break, 
        };

        if leaving.send(packet).await.is_err(){
            break;
        };
    }
}
