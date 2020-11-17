use std::net::{TcpListener,TcpStream};
fn main() {
    setup();
    run();
}
fn setup(){
    println!("server booting up");
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();
}
fn run(){
    let mut server_state = true;
    println!("Server running");
    while(server_state){
        flag = false;
        println!("server shutting down");
    }
}