use crate::{MyResult, User};

pub trait UserRepository {
    fn list(&self) -> Vec<User>;

    /// # Failures
    ///
    /// - `MyErrorType::Duplicate` : when user with given ID already exists.
    fn create(&self, user: User) -> MyResult<()>;

    /// # Failures
    ///
    /// - `MyErrorType::NotFound` : when user with given ID (inside User) does not exist.
    fn update(&self, user: User) -> MyResult<()>;
}
