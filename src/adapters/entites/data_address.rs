
#[derive(Serialize,Deserialize,Default,Clone)]
pub struct AddressEntity {
    pub id:uuid::Uuid,
    pub street_name :String,
    pub suburb:String,
    pub post_code:String,
    pub state:String,
    pub country:String,
    pub primary:bool

}