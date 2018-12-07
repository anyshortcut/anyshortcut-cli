use clap::crate_version;
use curl_http::{Client, Response, Result};
use failure::Fail;
use models::*;
use serde::de::DeserializeOwned;
use serde_json;
use serde_derive::Deserialize;
use std::fmt;
use std::rc::Rc;

const API_URL: &str = "https://api.anyshortcut.com";

thread_local! {
    static API: Rc<Api> = Rc::new(Api::new());
}

#[derive(Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub code: u32,
    pub data: T,
    pub message: String,
}

#[derive(Debug, Fail)]
pub struct ApiError {
    pub code: u32,
    pub message: String,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum ApiErrorKind {
    #[fail(display = "Access token required.")]
    AccessTokenRequired,
    #[fail(display = "Invalid access token.")]
    InvalidToken,
    #[fail(display = "Unknown error.")]
    UnknownError,

}

impl<T: fmt::Display> fmt::Display for ApiResponse<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, "\n  code: {},", self.code)?;
        write!(f, "\n  data: {},", self.data)?;
        write!(f, "\n  message: {}", self.message)?;
        write!(f, "\n}}")?;
        Ok(())
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, "\n  code: {},", self.code)?;
        write!(f, "\n  message: {}", self.message)?;
        write!(f, "\n}}")?;
        Ok(())
    }
}

impl<T> From<ApiResponse<T>> for ApiError {
    fn from(response: ApiResponse<T>) -> ApiError {
        ApiError {
            code: response.code,
            message: response.message,
        }
    }
}

pub struct Api {
    client: Client,
}

impl Api {
    pub fn new() -> Api {
        let mut client = Client::new(API_URL);
        client.set_user_agent(&format!("anyshortcut-cli/{}", crate_version!()));
        Api {
            client
        }
    }

    /// Returns the current api for the thread.
    pub fn get_current() -> Rc<Api> {
        API.with(|api| api.clone())
    }

    pub fn login_with_access_token(&self, access_token: &str) -> Result<serde_json::Value> {
        let response = self.client.get(&format!("/user/login?access_token={}", access_token))?;
        self.handle_http_response(&response)
    }

    pub fn get_all_shortcuts(&self) -> Result<ShortcutData> {
        let access_token = Meta::get_token();
        let response = self.client.get(
            &format!("/shortcuts/all?nested=false&access_token={}", access_token))?;
        self.handle_http_response(&response)
    }

    /// Handle http response internally to return correct api error according to api response code.
    fn handle_http_response<T: DeserializeOwned>(&self, response: &Response) -> Result<T> {
        let api_response = response.deserialize::<ApiResponse<serde_json::Value>>()?;
        match api_response.code {
            200 => {
                let response = response.deserialize::<ApiResponse<T>>()?;
                Ok(response.data)
            }
            1000 => Err(ApiError::from(api_response)
                .context(ApiErrorKind::AccessTokenRequired)
                .into()),
            1001 | 1002 => Err(ApiError::from(api_response)
                .context(ApiErrorKind::InvalidToken)
                .into()),
            _ => Err(ApiError::from(api_response)
                .context(ApiErrorKind::UnknownError)
                .into()),
        }
    }
}

