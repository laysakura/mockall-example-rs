use domain::{
    EmailAddress, MyResult, Repositories, User, UserFirstName, UserLastName, UserRepository,
};

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

    pub fn search_users(
        &self,
        first_name: Option<&UserFirstName>,
        last_name: Option<&UserLastName>,
        email: Option<&EmailAddress>,
    ) -> Vec<User> {
        fn name_eq(a: &str, b: &str) -> bool {
            a.to_lowercase() == b.to_lowercase()
        }

        if first_name.is_none() && last_name.is_none() && email.is_none() {
            vec![]
        } else {
            let users = self.user_repo.list();
            let users = users
                .into_iter()
                .filter(|user| {
                    first_name
                        .map(|f_name| {
                            name_eq(&f_name.to_string(), &user.name().first_name().to_string())
                        })
                        .unwrap_or_else(|| true)
                        && last_name
                            .map(|l_name| {
                                name_eq(&l_name.to_string(), &user.name().last_name().to_string())
                            })
                            .unwrap_or_else(|| true)
                        && email.map(|em| em == user.email()).unwrap_or_else(|| true)
                })
                .collect();
            users
        }
    }

    pub fn add_user(&self, user: User) -> MyResult<()> {
        self.user_repo.create(user)
    }
}
