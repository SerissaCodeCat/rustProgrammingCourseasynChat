use serde::{Deserialize, Serialize};
use std::sync::Arc;
pub mod utils;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Client {
    Join {
        chat_name: Arc<String>
    }, 
    Post {
        chat_name: Arc<String>,
        message: Arc<String>
    }
    
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Server {
    Message {
        chat_name: Arc<String>, 
        msg: Arc<String>,   
    },
    Error(String)
}
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
