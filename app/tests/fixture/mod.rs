use domain::{EmailAddress, User, UserFirstName, UserId, UserLastName, UserName};

pub trait UserFixture {
    fn fx1() -> User {
        User::new(UserId::new(1), UserName::fx1(), EmailAddress::fx1())
    }
    fn fx2() -> User {
        User::new(UserId::new(2), UserName::fx2(), EmailAddress::fx2())
    }
    fn fx3() -> User {
        // same name with fx2!
        User::new(UserId::new(3), UserName::fx2(), EmailAddress::fx3())
    }
}
impl UserFixture for User {}

pub trait UserNameFixture {
    fn fx1() -> UserName {
        UserName::new(UserFirstName::fx1(), UserLastName::fx1())
    }
    fn fx2() -> UserName {
        UserName::new(UserFirstName::fx2(), UserLastName::fx1())
    }
}
impl UserNameFixture for UserName {}

pub trait UserFirstNameFixture {
    fn fx1() -> UserFirstName {
        UserFirstName::new("FirstName1")
    }
    fn fx2() -> UserFirstName {
        UserFirstName::new("FirstName2")
    }
}
impl UserFirstNameFixture for UserFirstName {}

pub trait UserLastNameFixture {
    fn fx1() -> UserLastName {
        UserLastName::new("LastName1")
    }
}
impl UserLastNameFixture for UserLastName {}

pub trait EmailAddressFixture {
    fn fx1() -> EmailAddress {
        EmailAddress::new("user1@example.com")
    }
    fn fx2() -> EmailAddress {
        EmailAddress::new("user2@example.com")
    }
    fn fx3() -> EmailAddress {
        EmailAddress::new("user3@example.com")
    }
}
impl EmailAddressFixture for EmailAddress {}
