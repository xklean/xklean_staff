use async_trait::async_trait;

#[async_trait]
pub trait StaffService :Sync +Send {
    async fn manage_staff();
}