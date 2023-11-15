use anyhow::Context;
use reqwest::header::AUTHORIZATION;
use rocket::{http::{Cookie, CookieJar, SameSite}, response::{Debug, Redirect}};
use rocket_oauth2::{OAuth2, TokenResponse};
use serde_json::Value;


use super::{Auth, UserInfo};


/// User information to be retrieved from the Google People API.
#[derive(serde::Deserialize)]
pub struct GoogleUserInfo {
    names: Vec<Value>,
}


impl UserInfo for GoogleUserInfo {

}


struct Google { }


#[async_trait]
impl Auth<GoogleUserInfo> for Google {

    fn login(oauth2: OAuth2<GoogleUserInfo>, cookies: &CookieJar<'_>) -> Redirect {
        oauth2.get_redirect(cookies, &["profile"]).unwrap()
    }

    fn logout(_oauth2: OAuth2<GoogleUserInfo>, _cookies: &CookieJar<'_>) -> Redirect {
        todo!()
    }

    async fn callback(token: TokenResponse<GoogleUserInfo>, cookies: &CookieJar<'_>) -> Result<Redirect, Debug<anyhow::Error>> {
        // Use the token to retrieve the user's Google account information.
        let user_info: GoogleUserInfo = reqwest::Client::builder()
            .build()
            .context("failed to build reqwest client")?
            .get("https://people.googleapis.com/v1/people/me?personFields=names")
            .header(AUTHORIZATION, format!("Bearer {}", token.access_token()))
            .send()
            .await
            .context("failed to complete request")?
            .json()
            .await
            .context("failed to deserialize response")?;

        let real_name = user_info
            .names
            .first()
            .and_then(|n| n.get("displayName"))
            .and_then(|s| s.as_str())
            .unwrap_or("");

        // Set a private cookie with the user's name, and redirect to the home page.
        cookies.add_private(
            Cookie::build(("username", real_name.to_string()))
                .same_site(SameSite::Lax)
            );
        Ok(Redirect::to("/"))
    }

    fn register(_oauth2: OAuth2<GoogleUserInfo>, _cookies: &CookieJar<'_>) -> Redirect {
        todo!()
    }

}
