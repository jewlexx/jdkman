use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "JDKMan", version, author, about)]
pub struct JdkManArgs {}

impl JdkManArgs {
    pub fn parse() -> Self {
        debug!("Parsing Args");
        clap::Parser::parse()
    }
}
