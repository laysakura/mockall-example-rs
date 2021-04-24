pub mod dto;

use domain::{MyResult, UserRepository};

use crate::Repositories;

use self::dto::{
    AddUserRequestDTO, AddUserResponseDTO, SearchUsersRequestDTO, SearchUsersResponseDTO,
    UpdateUserRequestDTO, UpdateUserResponseDTO,
};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Controller<R: Repositories> {
    user_repo: R::UserRepo,
}

impl<R: Repositories> Controller<R> {
    pub fn new(repositories: &R) -> Self {
        Self {
            user_repo: repositories.user_repository(),
        }
    }

    pub fn search_users(&self, dto: SearchUsersRequestDTO) -> SearchUsersResponseDTO {
        todo!()
    }

    pub fn add_user(&self, dto: AddUserRequestDTO) -> MyResult<AddUserResponseDTO> {
        // FIXME usercase使う
        let user = dto.user;
        self.user_repo.create(user).map(|()| AddUserResponseDTO {})
    }

    pub fn update_user(&self, dto: UpdateUserRequestDTO) -> MyResult<UpdateUserResponseDTO> {
        todo!()
    }
}
