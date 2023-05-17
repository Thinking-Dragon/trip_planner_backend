use chrono::Utc;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, encode, Header, EncodingKey};
use rocket::http::{CookieJar, Cookie};
use pwhash::bcrypt;

use crate::{service::user_service::UserService, model::{credentials::Credentials, user::User}};

use super::claims::Claims;

#[derive(Clone, Copy)]
pub struct AuthenticationService {
    user_service: UserService,
}

impl AuthenticationService {
    pub fn new(user_service: UserService) -> Self {
        Self { user_service }
    }

    pub fn make_account(self, user: User) -> bool {
        let username_taken = self.user_service.get_user_by_name(user.clone().name).is_some();
        
        if username_taken {
            return false;
        }

        let hashed_user = self.hash_user(user);
        self.user_service.create_user(hashed_user);

        return true;
    }

    pub async fn authenticate(self, cookie_jar: &CookieJar<'_>, credentials: Credentials) -> Option<String> {
        let user = self.user_service.get_user_by_name(credentials.username);

        if user.is_none() {
            return None;
        }

        if !self.is_password_valid(credentials.password, user.clone().unwrap().password) {
            return None;
        }

        let user_id = user.unwrap().id;

        let exp = Utc::now().checked_add_signed(chrono::Duration::minutes(1)).expect("valid timestamp").timestamp() as usize;
        let token = self.encode_token(Claims { sub: user_id.clone(), exp });
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
        let user_id = claims.sub;

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

    fn hash_user(self, user: User) -> User {
        let mut hashed_user = user.clone();
        hashed_user.password = self.hash_password(hashed_user.password);
        return hashed_user;
    }

    fn hash_password(self, password: String) -> String {
        let hash = bcrypt::hash(password).unwrap();
        return hash;
    }

    fn is_password_valid(self, password: String, hash: String) -> bool {
        return bcrypt::verify(password, &hash);
    }
}