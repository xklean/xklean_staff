use std::fmt::{Debug, Display, Formatter};
use sea_orm::DbErr;

#[derive(Debug)]
pub struct ServiceErr(pub String);

impl From<String> for ServiceErr{
    fn from(value: String) -> Self {
        return Self(value);
    }
}

impl Display for ServiceErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       return write!(f,"{}",self.0);
    }
}
impl std::error::Error for ServiceErr {

}

impl From<sea_orm::DbErr> for ServiceErr{
    fn from(value: DbErr) -> Self {
        return Self(value.to_string())
    }
}