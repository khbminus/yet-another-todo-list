use crate::domain::UserName;
use crate::domain::UserPassword;

pub struct NewUser {
    pub name: UserName,
    pub password: UserPassword,
}