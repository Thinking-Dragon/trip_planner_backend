use rocket::{State, http::CookieJar, serde::json::Json};

use crate::{service::authentication::authentication_service::AuthenticationService, model::credentials::Credentials, controller::error_messages::make_wrong_credentials_error};

#[post("/login", format="json", data="<body>")]
pub async fn login(
    authentication_service: &State<AuthenticationService>,
    cookie_jar: &CookieJar<'_>,
    body: Json<Credentials>
) -> String {
    let credentials = body.0;
    let user_id = authentication_service.authenticate(cookie_jar, credentials).await;

    if user_id.is_none() {
        return make_wrong_credentials_error();
    }
    else {
        return format!("{{message:\"Login successful\",user_id:\"{}\"}}", user_id.unwrap());
    }
}