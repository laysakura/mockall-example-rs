use domain::UserRepository;

/// Controllerなどの各所で都度同じような型パラメータを定義しないで済むように、リポジトリtraitをこのtraitの関連型としてまとめる。
/// 例えば、 `ARepository` と `BRepository` を両方使う `XController` があった場合、この trait がなければ
///  `XController<A: ARepository, B: BRepository>` と2つの型パラメーターが必要なところ、
/// `XController<R: Repositories>` の1つで済む。
pub trait Repositories {
    type UserRepo: UserRepository;

    fn user_repository(&self) -> Self::UserRepo;
}
