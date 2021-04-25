use domain::{
    EmailAddress, MyError, MyErrorType, MyResult, Repositories, User, UserFirstName, UserLastName,
    UserName, UserRepository,
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

    pub fn update_user_by_email(
        &self,
        email: &EmailAddress,
        first_name: Option<UserFirstName>,
        last_name: Option<UserLastName>,
    ) -> MyResult<()> {
        let users = self.user_repo.list();

        let user = users
            .into_iter()
            .find(|user| user.email() == email)
            .ok_or_else(|| {
                MyError::new(
                    MyErrorType::NotFound,
                    format!("User with email address `{}` does not exist", email),
                )
            })?;

        let new_user_name = UserName::new(
            first_name.unwrap_or_else(|| user.name().first_name().clone()),
            last_name.unwrap_or_else(|| user.name().last_name().clone()),
        );
        let new_user = User::new(user.id().clone(), new_user_name, user.email().clone());

        self.user_repo.update(new_user)
    }
}
