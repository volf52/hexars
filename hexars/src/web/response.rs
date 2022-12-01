use axum::{http::StatusCode, response::IntoResponse};

pub struct HttpError(eyre::Report);
pub type HttpResult<T> = std::result::Result<T, HttpError>;

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        let err = self.0.to_string();

        return (StatusCode::INTERNAL_SERVER_ERROR, err).into_response();
    }
}
//
// This enables using `?` on functions that return `Result<_, eyre::Report>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for HttpError
where
    E: Into<eyre::Report>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
