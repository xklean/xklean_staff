use async_trait::async_trait;

#[async_trait]
pub trait IRepository {
   async fn get_staff_by_id()->Result<(),Err()>;
}