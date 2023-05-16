use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, encode, Header, EncodingKey};
use rocket::http::{CookieJar, Cookie};

use crate::{service::user_service::UserService, controller::error_messages::make_wrong_credentials_error, model::credentials::{self, Credentials}};

use super::claims::Claims;

#[derive(Clone, Copy)]
pub struct AuthenticationService {
    user_service: UserService,
}

impl AuthenticationService {
    pub fn new(user_service: UserService) -> Self {
        Self { user_service }
    }

    pub async fn authenticate(self, cookie_jar: &CookieJar<'_>, credentials: Credentials) -> Option<String> {
        let user = self.user_service.get_user_by_name(credentials.username);

        if user.is_none() {
            return None;
        }

        if user.clone().unwrap().password != credentials.password {

        }

        let user_id = user.unwrap().id;

        let token = self.encode_token(Claims { user_id: user_id.clone() });
        cookie_jar.add(Cookie::new("token", token));

        return Some(user_id);
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

    fn encode_token(self, claims: Claims) -> String {
        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())).unwrap();
        return token;
    }

    fn decode_token(self, token: &str) -> jsonwebtoken::errors::Result<jsonwebtoken::TokenData<Claims>> {
        let token_data = decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::new(Algorithm::HS256));
        return token_data;
    }

    fn hash_password(self, password: String) -> String {
        return password;
    }
}