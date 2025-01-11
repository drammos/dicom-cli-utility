use dicom::core::value::ConvertValueError;
pub type Result<T> = std::result::Result<T, AppError>;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Value not found")]
    NotFound,
    #[error("Not found {0}")]
    ReadError(#[from] dicom::object::ReadError),
    #[error("Not found {0}")]
    AccessError(#[from] dicom::object::AccessError),
    #[error("Not found {0}")]
    ConvertValueError(#[from] ConvertValueError),
    #[error("Not found {0}")]
    AccessByNameError(#[from] dicom::object::AccessByNameError),
}
