use sha2::{Sha256, Digest}; // Import SHA-256 hashing
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    id: String,
    username: String,
    password_hash: String,
    email: String,
}

struct UserService {
    users: Rc<RefCell<HashMap<String, User>>>,
}

impl UserService {
    fn new() -> Self {
        UserService {
            users: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    // Hash the password using SHA-256
    fn hash_password(password: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(password);
        format!("{:x}", hasher.finalize())
    }

    // Verify the password against the stored hash
    fn verify_password(password: &str, password_hash: &str) -> bool {
        let hashed_input = Self::hash_password(password);
        hashed_input == password_hash
    }

    // Use a sequential counter to generate unique user IDs
    fn generate_user_id(&self) -> String {
        let users = self.users.borrow();
        format!("user_{}", users.len() + 1)
    }

    fn create_user(&self, username: &str, email: &str, password: &str) -> Result<String, &'static str> {
        let user_id = self.generate_user_id(); // Generate a simple sequential user ID
        let password_hash = Self::hash_password(password); // Hash the password

        let user = User {
            id: user_id.clone(),
            username: username.to_string(),
            password_hash,
            email: email.to_string(),
        };

        let mut users = self.users.borrow_mut(); // Use RefCell to get a mutable reference
        if users.insert(user_id.clone(), user).is_some() {
            Err("User already exists")
        } else {
            Ok(user_id)
        }
    }

    fn delete_user(&self, user_id: &str) -> Result<(), &'static str> {
        let mut users = self.users.borrow_mut(); // Use RefCell to get a mutable reference
        if users.remove(user_id).is_none() {
            Err("User not found")
        } else {
            Ok(())
        }
    }

    fn authenticate_user(&self, username: &str, password: &str) -> Result<String, &'static str> {
        let users = self.users.borrow(); // Use RefCell to get an immutable reference
        for user in users.values() {
            if user.username == username {
                if Self::verify_password(password, &user.password_hash) {
                    return Ok(user.id.clone());
                } else {
                    return Err("Invalid password");
                }
            }
        }
        Err("User not found")
    }

    fn reset_password(&self, user_id: &str, new_password: &str) -> Result<(), &'static str> {
        let mut users = self.users.borrow_mut(); // Use RefCell to get a mutable reference
        if let Some(user) = users.get_mut(user_id) {
            user.password_hash = Self::hash_password(new_password); // Hash the new password
            Ok(())
        } else {
            Err("User not found")
        }
    }

    fn get_user_info(&self, user_id: &str) -> Result<User, &'static str> {
        let users = self.users.borrow(); // Use RefCell to get an immutable reference
        if let Some(user) = users.get(user_id) {
            Ok(user.clone())
        } else {
            Err("User not found")
        }
    }
}

fn main() {
    let user_service = UserService::new();

    // Example usage:
    let user_id = user_service.create_user("alice", "alice@example.com", "password123").unwrap();
    println!("User created with ID: {}", user_id);

    let auth_result = user_service.authenticate_user("alice", "password123").unwrap();
    println!("User authenticated with ID: {}", auth_result);

    let user_info = user_service.get_user_info(&auth_result).unwrap();
    println!("User info: {:?}", user_info);

    user_service.reset_password(&auth_result, "newpassword123").unwrap();
    println!("Password reset successful.");

    user_service.delete_user(&auth_result).unwrap();
    println!("User deleted successfully.");
}
