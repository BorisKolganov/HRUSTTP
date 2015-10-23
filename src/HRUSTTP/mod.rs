extern crate chrono;
extern crate url;
extern crate threadpool;

mod HttpHeader;
mod HttpResponse;
mod ContentType;


use self::chrono::offset::utc;
use self::url::{Url, UrlParser};
use self::url::percent_encoding::lossy_utf8_percent_decode;
use self::threadpool::ThreadPool;

use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::io::{Read, Write};
use std::fs::File;
use std::thread;

pub struct HRUSTTP {
    rootdir: String,
    n_cpu: u32,
    ip: String,
}

impl HRUSTTP {
    pub fn new(rootdir: String, n_cpu: u32, ip: String) -> HRUSTTP {
        HRUSTTP {rootdir: rootdir, n_cpu: n_cpu, ip: ip}
    }

    fn handle(mut stream: TcpStream, mut root: String) {
        let head = HttpHeader::HttpHeader::new(&mut stream);

        let mut response = HttpResponse::HttpResponseBuilder::new();
        response.version(head.version.clone())
            .date(utc::UTC::now())
            .server("HRUSTTP".to_string())
            .connection("close".to_string());

        match head.method.as_ref() {
            m @ "GET" | m @ "HEAD" => {
                match head.path.find("..") {
                    Some(_) => {
                        stream.write(response.code(403).description("you shoud not pass".into()).finalize().to_string().as_bytes()).ok().unwrap();
                    },
                    None => {
                        root.push_str(
                            &lossy_utf8_percent_decode(
                                UrlParser::new()
                                .base_url(&Url::parse("http://127.0.0.1/").ok().unwrap())
                                .parse(&head.path).unwrap()
                                .serialize_path().unwrap()
                                .as_bytes()
                            )
                        );

                        let path = Path::new(&root);
                        let mut path_buf = path.to_path_buf();
                        let mut file: File;
                        let extension = match path.extension() {
                            Some (ext) => {
                                file = match File::open(Path::new(path_buf.as_path())) {
                                    Ok(file) => file,
                                    Err(_) => {
                                        stream.write(response.code(404).description("Not found".into()).finalize().to_string().as_bytes()).ok().unwrap();
                                        return
                                    }
                                };
                                ext.to_str().unwrap()
                            },
                            None => {
                                path_buf.push("index.html");
                                file = match File::open(Path::new(path_buf.as_path())) {
                                    Ok(file) => file,
                                    Err(_) => {
                                        stream.write(response.code(403).description("No index file".into()).finalize().to_string().as_bytes()).ok().unwrap();
                                        return
                                    }
                                };
                                "html"
                            },
                        };
                        stream.write(response
                            .content_length(file.metadata().ok().unwrap().len())
                            .content_type(ContentType::lookup(extension))
                            .code(200)
                            .description("OK".into())
                            .finalize()
                            .to_string()
                            .as_bytes()).ok().unwrap();

                        if m == "GET" {
                           loop {
                               let mut buf = [0; 1024];
                               match file.read(&mut buf) {
                                   Ok(0) => { break; },
                                   Ok(m) => { stream.write(&buf[0..m]).ok().unwrap(); },
                                   Err(e) => {
                                       println!("{:?}", e);
                                       break;
                                   },
                               }
                           }
                       }
                    }
                }
            },
            _ => {
                stream.write(response.code(400).description("sorry about that".to_string()).finalize().to_string().as_bytes()).ok().unwrap();
            },

        };

    }
    pub fn go(&self) {
        let tcp = TcpListener::bind(&*self.ip).unwrap();
        println!("Welcome to HRUSTTP \r\nBind: {}", self.ip);

        if self.n_cpu == 0 {
            println!("You start with thread per request");
            for stream in tcp.incoming() {
                match stream {
                    Ok(stream) => {
                        let root = self.rootdir.clone();
                        thread::spawn(move || {
                            HRUSTTP::handle(stream, root);
                        });
                    }
                    Err(e) => {
                        println!("ERROR {}", e);
                    }
                }
            }
        } else {
            println!("You start with threadpool with pool = {}", self.n_cpu);
            let pool = ThreadPool::new(self.n_cpu as usize);
            for stream in tcp.incoming() {
                match stream {
                    Ok(stream) => {
                        let root = self.rootdir.clone();
                        pool.execute(move || {
                            HRUSTTP::handle(stream, root)
                        })
                    },
                    Err(e) => {
                        println!("ERROR {}", e);
                    }
                }
            }
        }
    }
}
