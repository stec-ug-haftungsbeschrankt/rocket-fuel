
trait UserInfo {

}


trait Auth {
    fn login(oauth2: OAuth2<UserInfo>, cookies: &CookieJar<'_>) -> Redirect;

    fn logout(oauth2: OAuth2<UserInfo>, cookies: &CookieJar<'_>) -> Redirect;

    fn callback(token: TokenResponse<UserInfo>, cookies: &CookieJar<'_>) -> Result<Redirect, Debug<Error>>;

    fn register(oauth2: OAuth2<UserInfo>, cookies: &CookieJar<'_>) -> Redirect;
}