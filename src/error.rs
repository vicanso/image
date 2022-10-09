use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct HTTPError {
    pub message: String,
    pub category: String,
    pub status: u16,
}

impl Default for HTTPError {
    fn default() -> Self {
        HTTPError {
            message: "".to_string(),
            category: "".to_string(),
            status: 400,
        }
    }
}
impl IntoResponse for HTTPError {
    fn into_response(self) -> Response {
        let status = match StatusCode::from_u16(self.status) {
            Ok(status) => status,
            Err(_) => StatusCode::BAD_REQUEST,
        };
        (status, Json(self)).into_response()
    }
}
impl From<ImageError> for HTTPError {
    fn from(error: ImageError) -> Self {
        HTTPError {
            message: error.message,
            category: error.category,
            ..Default::default()
        }
    }
}
impl From<base64::DecodeError> for HTTPError {
    fn from(error: base64::DecodeError) -> Self {
        HTTPError {
            message: error.to_string(),
            category: "base64".to_string(),
            ..Default::default()
        }
    }
}
impl From<reqwest::Error> for HTTPError {
    fn from(error: reqwest::Error) -> Self {
        HTTPError {
            message: error.to_string(),
            category: "reqwest".to_string(),
            ..Default::default()
        }
    }
}
impl From<reqwest::header::ToStrError> for HTTPError {
    fn from(error: reqwest::header::ToStrError) -> Self {
        HTTPError {
            message: error.to_string(),
            category: "reqwest".to_string(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ImageError {
    pub message: String,
    pub category: String,
}

impl From<imagequant::Error> for ImageError {
    fn from(error: imagequant::Error) -> Self {
        ImageError {
            message: error.to_string(),
            category: "imagequant".to_string(),
        }
    }
}
impl From<lodepng::Error> for ImageError {
    fn from(error: lodepng::Error) -> Self {
        ImageError {
            message: error.to_string(),
            category: "lodepng".to_string(),
        }
    }
}
impl From<image::ImageError> for ImageError {
    fn from(error: image::ImageError) -> Self {
        ImageError {
            message: error.to_string(),
            category: "image".to_string(),
        }
    }
}
impl From<std::string::String> for ImageError {
    fn from(message: std::string::String) -> Self {
        ImageError {
            message,
            category: "unknown".to_string(),
        }
    }
}
