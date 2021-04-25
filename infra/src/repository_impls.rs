pub(crate) mod user_repository_impl;

use domain::Repositories;

use self::user_repository_impl::UserRepositoryImpl;

/// リポジトリのインスタンスを詰め込む。
/// リポジトリtraitの具体型を決定する、静的なDI (Dependency Injection) をする場でもある。
#[derive(Clone, Debug, Default)]
pub(crate) struct RepositoryImpls {
    user_repo: UserRepositoryImpl,
}

impl Repositories for RepositoryImpls {
    type UserRepo = UserRepositoryImpl;

    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repo
    }
}
