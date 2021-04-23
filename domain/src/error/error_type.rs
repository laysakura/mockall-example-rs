#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum MyErrorType {
    NotFound,
    Duplicate,
}
