use constants;
use failure::Fail;
use http::{Client, Response, Result};
use models::*;
use serde_json;
use std::fmt;
use std::rc::Rc;

thread_local! {
    static API: Rc<Api> = Rc::new(Api::new());
}

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub code: u32,
    pub data: serde_json::Value,
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

impl fmt::Display for ApiResponse {
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

impl From<ApiResponse> for ApiError {
    fn from(response: ApiResponse) -> ApiError {
        ApiError {
            code: response.code,
            message: response.message,
        }
    }
}

pub struct Api {
    client: Client,
    token: Option<String>,
}

impl Api {
    pub fn new() -> Api {
        Api {
            client: Client::new(constants::API_URL),
            token: None,
        }
    }

    /// Returns the current api for the thread.
    pub fn get_current() -> Rc<Api> {
        API.with(|api| api.clone())
    }

    pub fn login_with_access_token(&self, access_token: &str) -> Result<serde_json::Value> {
        let response = self.client.get(&format!("/user/login?access_token={}", access_token))?;
        self.handle_http_response(response)
    }

    pub fn get_primary_shortcuts(&self) -> Result<serde_json::Value> {
        let access_token = Meta::get_token();
        let response = self.client.get(&format!("/shortcuts/all?access_token={}", access_token))?;
        self.handle_http_response(response)
    }

    /// Handle http response internally to return correct api error according to api response code.
    fn handle_http_response(&self, response: Response) -> Result<serde_json::Value> {
        let api_response = response.deserialize::<ApiResponse>()?;

        match api_response.code {
            200 => Ok(api_response.data),
            1000 => Err(ApiError::from(api_response)
                .context(ApiErrorKind::AccessTokenRequired)
                .into()),
            1002 => Err(ApiError::from(api_response)
                .context(ApiErrorKind::InvalidToken)
                .into()),
            _ => Err(ApiError::from(api_response)
                .context(ApiErrorKind::UnknownError)
                .into()),
        }
    }
}

