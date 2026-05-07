mod http;
mod store;
mod handlers;
mod templates;

use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let store = Arc::new(Mutex::new(store::Store::load()));
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).expect("failed to bind to 127.0.0.1:8080");
    println!("╔══════════════════════════════════════╗");
    println!("║      Eko Vision Vault  ⚡             ║");
    println!("╠══════════════════════════════════════╣");
    println!("║  http://{}             ║", addr);
    println!("║  Sovereign · Pure Rust · No Deps     ║");
    println!("╚══════════════════════════════════════╝");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let store = Arc::clone(&store);
                thread::spawn(move || handlers::handle(stream, store));
            }
            Err(e) => eprintln!("accept error: {e}"),
        }
    }
}
