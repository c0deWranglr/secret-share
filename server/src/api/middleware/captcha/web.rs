use actix_web::error::{ParseError};
use std::result::Result;
use actix_web::{HttpMessage,http::header};
use actix_web::http::header::TryIntoHeaderValue;
use actix_web::http::header::HeaderValue;
use actix_web::http::header::InvalidHeaderValue;

pub struct XCaptchaToken {
    pub(super) token: String
}

impl TryIntoHeaderValue for XCaptchaToken {
    type Error = InvalidHeaderValue;
    fn try_into_value(self) -> Result<HeaderValue, Self::Error> {
        HeaderValue::from_str(&self.token)
    }
}

impl header::Header for XCaptchaToken {
    fn name() -> header::HeaderName {
        header::HeaderName::from_lowercase(b"x-captcha-token").unwrap()
    }

    fn parse<M>(msg: &M) -> Result<Self, ParseError> where M: HttpMessage {
        let h_name = Self::name();
        let h_value = msg.headers().get(h_name).ok_or(ParseError::Header)?;
        let h_value_str = h_value.to_str().map_err(|_| ParseError::Header)?;
        Ok(XCaptchaToken {
            token: h_value_str.to_owned()
        })
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct CaptchaResponse {
    pub(super) success: bool,
    #[serde(rename="error-codes")]
    pub(super) error_codes: Option<Vec<String>>
}