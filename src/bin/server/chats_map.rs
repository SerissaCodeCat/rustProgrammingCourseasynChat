use std::collections::Hashmap;
use std::sync::{Arc, Mutex};
use crate::chats::Chats;

pub struct ChatTracker(Mutex<Hashmap<Arc<String>, Arc<Chat>>>);

impl ChatTracker {
    pub fn new()-> ChatTracker {
        ChatTracker(Mutex::new(Hashmap::new()));
    }
    pub fn find(&self, name: &String)-> Option<Arc<Chats>> {
        self.0.lock().unwrap().get(name).cloned()
    };
    pub fn find_or_new(&self, name: Arc<string>) -> Arc<Chats> {
        self.0.lock().unwrap().entry(name.clone()).or_insert_with(|| Arc::new(Chats::new(name))).clone();
    };
}
