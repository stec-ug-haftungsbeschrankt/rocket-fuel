use anyhow::Context;
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use rocket::{http::{Cookie, CookieJar, SameSite}, response::{Debug, Redirect}};
use rocket_oauth2::{OAuth2, TokenResponse};

use crate::auth::User;

use super::{Auth, UserInfo};



/// User information to be retrieved from the GitHub API.
#[derive(serde::Deserialize)]
pub struct GitHubUserInfo {
    #[serde(default)]
    name: String,
    #[serde(default)]
    email: String
}

impl UserInfo for GitHubUserInfo {

}


struct GitHub { }


#[async_trait]
impl Auth<GitHubUserInfo> for GitHub {
    fn login(oauth2: OAuth2<GitHubUserInfo>, cookies: &CookieJar<'_>) -> Redirect {
        oauth2.get_redirect(cookies, &["user:read"]).unwrap()
    }

    fn logout(_oauth2: OAuth2<GitHubUserInfo>, _cookies: &CookieJar<'_>) -> Redirect {
        todo!()
    }

    async fn callback(token: TokenResponse<GitHubUserInfo>, cookies: &CookieJar<'_>) -> Result<Redirect, Debug<anyhow::Error>> {
        // Use the token to retrieve the user's GitHub account information.
        let user_info: User = reqwest::Client::builder()
            .build()
            .context("failed to build reqwest client")?
            .get("https://api.github.com/user")
            .header(AUTHORIZATION, format!("token {}", token.access_token()))
            .header(ACCEPT, "application/vnd.github.v3+json")
            .header(USER_AGENT, "rocket_oauth2 demo application")
            .send()
            .await
            .context("failed to complete request")?
            .json()
            .await
            .context("failed to deserialize response")?;

        let data: String = user_info.into();

        // Set a private cookie with the user's name, and redirect to the home page.
        cookies.add_private(
            Cookie::build(("user", data))
                .same_site(SameSite::Lax)
        );
        Ok(Redirect::to("/"))
    }

    fn register(_oauth2: OAuth2<GitHubUserInfo>, _cookies: &CookieJar<'_>) -> Redirect {
        todo!()
    }
}
