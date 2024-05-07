pub struct Point {
    pub id: u64,
    pub northing: f64,
    pub easting: f64,
    pub elevation: f64,
    pub description: Option<String>,
}

impl Point {
    pub fn print(&self) {
        println!(
            "id: {}, northing: {}, easting: {}, elevation: {}, description: {:?}",
            self.id, self.northing, self.easting, self.elevation, self.description
        );
    }
}
