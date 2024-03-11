pub struct RestError(anyhow::Error);

impl From<anyhow::Error> for RestError {
  fn from(error: anyhow::Error) -> Self {
    Self(error)
  }
}

impl axum::response::IntoResponse for RestError {
  fn into_response(self) -> axum::response::Response {
    let status_code = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
    // let status_code = if let Some(error) = self.0.downcast_ref::<ApiError>() {
    //   match error {
    //     ApiError::NotFound => axum::http::StatusCode::NOT_FOUND,
    //     ApiError::BadRequest => axum::http::StatusCode::BAD_REQUEST,
    //   }
    // } else {
    //   axum::http::StatusCode::INTERNAL_SERVER_ERROR
    // };
    (status_code, self.0.to_string()).into_response()
  }
}
