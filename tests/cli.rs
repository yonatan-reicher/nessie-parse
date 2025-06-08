use nessie_parse::{
    parser::{self, OptionParserExt},
    Parser, ParserExt,
};
use std::path::PathBuf;

type Token = String;

type Warning = String;

type Error = String;

enum RunMode {
    Stdin,
    File(PathBuf),
    String(String),
}

enum Cli {
    Help,
    Repl { load: Option<Vec<String>> },
    Run(RunMode),
    Debug(RunMode),
}

fn run_mode_parser() -> impl Parser<Token, Warning, Error, RunMode> {
    parser::cases()
        .case(parser::is_eq("--"), parser::ret(|| RunMode::Stdin))
        .case(
            parser::is_eq("--str"),
            parser::token()
                .some_or_err(|| "Expected string")
                .map(RunMode::String),
        )
        .default(
            parser::token().map(|t| RunMode::File(t.into())))
}

fn cli_parser() -> Parser<Token, Warning, Error> {
    parse::first_of! {
        parse::eq("help", Cli::Help),
        parse::eq("repl", ())
            .and(
                parse::eq("--load", ()).and()
                    .or()
            )
            .map(|load| Repl { load })
        Repl { load: Option<Vec<String>> },
        Run(RunMode),
        Debug(RunMode),
    }
}
