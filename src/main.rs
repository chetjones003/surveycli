mod cli;
mod csv_reader;
mod point;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::cli_setup();
    let filename = args.get_one::<String>("file").expect("required");

    match csv_reader::read_csv_as_pnezd(filename) {
        Ok(output) => {
            for point in output {
                point.print()
            }
        }
        Err(e) => return Err(e.into()),
    };

    Ok(())
}
