use clap::{arg, Command};

fn cli() -> Command {
    Command::new("dld")
        .about("A command line tool for performing repetative Digital Logic Design Tasks")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("convert")
                .about("Convert the given number form one base system to another")
                .visible_alias("c")
                .arg(arg!(<NUMBER> "The number to convert"))
                .arg(
                    arg!(<FROM> "The base system to convert from")
                        .value_parser(["2", "8", "10", "16"])
                        .num_args(1),
                )
                .arg(
                    arg!(<TO> "The base system to convert to")
                        .value_parser(["2", "8", "10", "16"])
                        .num_args(1),
                )
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("compliment")
                .about("Find R's compliment of the given nubmer")
                .visible_alias("r")
                .arg_required_else_help(true)
                .arg(arg!(<NUMBER> "The number to find compliment of")),
        )
        .subcommand(
            Command::new("k-map")
                .about("Simplify Boolean expressions using Karnaugh maps")
                .visible_alias("k")
                .arg_required_else_help(true)
                .arg(arg!(<EXPRESSION> "The expression to simplify")),
        )
        .subcommand(
            Command::new("logic")
                .about("Evaluate the given logical expression")
                .visible_alias("l")
                .arg_required_else_help(true)
                .arg(arg!(<EXPRESSION> "The expression to evaluate")),
        )
        .subcommand(
            Command::new("truth-table")
                .about("Generate truth tables for given logical expressions")
                .visible_alias("t")
                .arg_required_else_help(true)
                .arg(arg!(<EXPRESSION> "The expression to generate truth-table of" )),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("convert", _sub_matches)) => {
            todo!();
        }
        Some(("compliment", _sub_matches)) => {
            todo!();
        }
        Some(("k-map", _sub_matches)) => {
            todo!();
        }
        Some(("logic", _sub_matches)) => {
            todo!();
        }
        Some(("truth-table", _sub_matches)) => {
            todo!();
        }
        _ => unreachable!(),
    }
}
