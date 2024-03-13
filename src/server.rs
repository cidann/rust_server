use std::{fs::File, io::{Read, Write}, net::{self, TcpStream}, path::Path};

use crate::{http, threadpool::ThreadPool};
use super::http::HttpRequest;

#[derive(Clone)]
pub struct ServerConfig{
    ip:String,
    port:u16,
    root:String,
}

pub struct Server{
    config:ServerConfig,
    pool:ThreadPool
}

impl ServerConfig {
    pub fn new(ip:String,port:u16,root:String)->ServerConfig{
        ServerConfig{
            ip,
            port,
            root
        }
    }
}

impl Server {
    pub fn new(config:ServerConfig,workers:usize)->Server{
        Server{
            config,
            pool:ThreadPool::new(workers)
        }
    }

    pub fn start(&self){
        let listener=net::TcpListener::bind(format!("{}:{}",self.config.ip,self.config.port)).unwrap();
    
        for connection in listener.incoming(){
            match connection {
                Ok(stream) => {
                    let config=self.config.clone();
                    match self.pool.execute(Box::new(||handle_http_connection(stream,config))){
                        Ok(_) => {},
                        Err(msg) => {
                            println!("Request failed {msg}")
                        },
                    }
                },
                Err(_) => continue,
            }
        }
    }
}


pub fn handle_http_connection(mut stream:TcpStream,config:ServerConfig){
    let request=http::read_http_to_string(&stream);

    match request {
        Ok(request) => {
            match HttpRequest::new(request) {
                Ok(request) => {
                    dispatch_request(&mut stream, &request,config);
                    return;
                },
                Err(msg) => {
                    request_error(&mut stream, &msg, 400,config)
                },
            }
        },
        Err(msg) => {
            request_error(&mut stream, &msg , 400,config)
        },
    }

}

fn request_error(stream:&mut TcpStream,msg:&str,status:usize,config:ServerConfig){
    match stream.write(format!("HTTP/1.1 {status} Err\r\n\r\n {msg}",).as_bytes()){
        Ok(_) => {},
        Err(_) => {},
    }
}

fn dispatch_request(stream:&mut TcpStream,request:&HttpRequest,config:ServerConfig){
    //Try opening file
    let relative_path=match request.url.ends_with("/") {
        true => "index.html",
        false => &request.url,
    };
    let path=format!("{}/{}",config.root,relative_path);
    let path=Path::new(&path);
    let mut file=match File::open(path){
        Ok(file) => {
            file
        },
        Err(_) => {
            request_error(stream, "invalid path", 404,config);
            return;
        },
    };
    //Read file content
    let mut response_body=String::new();
    match file.read_to_string(&mut response_body) {
        Ok(_) => {},
        Err(_) => {
            request_error(stream, "invalid path", 404,config);
            return;
        },
    }
    //Respond to client
    let response=format!("HTTP/1.1 200 OK\r\n\r\n{}",response_body);
    match stream.write(response.as_bytes()){
        Ok(_) => {},
        Err(_) => {},
    }
}