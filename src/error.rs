use dicom::core::value::ConvertValueError;
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
    pub fn handle_error(&self, path: &std::path::Path) {
        match self {
            AppError::ReadError(_) => {
                eprintln!("Warning: {} is not a valid DICOM file", path.display());
            }
            AppError::AccessError(e) => {
                eprintln!("Access error: {:?}", e);
            }
            AppError::ConvertValueError(e) => {
                eprintln!("Error converting value: {:?}", e);
            }
            AppError::AccessByNameError(e) => {
                eprintln!("Error accessing by name: {:?}", e);
            }
            AppError::NotFound => {
                eprintln!("Item not found in DICOM file.");
            }
        }
    }
}
