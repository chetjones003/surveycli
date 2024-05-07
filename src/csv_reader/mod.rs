use crate::point::Point;
use std::error::Error;
use std::fs::File;
use std::path::Path;

/// Read the entire contents of a file while decoding and parsing it to the PNEZD struct.
/// P -> Point ID, N -> Northing, E -> Easting, Z -> Elevation, D -> Description
///
/// # Errors
///
/// This function will return an error if `path` does not already exist.
/// Or if any of the parsing actions error
pub fn read_csv_as_pnezd<P>(path: P) -> Result<Vec<Point>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    let mut points: Vec<Point> = Vec::new();
    for result in reader.records() {
        let record = result?;
        let id: u64 = record[0].parse()?;
        let northing: f64 = record[1].parse()?;
        let easting: f64 = record[2].parse()?;
        let elevation: f64 = record[3].parse()?;
        let description: Option<String> = record[4].parse().ok();

        let some = Point {
            id,
            northing,
            easting,
            elevation,
            description,
        };

        points.push(some)
    }

    Ok(points)
}
