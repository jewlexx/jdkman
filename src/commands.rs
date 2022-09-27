#[derive(Debug, thiserror::Error)]
pub enum CommandError {}

#[async_trait::async_trait]
pub trait Command {
    async fn execute(&self) -> Result<(), CommandError>;
}
