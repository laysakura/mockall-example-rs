use self::{user_id::UserId, user_name::UserName};
use crate::email_address::EmailAddress;
use serde::{Deserialize, Serialize};

pub mod user_id;
pub mod user_name;
pub mod user_repository;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct User {
    id: UserId,
    name: UserName,
    email: EmailAddress,
}

impl User {
    pub fn new(id: UserId, name: UserName, email: EmailAddress) -> Self {
        Self { id, name, email }
    }

    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn name(&self) -> &UserName {
        &self.name
    }

    pub fn email(&self) -> &EmailAddress {
        &self.email
    }
}
