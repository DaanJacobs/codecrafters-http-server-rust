use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
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

    pub fn from_stream(stream: &TcpStream) -> Self {
        let mut reader = BufReader::new(stream);
        let mut builder = HttpRequestBuilder::new();

        let mut first_line = String::new();
        reader.read_line(&mut first_line).unwrap();

        let mut first_line = first_line.split(' ');
        let method = HttpMethod::from_string(first_line.next().unwrap()).unwrap();
        let path = first_line.next().unwrap().to_owned();
        let version = first_line.next().unwrap().to_owned();
        builder.with_method(method);
        builder.with_path(path);
        builder.with_version(version);

        let mut content_length = 0;
        loop {
            let mut header = String::new();
            reader.read_line(&mut header).unwrap();

            if header == "\r\n" {
                break;
            }

            let (key, value) = header.split_once(':').unwrap();
            builder.with_header(key.trim().to_owned(), value.trim().to_owned());
            if key.trim().to_lowercase() == "content-length" {
                content_length = value.trim().parse().unwrap();
            }
        }

        if content_length > 0 {
            let mut body = vec![0; content_length];
            reader.read_exact(&mut body).unwrap();

            builder.with_body(String::from_utf8_lossy(&body).to_string());
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

    fn with_body(&mut self, body: String) -> &Self {
        self.body = Some(body);
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
