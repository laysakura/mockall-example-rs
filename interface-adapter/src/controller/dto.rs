use domain::{EmailAddress, User, UserFirstName, UserLastName};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct SearchUsersRequestDTO {
    pub email: Option<EmailAddress>,
    pub first_name: Option<UserFirstName>,
    pub last_name: Option<UserLastName>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct SearchUsersResponseDTO {
    pub users: Vec<User>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AddUserRequestDTO {
    pub user: User,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct AddUserResponseDTO;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct UpdateUserRequestDTO {
    pub email: EmailAddress,
    pub first_name: Option<UserFirstName>,
    pub last_name: Option<UserLastName>,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct UpdateUserResponseDTO;
