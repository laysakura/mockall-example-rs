use domain::MyResult;

use self::dto::{
    AddUserRequestDTO, AddUserResponseDTO, SearchUsersRequestDTO, SearchUsersResponseDTO,
    UpdateUserRequestDTO, UpdateUserResponseDTO,
};

mod dto;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Controller;

impl Controller {
    pub fn search_users(dto: SearchUsersRequestDTO) -> MyResult<SearchUsersResponseDTO> {
        todo!()
    }

    pub fn add_user(dto: AddUserRequestDTO) -> MyResult<AddUserResponseDTO> {
        todo!()
    }

    pub fn update_user(dto: UpdateUserRequestDTO) -> MyResult<UpdateUserResponseDTO> {
        todo!()
    }
}
