use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Default)]
pub struct Address{
    pub id:uuid::Uuid,
    pub street_name :String,
    pub suburb:String,
    pub post_code:String,
    pub state:String,
    pub country:String
}