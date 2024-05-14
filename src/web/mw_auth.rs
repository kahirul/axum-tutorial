use crate::ctx::Ctx;
use crate::error::{Error, Result};
use crate::web::AUTH_TOKEN;
use async_trait::async_trait;
use axum::body::{Body, HttpBody};
use axum::extract::{FromRequestParts, Request};
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;
use axum::RequestPartsExt;
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

pub async fn mw_require_auth(ctx: Result<Ctx>, request: Request, next: Next) -> Result<Response> {
    println!("->> {:<16} - mw_require_auth", "middleware");

    ctx?;

    Ok(next.run(request).await)
}

pub async fn mw_ctx_resolver(
    cookies: Cookies,
    mut request: Request,
    next: Next,
) -> Result<Response> {
    println!("->> {:<16} - mw_ctx_resolver", "middleware");
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    let result_ctx = match auth_token
        .ok_or(Error::AuthTokenCookieNoFound)
        .and_then(parse_token)
    {
        Ok((user_id, exp, sign)) => {
            // TODO: Token components validation
            Ok(Ctx::new(user_id))
        }
        Err(e) => Err(e),
    };

    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthTokenCookieNoFound)) {
        cookies.remove(Cookie::from(AUTH_TOKEN));
    }

    // Unique by type, will return existing entry with the same type
    request.extensions_mut().insert(result_ctx);

    Ok(next.run(request).await)
}

#[async_trait]
impl<S: Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<16} - Ctx", "extractor");

        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::AuthCtxNotFound)?
            .clone()
    }
}

fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) =
        regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token).ok_or(Error::AuthTokenInvalid)?;

    let user_id: u64 = user_id.parse().map_err(|_| Error::AuthTokenInvalid)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}
