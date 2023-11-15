use anyhow::Context;
use reqwest::header::{AUTHORIZATION, USER_AGENT};
use rocket::{http::{Cookie, CookieJar, SameSite}, response::{Debug, Redirect}};
use rocket_oauth2::{OAuth2, TokenResponse};

use crate::auth::User;
use super::{Auth, UserInfo};


/// User information to be retrieved from the Keycloak API.
#[derive(serde::Deserialize)]
pub struct KeycloakUserInfo {
}


impl UserInfo for KeycloakUserInfo {

}


struct Keycloak { }


#[async_trait]
impl Auth<KeycloakUserInfo> for Keycloak {

    fn login(oauth2: OAuth2<KeycloakUserInfo>, cookies: &CookieJar<'_>) -> Redirect {
        oauth2.get_redirect(cookies, &["openid", "profile", "email"]).unwrap()
    }

    fn logout(_oauth2: OAuth2<KeycloakUserInfo>, cookies: &CookieJar<'_>) -> Redirect {
        cookies.remove(Cookie::build("user"));
        // Redirect::to("/") FIXME Where does the redirect go
        Redirect::to("https://auth.stecgmbh.de/auth/realms/stec/protocol/openid-connect/logout")
    }

    async fn callback(token: TokenResponse<KeycloakUserInfo>, cookies: &CookieJar<'_>) -> Result<Redirect, Debug<anyhow::Error>> {
        let user_info: User = reqwest::Client::builder()
            .build()
            .context("failed to build reqwest client")?
            .get("https://auth.stecgmbh.de/auth/realms/stec/protocol/openid-connect/userinfo")
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
            Cookie::build(("user", data))
                .same_site(SameSite::Lax)
        );
        Ok(Redirect::to("/"))
    }

    fn register(_oauth2: OAuth2<KeycloakUserInfo>, _cookies: &CookieJar<'_>) -> Redirect {
        //Redirect::to("https://auth.stecgmbh.de/auth/realms/stec/protocol/openid-connect/register")
        //oauth2.get_redirect(cookies, &["openid", "email"]).unwrap()

        // FIXME Extend JebRosens auth crate for registration page
        Redirect::to("https://auth.stecgmbh.de/auth/realms/stec/protocol/openid-connect/registrations?client_id=stec_general&response_type=code&scope=openid%20email&redirect_uri=http://localhost:8000/auth/keycloak&kc_locale=en")

        // See https://stackoverflow.com/questions/51514437/keycloak-direct-user-link-registration
        // http://<domain.com>/auth/realms/<realm-name>/protocol/openid-connect/registrations?client_id=<client_id>&response_type=code&scope=openid email&redirect_uri=http://<domain.com>/<redirect-path>&kc_locale=<two-digit-lang-code>
    }
}
