use std::cell::RefCell;

use domain::{MyError, MyErrorType, MyResult, User, UserRepository};

use crate::persistence::yaml::user_yaml_storage::UserYamlStorage;

#[derive(Clone, Debug)]
pub(crate) struct UserRepositoryImpl {
    // UserRepository の関数は (`&mut self` ではなく) `&self` を取るため、
    // 更新処理のために内部可変性 (RefCell) を使う。
    users: RefCell<Vec<User>>,
}

impl Default for UserRepositoryImpl {
    fn default() -> Self {
        let storage = UserYamlStorage::new();
        let users = storage.load();
        Self {
            users: RefCell::new(users),
        }
    }
}

impl UserRepository for UserRepositoryImpl {
    fn list(&self) -> Vec<User> {
        self.users.borrow().clone()
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
            self.save();
            Ok(())
        }
    }

    fn update(&self, user: User) -> MyResult<()> {
        todo!()
    }
}

impl UserRepositoryImpl {
    fn save(&self) {
        let storage = UserYamlStorage::new();
        storage.save(&self.users.borrow())
    }
}
