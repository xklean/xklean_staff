#[async_trait]
pub trait IMutationRepository :Send+Sync{
    async fn create_staff()->types::Response<String>;
}