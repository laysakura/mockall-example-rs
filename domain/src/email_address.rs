use std::fmt::Display;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct EmailAddress {
    user: String,
    domain: String,
}

impl EmailAddress {
    pub fn new(user: impl ToString, domain: impl ToString) -> Self {
        // TODO 本当はここでメアドのRFCに沿ってるかなどバリデーションすべき

        Self {
            user: user.to_string(),
            domain: domain.to_string(),
        }
    }
}

impl Display for EmailAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}", self.user, self.domain)
    }
}
