use std::collections::HashMap;

use itertools::Itertools;

pub struct HttpResponse {
    version: String,
    status_code: usize,
    status_message: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl HttpResponse {
    fn new(
        version: String,
        status_code: usize,
        status_message: String,
        headers: HashMap<String, String>,
        body: Option<String>,
    ) -> Self {
        HttpResponse {
            version,
            status_code,
            status_message,
            headers,
            body,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let status_line = format!(
            "{} {} {}",
            self.version, self.status_code, self.status_message
        );
        let headers = self
            .headers
            .iter()
            .map(|(key, value)| format!("{}: {}", key, value))
            .join("\r\n");
        let body = self.body.as_ref().map_or("", |b| b).to_owned();
        format!("{}\r\n{}\r\n\r\n{}", status_line, headers, body)
            .as_bytes()
            .to_vec()
    }
}

pub struct HttpResponseBuilder {
    version: String,
    status_code: usize,
    status_message: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl HttpResponseBuilder {
    pub fn new() -> Self {
        HttpResponseBuilder {
            version: String::from("HTTP/1.1"),
            status_code: 200,
            status_message: String::from("OK"),
            headers: HashMap::new(),
            body: None,
        }
    }

    pub fn with_status(mut self, code: usize, message: String) -> Self {
        self.status_code = code;
        self.status_message = message;
        self
    }

    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    pub fn with_body(mut self, body: String) -> Self {
        self.headers
            .insert(String::from("Content-Length"), body.len().to_string());
        self.body = Some(body);
        self
    }

    pub fn build(self) -> HttpResponse {
        HttpResponse::new(
            self.version,
            self.status_code,
            self.status_message,
            self.headers,
            self.body,
        )
    }
}
