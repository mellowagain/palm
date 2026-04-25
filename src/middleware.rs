use base64::{engine::general_purpose::STANDARD, Engine};
use poem::{Endpoint, IntoResponse, Request, Response};
use poem::http::StatusCode;

pub(crate) struct BasicAuth<E> {
    pub(crate) inner: E,
    pub(crate) username: String,
    pub(crate) password: String,
}

impl<E: Endpoint> Endpoint for BasicAuth<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> poem::Result<Response> {
        let authorized = req.headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Basic "))
            .and_then(|encoded| STANDARD.decode(encoded).ok())
            .and_then(|decoded| String::from_utf8(decoded).ok())
            .map(|credentials| {
                let mut parts = credentials.splitn(2, ':');
                let user = parts.next().unwrap_or_default();
                let pass = parts.next().unwrap_or_default();
                user == self.username && pass == self.password
            })
            .unwrap_or(false);

        if authorized {
            self.inner.call(req).await.map(|r| r.into_response())
        } else {
            Ok(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header("WWW-Authenticate", r#"Basic realm="Palm""#)
                .body("Unauthorized"))
        }
    }
}
