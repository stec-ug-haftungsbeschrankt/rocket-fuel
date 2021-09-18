use anyhow::{Context, Error};
use reqwest::header::{AUTHORIZATION, USER_AGENT};
use rocket::{http::{Cookie, CookieJar, SameSite}, response::{Debug, Redirect}};
use rocket_oauth2::{OAuth2, TokenResponse};

use crate::auth::User;






/// User information to be retrieved from the Auth0 API.
#[derive(serde::Deserialize)]
pub struct Auth0UserInfo {
    #[serde(default)]
    name: String,
}


#[get("/auth/login/auth0")]
pub fn auth0_login(oauth2: OAuth2<Auth0UserInfo>, cookies: &CookieJar<'_>) -> Redirect {
    oauth2.get_redirect(cookies, &["openid", "profile", "email"]).unwrap()
}

#[get("/auth/auth0")]
pub async fn auth0_callback(token: TokenResponse<Auth0UserInfo>, cookies: &CookieJar<'_>) -> Result<Redirect, Debug<Error>> {
    let user_info: User = reqwest::Client::builder()
        .build()
        .context("failed to build reqwest client")?
        .get("https://dev-3xbzq-01.eu.auth0.com/userinfo")
        .header(AUTHORIZATION, format!("Bearer {}", token.access_token()))
        .header(USER_AGENT, "STEC Monitoring")
        .send()
        .await
        .context("failed to complete request")?
        .json()
        .await
        .context("failed to deserialize response")?;

    let data: String = user_info.into();

    cookies.add_private(
        Cookie::build("user", data)
            .same_site(SameSite::Lax)
            .finish(),
    );
    Ok(Redirect::to("/"))
}