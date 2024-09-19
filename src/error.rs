use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::derive::{Display, Error};

#[derive(Debug, Display, Error)]
pub(crate) enum DavAuthError {
    #[display("Nice try.")]
    DavAuthError,
}

impl error::ResponseError for DavAuthError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(("WWW-Authenticate", "Basic realm=\"User Visible Realm\""))
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            DavAuthError::DavAuthError => StatusCode::UNAUTHORIZED,
        }
    }
}
