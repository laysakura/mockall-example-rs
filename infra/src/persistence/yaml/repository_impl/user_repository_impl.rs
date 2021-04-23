use domain::UserRepository;

#[derive(Clone, Hash, Debug, Default)]
pub(crate) struct UserRepositoryImpl;

impl UserRepository for UserRepositoryImpl {
    fn find(&self, id: &domain::UserId) -> domain::MyResult<domain::User> {
        todo!()
    }

    fn create(&self, user: domain::User) -> domain::MyResult<()> {
        todo!()
    }

    fn update(&self, user: domain::User) -> domain::MyResult<()> {
        todo!()
    }
}
