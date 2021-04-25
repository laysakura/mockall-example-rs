use domain::{MockUserRepository, Repositories};

pub struct TestRepositories {
    user_repo: MockUserRepository,
}

impl Repositories for TestRepositories {
    type UserRepo = MockUserRepository;

    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repo
    }
}

impl TestRepositories {
    pub fn new(user_repo: MockUserRepository) -> Self {
        Self { user_repo }
    }
}
