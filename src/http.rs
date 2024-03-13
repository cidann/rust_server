
use std::{collections::HashMap, io::{BufRead, BufReader}, net::TcpStream, str::Lines};

pub struct HttpRequest{
    method:String,
    pub url:String,
    version:String,
    headers:HashMap<String,String>,
    body:String
}

impl HttpRequest {
    pub fn new(data:String)->Result<HttpRequest,String>{
        let mut lines=data.lines();
        let (method,url,version)=HttpRequest::parse_request(&mut lines)?;
        let headers=HttpRequest::parse_headers(&mut lines)?;
        let body=HttpRequest::parse_body(&mut lines);

        Ok(HttpRequest{
            method,
            url,
            version,
            headers,
            body
        })
    }
    fn parse_request(line_iter:&mut Lines)->Result<(String,String,String),String>{
        match line_iter.next() {
            Some(request_line) => {
                let request_line_fields:Vec<&str>=request_line.split(" ").collect();
                match request_line_fields.len() {
                    3=>{
                        let (method,url,version)=(
                            String::from(request_line_fields[0]),
                            String::from(request_line_fields[1]),
                            String::from(request_line_fields[2])
                        );

                        Ok((method,url,version))
                    }
                    _=>{
                        Err(format!("Invalid http request line {request_line}"))
                    }
                }
            },
            None =>  Err(String::from("No http request line")),
        }
    }

    fn parse_headers(line_iter:&mut Lines)->Result<HashMap<String,String>,String>{
        line_iter
        .map(|line| line.splitn(2,":").collect())
        .map(|header:Vec<&str>| match header.len() {
            2=>{
                Ok((String::from(header[0]),String::from(header[1])))
            }
            _=>{
                Err(format!("Invalid header: {}",header.join("")))
            }
        })
        .collect()
    }

    fn parse_body(line_iter:&mut Lines)->String{
        line_iter.collect()
    }
}

pub fn read_http_to_string(stream:&TcpStream)->Result<String,String>{
    let mut result=Ok(());
    let request:String=
        BufReader::new(stream)
        .lines()
        .scan(&mut result, |state,line|{
            match line {
                Ok(line) => Some(line),
                Err(_) => {**state=Err(());None},
            }
        })
        .take_while(|line|!line.is_empty())
        .flat_map(|line| [line,"\n".to_string()])
        .collect();

    match result {
        Ok(_) => {
            Ok(request)
        },
        Err(msg) => {
            Err("erroneous request format".to_string())
        },
    }
}