use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "JDKMan", version, author, about)]
pub struct JdkManArgs {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Parser)]
pub enum Command {
    Add,
    List {
        #[clap(short, long, help = "Only list installed versions")]
        installed: bool,
    },
    Remove,
    Use,
}

impl JdkManArgs {
    pub fn parse() -> Self {
        debug!("Parsing Args");
        clap::Parser::parse()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CommandError {}

#[async_trait::async_trait]
pub trait CommandExecutor {
    async fn execute(&self) -> Result<(), CommandError>;
}
