use serde::{Deserialize, Serialize};
use crate::adapters::entites::{AddressEntity};

#[derive(Serialize,Deserialize,Default,Clone)]
pub struct Address {
    pub id:uuid::Uuid,
    pub street_name :String,
    pub suburb:String,
    pub post_code:String,
    pub state:String,
    pub country:String,
    pub primary:bool
}

impl From<AddressEntity> for Address{
    fn from(value: AddressEntity) -> Self {
       return Self{
           id: value.id,
           street_name: value.street_name,
           suburb: value.suburb,
           post_code: value.post_code,
           state: value.state,
           country:value.country,
           primary: value.primary,
       }
    }
}

impl From<Address> for AddressEntity{
    fn from(value: Address) -> Self {
        return Self{
            id: value.id,
            street_name: value.street_name,
            suburb: value.suburb,
            post_code: value.post_code,
            state: value.state,
            country:value.country,
            primary: value.primary,
        }
    }
}