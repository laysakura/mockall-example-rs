use crate::{MyResult, User};

use super::user_id::UserId;

pub trait UserRepository {
    /// # Failures
    ///
    /// - `MyErrorType::NotFound` : when user with given ID does not exist.
    fn find(&self, id: &UserId) -> MyResult<User>;

    /// # Failures
    ///
    /// - `MyErrorType::Duplicate` : when user with given ID already exists.
    fn create(&self, user: User) -> MyResult<()>;

    /// # Failures
    ///
    /// - `MyErrorType::NotFound` : when user with given ID (inside User) does not exist.
    fn update(&self, user: User) -> MyResult<()>;
}
