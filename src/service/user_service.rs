use crate::model::user::User;

#[derive(Clone, Copy)]
pub struct UserService;

impl UserService {
    pub fn get_users(self) -> Vec<User> {
        let users: Vec<User> = Vec::new();
        return users;
    }

    pub fn get_user(self, user_id: String) -> User {
        let user: User = User {
            id: "".to_string(),
            name: "User".to_string(),
            password: "123".to_string(),
            picture_url: "https://123.com/user/123".to_string()
        };
        return user;
    }

    pub fn get_user_by_name(self, username: String) -> Option<User> {
        let user: User = User {
            id: "".to_string(),
            name: "User".to_string(),
            password: "123".to_string(),
            picture_url: "https://123.com/user/123".to_string()
        };
        return Some(user);
    }

    pub fn create_user(self, user: User) {

    }

    pub fn update_user(self, user_id: String, user: User) {

    }
}