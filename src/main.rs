mod cli;
mod csv_reader;
mod point;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::cli_setup();
    let filename = args.get_one::<String>("file").expect("required");

    let _file = match csv_reader::read_csv_as_pnezd(filename) {
        Ok(output) => {
            for point in output {
                point.print()
            }
        }
        Err(e) => return Err(e.into()),
    };

    match args.subcommand() {
        Some(("inverse", sub_matches)) => println!(
            "inverse was used with arguments `{:?}` and `{:?}`",
            sub_matches.get_one::<String>("POINT1"),
            sub_matches.get_one::<String>("POINT2")
        ),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    };

    Ok(())
}
