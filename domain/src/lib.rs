///! Domain layer
mod email_address;
mod error;
mod user;

pub use email_address::EmailAddress;
pub use error::{error_type::MyErrorType, MyError, MyResult};
pub use user::{user_id::UserId, user_name::UserName, user_repository::UserRepository, User};
