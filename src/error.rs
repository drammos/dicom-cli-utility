use dicom::core::value::ConvertValueError;
use std::sync::{Arc, Mutex};
pub type Result<T> = std::result::Result<T, AppError>;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Value not found")]
    NotFound,
    #[error("It's not a valid DICOM file {0}")]
    ReadError(#[from] dicom::object::ReadError),
    #[error("Access error {0}")]
    AccessError(#[from] dicom::object::AccessError),
    #[error("Error converting value {0}")]
    ConvertValueError(#[from] ConvertValueError),
    #[error("Error accessing by name {0}")]
    AccessByNameError(#[from] dicom::object::AccessByNameError),
}

impl AppError {
    // Handle the errors for AppError
    pub fn handle_error(&self, path: &std::path::Path, console_lock: Arc<Mutex<()>>) {
        match self {
            AppError::ReadError(_) => {
                let _lock = console_lock.lock().unwrap();
                eprintln!("Warning: {} is not a valid DICOM file", path.display());
            }
            AppError::AccessError(_) => {
                let _lock = console_lock.lock().unwrap();
                eprintln!("Access error: , in the file {}", path.display());
            }
            AppError::ConvertValueError(_) => {
                let _lock = console_lock.lock().unwrap();
                eprintln!("Error converting value: , in the file {}", path.display());
            }
            AppError::AccessByNameError(_) => {
                let _lock = console_lock.lock().unwrap();
                eprintln!("Error accessing by name: , in the file {}", path.display());
            }
            AppError::NotFound => {
                let _lock = console_lock.lock().unwrap();
                eprintln!("Item not found in DICOM file.");
            }
        }
    }
}
