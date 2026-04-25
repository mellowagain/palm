use base64::{engine::general_purpose::STANDARD, Engine};
use poem::{Endpoint, IntoResponse, Middleware, Request, Response, Result};
use poem::http::StatusCode;

pub(crate) struct BasicAuth {
    username: String,
    password: String,
}

impl BasicAuth {
    pub(crate) fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

impl<E: Endpoint> Middleware<E> for BasicAuth {
    type Output = BasicAuthEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        BasicAuthEndpoint {
            inner: ep,
            username: self.username.clone(),
            password: self.password.clone(),
        }
    }
}

pub(crate) struct BasicAuthEndpoint<E> {
    inner: E,
    username: String,
    password: String,
}

impl<E: Endpoint> Endpoint for BasicAuthEndpoint<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
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
