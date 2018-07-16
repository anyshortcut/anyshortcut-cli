use http;
use http::{Client, Response};
use std::rc::Rc;

static API_URL: &'static str = "https://api.anyshortcut.com";

thread_local! {
    static API: Rc<Api> = Rc::new(Api::new());
}

///
/// The Shortcut struct.
///
#[derive(Debug)]
pub struct Shortcut {
    pub key: String,
    pub url: String,
    pub title: String,
    pub comment: String,
    pub favicon: String,
    pub domain: String,
    pub open_times: i32,
}

pub struct Api {
    client: Client,
    token: Option<String>,
}

impl Api {
    pub fn new() -> Api {
        Api {
            client: Client::new(API_URL),
            token: None,
        }
    }

    /// Returns the current api for the thread.
    pub fn get_current() -> Rc<Api> {
        API.with(|api| api.clone())
    }

    pub fn login_with_token(&self, token: &str) -> http::Result<Response> {
        self.client.get("/login")
    }
}

