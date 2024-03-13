use std::net;
use server::server;
use ::server::server::ServerConfig;

const IP:&'static str="localhost";
const PORT:u16=8000;
const ROOT:&'static str="./data/";

fn main() {
    let server=server::Server::new(
        ServerConfig::new(
            String::from(IP),
            PORT,
            String::from(ROOT),
        ),
         8
    );

    server.start()
}
