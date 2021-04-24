use domain::{MyResult, Repositories, User, UserRepository};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct UseCase<R: Repositories> {
    user_repo: R::UserRepo,
}

impl<R: Repositories> UseCase<R> {
    pub fn new(repositories: &R) -> Self {
        Self {
            user_repo: repositories.user_repository(),
        }
    }

    pub fn add_user(&self, user: User) -> MyResult<()> {
        self.user_repo.create(user)
    }
}
