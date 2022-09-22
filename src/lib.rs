use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;

pub static INDEX: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Address {
    pub list: Vec<String>,
}

pub static ADDRESS: Lazy<Arc<RwLock<HashMap<String, Address>>>> =
    Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));

pub async fn get_address(server_name: &str) -> String {
    let r = ADDRESS.read().await;
    let data = r.get(server_name).unwrap();
    let mut index = INDEX.load(Ordering::Relaxed);
    let address: Option<&String> = data.list.get(index);
    let address_str;
    if address.is_some() {
        address_str = address.unwrap().as_str();
    } else {
        address_str = "";
    }
    if data.list.len() == index + 1 {
        index = 0;
    } else if data.list.len() == 0 {
        index = 0;
    } else {
        index += 1;
    }
    INDEX.store(index, Ordering::Relaxed);
    return address_str.to_string();
}

pub async fn load_address(server_name: &str, servers: Vec<String>) {
    let mut w = ADDRESS.write().await;
    w.insert(server_name.to_string(), Address { list: servers });
}
