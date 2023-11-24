use clap::{Parser, Subcommand};

use crate::arrays::ArrayArgs;
use crate::json::{JsonDefaultPrinter, JsonFormatter, JsonPrettyPrinter, JsonValueCommand};
use crate::objects::ObjectArgs;
use crate::strings::StringArgs;

pub fn run() {
    let args = Cli::parse();

    let printer = args.printer();

    let command = args.value_command();
    let json_value = command.get_json_value();
    printer.print(json_value);
}

/// A command line tool for generating JSON values
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Pretty print the JSON output
    #[arg(long, short)]
    pretty: bool,
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    fn value_command(&self) -> Box<dyn JsonValueCommand> {
        match &self.command {
            Commands::Array(args) => Box::new(args.clone()),
            Commands::Object(args) => Box::new(args.clone()),
            Commands::String(args) => Box::new(args.clone()),
        }
    }

    fn printer(&self) -> Box<dyn JsonFormatter> {
        if self.pretty {
            Box::new(JsonPrettyPrinter)
        } else {
            Box::new(JsonDefaultPrinter)
        }
    }
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Returns the arguments as a JSON array
    Array(ArrayArgs),
    /// Returns the arguments as a JSON object
    Object(ObjectArgs),
    /// Returns the argument as a JSON string
    String(StringArgs),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
