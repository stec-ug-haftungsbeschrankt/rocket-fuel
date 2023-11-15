use async_trait::async_trait;
use rocket::{http::CookieJar, request, response::Debug, response::Redirect};
use rocket_oauth2::{OAuth2, TokenResponse};

mod auth0;
mod github;
mod google;
mod keycloak;
mod microsoft;

trait UserInfo {

}


#[async_trait]
trait Auth<T: UserInfo> {
    fn login(oauth2: OAuth2<T>, cookies: &CookieJar<'_>) -> Redirect;

    fn logout(oauth2: OAuth2<T>, cookies: &CookieJar<'_>) -> Redirect;

    async fn callback(token: TokenResponse<T>, cookies: &CookieJar<'_>) -> Result<Redirect, Debug<anyhow::Error>>;

    fn register(oauth2: OAuth2<T>, cookies: &CookieJar<'_>) -> Redirect;
}


#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct User {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub email: String
}

impl From<String> for User {
    fn from(data: String) -> Self {
        serde_json::from_str(&data).expect("Unable to deserialize User")
    }
}

impl Into<String> for User {
    fn into(self) -> String {
        serde_json::to_string(&self).expect("Unable to serialize User")
    }
}

#[rocket::async_trait]
impl<'r> request::FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r request::Request<'_>) -> request::Outcome<User, ()> {
        let cookies = request
            .guard::<&CookieJar<'_>>()
            .await
            .expect("request cookies");
        if let Some(cookie) = cookies.get_private("user") {
            return request::Outcome::Success(User::from(cookie.value().to_string()));
        }
        request::Outcome::Forward(rocket::http::Status { code: 200 })
    }
}

