use crate::adapters::data::Staff;
use crate::adapters::repository;
use crate::adapters::repository::IMutationRepository;


impl IMutationRepository for Repository{
    async fn create_staff() -> types::Response<String> {
        todo!()
    }
}