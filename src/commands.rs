#[derive(Debug, thiserror::Error)]
pub enum CommandError {}

pub trait Command {
    fn execute(&self) -> Result<(), CommandError>;
}
