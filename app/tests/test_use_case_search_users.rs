mod fixture;
mod test_repositories;

use app::UseCase;
use domain::{EmailAddress, MockUserRepository, User, UserFirstName, UserLastName};
use fixture::{EmailAddressFixture, UserFirstNameFixture, UserFixture};
use test_repositories::TestRepositories;

#[test]
fn test_with_blank_repository() {
    let mut user_repo = MockUserRepository::new();
    user_repo.expect_list().returning(|| vec![]);

    let repositories = TestRepositories::new(user_repo);
    let use_case = UseCase::new(&repositories);

    assert_eq!(use_case.search_users(None, None, None), vec![]);
    assert_eq!(
        use_case.search_users(Some(&UserFirstName::new("a")), None, None),
        vec![]
    );
    assert_eq!(
        use_case.search_users(None, Some(&UserLastName::new("a")), None),
        vec![]
    );
    assert_eq!(
        use_case.search_users(None, None, Some(&EmailAddress::new("a@b"))),
        vec![]
    );
}

#[test]
fn test_with_3users_repository() {
    let mut user_repo = MockUserRepository::new();
    user_repo
        .expect_list()
        .returning(|| vec![User::fx1(), User::fx2(), User::fx3()]);

    let repositories = TestRepositories::new(user_repo);
    let use_case = UseCase::new(&repositories);

    assert_eq!(use_case.search_users(None, None, None), vec![]);
    assert_eq!(
        use_case.search_users(Some(&UserFirstName::fx1()), None, None),
        vec![User::fx1()]
    );
    assert_eq!(
        use_case.search_users(Some(&UserFirstName::fx2()), None, None),
        vec![User::fx2(), User::fx3()]
    );
    assert_eq!(
        use_case.search_users(None, None, Some(&EmailAddress::fx1())),
        vec![User::fx1()]
    );
    assert_eq!(
        use_case.search_users(
            Some(&UserFirstName::fx2()),
            None,
            Some(&EmailAddress::fx2())
        ),
        vec![User::fx2()]
    );
}
