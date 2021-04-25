mod test_repositories;

use app::UseCase;
use domain::{MockUserRepository, UserFirstName};
use test_repositories::TestRepositories;

#[test]
fn test_with_blank_repository() {
    let user_repo = MockUserRepository::new();
    let repositories = TestRepositories::new(user_repo);
    let use_case = UseCase::new(&repositories);

    assert_eq!(use_case.search_users(None, None, None), vec![]);
    assert_eq!(
        use_case.search_users(Some(&UserFirstName::new("aa")), None, None),
        vec![]
    );
}
