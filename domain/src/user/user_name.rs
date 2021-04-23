pub mod user_first_name;
pub mod user_last_name;

use std::fmt::Display;

use self::{user_first_name::UserFirstName, user_last_name::UserLastName};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct UserName {
    first_name: UserFirstName,
    last_name: UserLastName,
}

impl UserName {
    pub fn new(first_name: UserFirstName, last_name: UserLastName) -> Self {
        Self {
            first_name,
            last_name,
        }
    }
}

impl Display for UserName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.first_name, self.last_name)
    }
}
