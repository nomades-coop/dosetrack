use hyper::{
    Body as HyperBody, Client as HyperClient, Error as HyperError, Method, Request as HyperRequest,
};
use hyper_tls::HttpsConnector;
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest};
use rocket::response::Debug;
use rocket::Request;
use rocket_oauth2::TokenResponse;
use serde::Deserialize;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserSession {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        request
            .cookies()
            .get_private("user_id")
            .and_then(|c| c.value().parse().ok())
            .map(|id| UserSession(id))
            .or_forward(())
    }
}

pub async fn authenticate(
    token: TokenResponse<GoogleUserInfo>,
) -> Result<String, Debug<HyperError>> {
    let https = HttpsConnector::new();
    let client = HyperClient::builder().build::<_, hyper::Body>(https);
    // Use the token to retrieve the user's Google account information.
    let req = HyperRequest::builder()
        .method(Method::GET)
        .uri("https://www.googleapis.com/oauth2/v3/userinfo")
        .header(
            "Authorization",
            format!("Bearer {}", token.access_token().to_string()),
        )
        .header("Accept", "application/json")
        .body(HyperBody::empty())
        .unwrap();

    let resp = client.request(req).await?;
    let body = hyper::body::to_bytes(resp.into_body()).await?;
    let user: GoogleAuthUser = serde_json::from_slice(&body).unwrap();

    Ok(user.sub)
}

#[derive(serde::Deserialize)]
pub struct GoogleUserInfo;

#[derive(Debug)]
pub struct UserSession(pub String);

#[derive(Deserialize, Debug)]
struct GoogleAuthUser {
    sub: String, // Google ID
    name: String,
    given_name: String,
    family_name: String,
    picture: String,
}
