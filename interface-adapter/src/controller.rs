pub mod dto;

use app::UseCase;
use domain::{MyResult, Repositories};

use self::dto::{
    AddUserRequestDTO, AddUserResponseDTO, SearchUsersRequestDTO, SearchUsersResponseDTO,
    UpdateUserRequestDTO, UpdateUserResponseDTO,
};

pub struct Controller<'r, R: Repositories> {
    use_case: UseCase<'r, R>,
}

impl<'r, R: Repositories> Controller<'r, R> {
    pub fn new(repositories: &'r R) -> Self {
        let use_case = UseCase::new(repositories);
        Self { use_case }
    }

    pub fn search_users(&self, dto: SearchUsersRequestDTO) -> SearchUsersResponseDTO {
        let users = self.use_case.search_users(
            dto.first_name.as_ref(),
            dto.last_name.as_ref(),
            dto.email.as_ref(),
        );
        SearchUsersResponseDTO { users }
    }

    pub fn add_user(&self, dto: AddUserRequestDTO) -> MyResult<AddUserResponseDTO> {
        let user = dto.user;
        self.use_case.add_user(user).map(|()| AddUserResponseDTO {})
    }

    pub fn update_user(&self, dto: UpdateUserRequestDTO) -> MyResult<UpdateUserResponseDTO> {
        self.use_case
            .update_user_by_email(&dto.email, dto.first_name, dto.last_name)
            .map(|()| UpdateUserResponseDTO {})
    }
}
