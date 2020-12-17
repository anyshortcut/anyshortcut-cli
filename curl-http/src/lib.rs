use std::cell::{RefCell, RefMut};
use std::fmt;
use std::io::{Read, Write};

use serde::de::DeserializeOwned;
use serde::{self, Serialize};
use thiserror::Error;

/// Shortcut alias for results of this module.
pub type Result<T> = std::result::Result<T, RequestError>;

/// A enum represents HTTP methods.
#[derive(PartialEq, Debug)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Method::Get => write!(f, "GET"),
            Method::Head => write!(f, "HEAD"),
            Method::Post => write!(f, "POST"),
            Method::Put => write!(f, "PUT"),
            Method::Delete => write!(f, "DELETE"),
        }
    }
}

///
/// A Http client base on curl.
///
pub struct Client {
    shared_handle: RefCell<curl::easy::Easy>,
    base_url: String,
    user_agent: String,
}

impl Client {
    /// Initialize a curl http client based on the **base_url**.
    pub fn new(base_url: &str) -> Client {
        Client {
            shared_handle: RefCell::new(curl::easy::Easy::new()),
            base_url: base_url.to_string(),
            user_agent: "curl-http".to_string(),
        }
    }

    /// Set the User-Agent header. Default is `curl-http`
    pub fn set_user_agent(&mut self, user_agent: &str) {
        self.user_agent = user_agent.to_string();
    }

    /// Make a specific method request.
    pub fn request(&self, method: Method, endpoint: &str) -> Result<Request> {
        let url = format!("{}{}", self.base_url, endpoint);
        let mut handle = self.shared_handle.borrow_mut();
        handle.reset();
        Request::new(handle, method, &url)?.with_user_agent(&self.user_agent)
    }

    /// High level HTTP **GET** method
    pub fn get(&self, endpoint: &str) -> Result<Response> {
        self.request(Method::Get, endpoint)?.send()
    }

    /// High level HTTP **POST** method
    pub fn post<S: Serialize>(&self, endpoint: &str, body: &S) -> Result<Response> {
        self.request(Method::Post, endpoint)?
            .with_json_body(body)?
            .send()
    }

    /// High level HTTP **PUT** method
    pub fn put<S: Serialize>(&self, endpoint: &str, body: &S) -> Result<Response> {
        self.request(Method::Put, endpoint)?
            .with_json_body(body)?
            .send()
    }

    /// High level HTTP **DELETE** method
    pub fn delete(&self, endpoint: &str) -> Result<Response> {
        self.request(Method::Delete, endpoint)?.send()
    }
}

/// The struct represents the HTTP request.
pub struct Request<'a> {
    handle: RefMut<'a, curl::easy::Easy>,
    headers: curl::easy::List,
    url: String,
    body: Option<Vec<u8>>,
}

impl<'a> Request<'a> {
    pub fn new(
        mut handle: RefMut<'a, curl::easy::Easy>,
        method: Method,
        url: &str,
    ) -> Result<Request<'a>> {
        match method {
            Method::Get => handle.get(true)?,
            Method::Head => {
                handle.get(true)?;
                handle.custom_request("HEAD")?;
                handle.nobody(true)?;
            }
            Method::Post => handle.custom_request("POST")?,
            Method::Put => handle.custom_request("PUT")?,
            Method::Delete => handle.custom_request("DELETE")?,
        }

        Ok(Request {
            handle,
            headers: curl::easy::List::new(),
            url: url.to_string(),
            body: None,
        })
    }

    /// Set the HTTP header.
    pub fn with_header(mut self, key: &str, value: &str) -> Result<Request<'a>> {
        self.headers.append(&format!("{}: {}", key, value))?;
        Ok(self)
    }

    /// Set custom User-Agent.
    pub fn with_user_agent(mut self, ua: &str) -> Result<Request<'a>> {
        self.headers.append(&format!("User-Agent: {}", ua))?;
        Ok(self)
    }

    /// Set custom url arguments or querystring.
    pub fn with_arguments(mut self, args: &str) -> Result<Request<'a>> {
        self.url = format!("{}?{}", self.url, args);
        Ok(self)
    }

    /// Set the JSON request body for the request.
    pub fn with_json_body<S: Serialize>(mut self, body: &S) -> Result<Request<'a>> {
        let mut body_bytes: Vec<u8> = vec![];
        // Serialize json object to bytes
        serde_json::to_writer(&mut body_bytes, &body).map_err(|_| RequestError::InvalidJsonBody)?;

        self.body = Some(body_bytes);
        self.headers.append("Content-Type: application/json")?;
        Ok(self)
    }

    /// Sends the request and reads the response body into the response object.
    pub fn send(mut self) -> Result<Response> {
        self.handle.http_headers(self.headers)?;
        self.handle.url(&self.url)?;

        match self.body {
            Some(ref body) => {
                let mut body: &[u8] = &body[..];
                self.handle.upload(true)?;
                self.handle.in_filesize(body.len() as u64)?;
                handle_request(&mut self.handle, &mut |buffer| {
                    body.read(buffer).unwrap_or(0)
                })
            }
            None => handle_request(&mut self.handle, &mut |_| 0),
        }
    }
}

fn handle_request(
    handle: &mut curl::easy::Easy,
    read: &mut dyn FnMut(&mut [u8]) -> usize,
) -> Result<Response> {
    let mut response_body = vec![];
    let mut response_headers = vec![];

    {
        let mut handle = handle.transfer();

        handle.read_function(move |buffer| Ok(read(buffer)))?;

        handle.write_function(|data| {
            Ok(match response_body.write_all(data) {
                Ok(_) => data.len(),
                Err(_) => 0,
            })
        })?;

        handle.header_function(|data| {
            response_headers.push(String::from_utf8_lossy(data).into_owned());
            true
        })?;
        handle.perform()?;
    }

    Ok(Response {
        status: handle.response_code()?,
        headers: response_headers,
        body: Some(response_body),
    })
}

/// Type alias for **u32** http status.
pub type HttpStatus = u32;

/// The struct represents the HTTP response.
#[derive(Clone, Debug)]
pub struct Response {
    status: HttpStatus,
    headers: Vec<String>,
    body: Option<Vec<u8>>,
}

impl Response {
    pub fn status(&self) -> HttpStatus {
        self.status
    }

    pub fn failed(&self) -> bool {
        self.status >= 400 && self.status <= 600
    }

    pub fn ok(&self) -> bool {
        !self.failed()
    }

    /// Deserialize the response body into the given type
    pub fn deserialize<T: DeserializeOwned>(&self) -> Result<T> {
        if self.ok() {
            Ok(serde_json::from_reader(match self.body {
                Some(ref body) => body,
                None => &b""[..],
            })
                .map_err(|_| RequestError::InvalidJson)?)
        } else {
            Err(RequestError::RequestFailed)
        }
    }
}

#[derive(Error, Debug)]
pub enum RequestError {
    #[error(transparent)]
    CurlError(#[from] curl::Error),
    #[error("Request failed")]
    RequestFailed,
    #[error("Could not serialize value as JSON")]
    InvalidJsonBody,
    #[error("Could not parse JSON response")]
    InvalidJson,
}
