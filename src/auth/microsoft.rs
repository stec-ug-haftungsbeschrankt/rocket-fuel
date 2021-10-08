use anyhow::{Context};
use reqwest::header::{AUTHORIZATION};
use rocket::{http::{Cookie, CookieJar, SameSite}, response::{Debug, Redirect}};
use rocket_oauth2::{OAuth2, TokenResponse};

use super::{Auth, UserInfo};


/// User information to be retrieved from the Microsoft API.
#[derive(serde::Deserialize)]
pub struct MicrosoftUserInfo {
    #[serde(default, rename = "displayName")]
    display_name: String,
}


impl UserInfo for MicrosoftUserInfo {

}


struct Microsoft { }


#[async_trait]
impl Auth<MicrosoftUserInfo> for Microsoft {
    fn login(oauth2: OAuth2<MicrosoftUserInfo>, cookies: &CookieJar<'_>) -> Redirect {
        oauth2.get_redirect(cookies, &["user.read"]).unwrap()
    }

    fn logout(_oauth2: OAuth2<MicrosoftUserInfo>, _cookies: &CookieJar<'_>) -> Redirect {
        todo!()
    }

    async fn callback(token: TokenResponse<MicrosoftUserInfo>, cookies: &CookieJar<'_>) -> Result<Redirect, Debug<anyhow::Error>> {
        // Use the token to retrieve the user's Microsoft account information.
        let user_info: MicrosoftUserInfo = reqwest::Client::builder()
            .build()
            .context("failed to build reqwest client")?
            .get("https://graph.microsoft.com/v1.0/me")
            .header(AUTHORIZATION, format!("Bearer {}", token.access_token()))
            .send()
            .await
            .context("failed to complete request")?
            .json()
            .await
            .context("failed to deserialize response")?;

        // Set a private cookie with the user's name, and redirect to the home page.
        cookies.add_private(
            Cookie::build("username", user_info.display_name)
                .same_site(SameSite::Lax)
                .finish(),
        );
        Ok(Redirect::to("/"))
    }

    fn register(_oauth2: OAuth2<MicrosoftUserInfo>, _cookies: &CookieJar<'_>) -> Redirect {
        todo!()
    }

}

