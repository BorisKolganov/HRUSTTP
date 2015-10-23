use HRUSTTP::chrono::DateTime;
use HRUSTTP::chrono::offset::utc;


pub struct HttpResponse {
    version: String,
    code: u16,
    description: String,
    date: DateTime <utc::UTC>,
    server: String,
    content_length: u64,
    content_type: String,
    connection: String,
}

pub struct HttpResponseBuilder {
    version: String,
    code: u16,
    description: String,
    date: DateTime <utc::UTC>,
    server: String,
    content_length: u64,
    content_type: String,
    connection: String,
}

impl HttpResponseBuilder {
    pub fn new() -> HttpResponseBuilder {
        HttpResponseBuilder {
            version: String::new(),
            code: 0,
            description: String::new(),
            date: utc::UTC::now(),
            server: String::new(),
            content_length: 0,
            content_type: String::new(),
            connection: String::new(),
        }
    }
    pub fn version(& mut self, v: String) -> & mut HttpResponseBuilder {
        self.version = v;
        self
    }
    pub fn code(& mut self, c: u16) -> & mut HttpResponseBuilder {
        self.code = c;
        self
    }
    pub fn description(& mut self, d: String) -> & mut HttpResponseBuilder {
        self.description = d;
        self
    }
    pub fn date(& mut self, d: DateTime<utc::UTC>) -> & mut HttpResponseBuilder {
        self.date = d;
        self
    }
    pub fn server(& mut self, s: String) -> & mut HttpResponseBuilder {
        self.server = s;
        self
    }
    pub fn content_length(& mut self, cl: u64) -> & mut HttpResponseBuilder {
        self.content_length = cl;
        self
    }
    pub fn content_type(& mut self, ct: String) -> & mut HttpResponseBuilder {
        self.content_type = ct;
        self
    }
    pub fn connection(& mut self, c: String) -> & mut HttpResponseBuilder {
        self.connection = c;
        self
    }
    pub fn finalize(&self) -> HttpResponse {
        HttpResponse {
            version: self.version.clone(),
            code: self.code,
            description: self.description.clone(),
            date: self.date,
            server: self.server.clone(),
            content_length: self.content_length,
            content_type: self.content_type.clone(),
            connection: self.connection.clone(),
        }
    }
}

impl HttpResponse {
    pub fn to_string(&self) -> String {
        format!("{} {} {}\r\nDate: {}\r\nServer: {}\r\n{}Connection: {}\r\n\r\n",
        self.version,
        self.code,
        self.description,
        self.date.format("%a, %b %e %Y %T GMT"),
        self.server,
        match self.content_type.as_ref() {
            "" => {
                "".to_string()
            },

            _ => {
                format!("Content-Type: {}\r\nContent-Length: {}\r\n",
                    self.content_type,
                    self.content_length)
                }
        },
        self.connection)
    }
}
