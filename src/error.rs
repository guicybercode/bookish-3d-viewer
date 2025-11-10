use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    ModelLoadError(String),
    ImageLoadError(String),
    ConfigError(String),
    RenderError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::ModelLoadError(msg) => write!(f, "Model load error: {}", msg),
            AppError::ImageLoadError(msg) => write!(f, "Image load error: {}", msg),
            AppError::ConfigError(msg) => write!(f, "Config error: {}", msg),
            AppError::RenderError(msg) => write!(f, "Render error: {}", msg),
        }
    }
}

impl Error for AppError {}

