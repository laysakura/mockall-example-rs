use std::cell::RefCell;

use domain::{MyError, MyErrorType, MyResult, User, UserRepository};

#[derive(Clone, Debug)]
pub(crate) struct UserRepositoryImpl {
    // UserRepository の関数は (`&mut self` ではなく) `&self` を取るため、
    // 更新処理のために内部可変性 (RefCell) を使う。
    users: RefCell<Vec<User>>,
}

impl Default for UserRepositoryImpl {
    fn default() -> Self {
        todo!()
    }
}

impl UserRepository for UserRepositoryImpl {
    fn find(&self, id: &domain::UserId) -> MyResult<User> {
        todo!()
    }

    fn create(&self, user: User) -> MyResult<()> {
        if self
            .users
            .borrow()
            .iter()
            .any(|u| u.id() == user.id() || u.email() == user.email())
        {
            Err(MyError::new(
                MyErrorType::Duplicate,
                format!("Duplicate user: {:?}", user),
            ))
        } else {
            self.users.borrow_mut().push(user);
            Ok(())
        }
    }

    fn update(&self, user: User) -> MyResult<()> {
        todo!()
    }
}
