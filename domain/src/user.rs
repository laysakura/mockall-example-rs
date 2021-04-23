use crate::email_address::EmailAddress;

use self::{user_id::UserId, user_name::UserName};

pub mod user_id;
pub mod user_name;
pub mod user_repository;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct User {
    id: UserId,
    name: UserName,
    email: EmailAddress,
}

impl User {
    pub fn new(id: UserId, name: UserName, email: EmailAddress) -> Self {
        Self { id, name, email }
    }
}
