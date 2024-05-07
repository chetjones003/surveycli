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
                .help("Sets the input CSV file to use. Can be absolute <path> or relative `cwd` <path>")
                .required(true)
        )
        .subcommand(
            Command::new("inverse")
                .about("Inverse between two points in file specified by entering point id as such `-i 1-2`")
                .arg(arg!([POINT1]))
                .arg(arg!([POINT2]))
        )
        .get_matches();

    return matches;
}
