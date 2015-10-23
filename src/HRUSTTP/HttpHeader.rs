use std::fmt;
use std::net::TcpStream;
use std::io::{Read, Write};

pub struct HttpHeader {
    pub method: String,
    pub path: String,
    pub version: String,
}
impl HttpHeader {
    pub fn new(stream: &mut TcpStream) -> Option<HttpHeader> {
        let mut st = String::new();
        loop {
            let mut temp = [0; 256];
            match stream.read(&mut temp) {
                Ok(0) => {break;},
                Ok(m) => {
                    st.push_str(&String::from_utf8_lossy(&temp[0..m]));
                    match st.find("\r\n\r\n") {
                        Some(_) => {
                            break;
                        },
                        None => continue
                    }
                },
                Err(e) => println!("{:?}", e)
            }
        }
        if !st.is_empty() {
            let st: Vec<&str> = st.lines().next().unwrap().split_whitespace().collect();
            Some(HttpHeader { method: st[0].to_string(), path: st[1].to_string(), version: st[2].to_string() })
        } else {
            None
        }

    }
}
impl fmt::Display for HttpHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "Method: {} \r\nPath: {}\r\nVersion {}", self.method, self.path, self.version)
   }
}
