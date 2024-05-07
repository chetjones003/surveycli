use clap::{arg, ArgMatches, Command};

pub fn cli_setup() -> ArgMatches {
    let matches = Command::new("Surveycli")
        .version("1.0")
        .author("Chet Jones")
        .about("Reads a csv file and allows for editing and cogo calculations")
        .arg(
            arg!(--file <VALUE>)
                .short('f')
                .long("file")
                .help("Sets the input CSV file to use")
                .required(true),
        )
        .get_matches();

    return matches;
}
