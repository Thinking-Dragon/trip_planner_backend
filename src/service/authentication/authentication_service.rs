use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use rocket::http::CookieJar;

use crate::service::user_service::UserService;

use super::claims::Claims;

#[derive(Clone, Copy)]
pub struct AuthenticationService {
    user_service: UserService,
}

impl AuthenticationService {
    pub fn new(user_service: UserService) -> Self {
        Self { user_service }
    }

    pub async fn validate_authentication(self, cookie_jar: &CookieJar<'_>) -> Option<String> {
        let token_cookie = cookie_jar.get("token");

        if token_cookie.is_none() {
            return None;
        }

        let token = token_cookie.unwrap().value();
        let decoding_result = self.decode_token(token);

        if decoding_result.is_err() {
            return None;
        }

        let claims = decoding_result.unwrap().claims;
        let user_id = claims.user_id;

        return Some(user_id);
    }

    fn decode_token(self, token: &str) -> jsonwebtoken::errors::Result<jsonwebtoken::TokenData<Claims>> {
        let token_message = decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::new(Algorithm::HS256));
        return token_message;
    }
}