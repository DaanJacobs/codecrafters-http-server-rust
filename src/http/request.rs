use std::{
    collections::HashMap,
    io::{self, BufRead},
    net::TcpStream,
};

pub enum HttpMethod {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}
impl HttpMethod {
    fn from_string(s: &str) -> Option<HttpMethod> {
        match s {
            "GET" => Some(Self::Get),
            "HEAD" => Some(Self::Head),
            "POST" => Some(Self::Post),
            "PUT" => Some(Self::Put),
            "DELETE" => Some(Self::Delete),
            "CONNECT" => Some(Self::Connect),
            "OPTIONS" => Some(Self::Options),
            "TRACE" => Some(Self::Trace),
            "PATCH" => Some(Self::Patch),
            _ => None,
        }
    }
}

pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl HttpRequest {
    fn new(
        method: HttpMethod,
        path: String,
        version: String,
        headers: HashMap<String, String>,
        body: Option<String>,
    ) -> Self {
        HttpRequest {
            method,
            path,
            version,
            headers,
            body,
        }
    }

    pub fn from_stream(mut stream: &TcpStream) -> Self {
        let buffer = io::BufReader::new(&mut stream);
        let mut builder = HttpRequestBuilder::new();
        let mut headers = true;
        for (nr, result) in buffer.lines().enumerate() {
            match result {
                Ok(line) => {
                    if nr == 1 {
                        let mut first_line = line.split(' ');

                        let method = HttpMethod::from_string(first_line.nth(0).unwrap()).unwrap();
                        let path = first_line.nth(1).unwrap().to_owned();
                        let version = first_line.nth(2).unwrap().to_owned();
                        builder.with_method(method);
                        builder.with_path(path);
                        builder.with_version(version);
                    } else if headers {
                        let mut key_value = line.split(' ');
                        let key = key_value.nth(0).unwrap().to_owned();
                        let value = key_value.nth(1).unwrap().to_owned();
                        builder.with_header(key, value);
                    } else {
                        if line.is_empty() {
                            headers = false;
                        } else {
                            builder.with_body_line(line);
                        }
                    }
                }
                Err(_) => {}
            }
        }

        builder.build()
    }
}

struct HttpRequestBuilder {
    method: HttpMethod,
    path: String,
    version: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl HttpRequestBuilder {
    pub fn new() -> Self {
        HttpRequestBuilder {
            method: HttpMethod::Get,
            path: String::new(),
            version: String::new(),
            headers: HashMap::new(),
            body: None,
        }
    }

    fn with_method(&mut self, method: HttpMethod) -> &Self {
        self.method = method;
        self
    }

    fn with_path(&mut self, path: String) -> &Self {
        self.path = path;
        self
    }

    fn with_version(&mut self, version: String) -> &Self {
        self.version = version;
        self
    }

    fn with_header(&mut self, key: String, value: String) -> &Self {
        self.headers.insert(key, value);
        self
    }

    fn with_body_line(&mut self, body: String) -> &Self {
        if let Some(old_body) = self.body.as_mut() {
            old_body.push_str(&String::from("\n\r\n\r"));
            old_body.push_str(&body);
        } else {
            self.body = Some(body);
        };
        self
    }

    pub fn build(self) -> HttpRequest {
        HttpRequest::new(
            self.method,
            self.path,
            self.version,
            self.headers,
            self.body,
        )
    }
}
