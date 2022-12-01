use thiserror::Error;

#[derive(Debug, Error)]
pub enum MultipartError {
    #[error("No named field in multipart")]
    NoName,
    #[error("Invalid value in multipart")]
    InvalidValue,
    #[error("Reading multipart error")]
    ReadError,
}

#[derive(Debug, Error)]
pub enum SignupError {
    #[error("Username already exists")]
    UsernameExists,
    #[error("Invalid Username")]
    InvalidUsername,
    #[error("Passswords do not match")]
    PasswordsDoNotMatch,
    #[error("Missing Details")]
    MissingDetails,
    #[error("Invalid Password")]
    InvalidPassword,
    #[error("Internal Error")]
    InternalError,
}

#[derive(Debug, Error)]
pub enum LoginError {
    #[error("Missing details")]
    MissingDetails,
    #[error("User does not exist")]
    UserDoesNotExist,
    #[error("Wrong password")]
    WrongPassword,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error(".env file not found")]
    NoEnvFile(#[from] std::io::Error),
    #[error("DATABASE_URL not present in .env")]
    DatabaseUrlNotFound,
    #[error("Invalid Database Url")]
    InvalidDatabaseUrl,
    #[error("Already initialized")]
    AlreadyInitialized,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    ConfigError(#[from] ConfigError),
    #[error("could not find user {0}")]
    NoUser(String),
    #[error("Not logged in")]
    NotLoggedIn,
    #[error(transparent)]
    LoginError(#[from] LoginError),
    #[error(transparent)]
    SignupError(#[from] SignupError),
    #[error(transparent)]
    MultipartError(#[from] MultipartError),
}
