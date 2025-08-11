use std::fs::File;
use std::io::{self, BufRead};

#[test]
fn test() {
    run();
}

pub fn run() {
    let input_path = "input/ex1";
    let input = NetworkDescription::parse(input_path);
    println!("{input:#?}");
}

#[derive(Debug)]
struct Stop {
    id: String,
    name: String,
    latitude: f64,
    longitude: f64,
}

impl Stop {
    fn distance(&self, other: Self) -> f64 {
        let x =
            (other.longitude - self.longitude) * f64::cos((self.latitude + other.latitude) / 2.);
        let y = other.latitude - self.latitude;
        f64::sqrt(x * x + y * y) * 6371.
    }
}

#[derive(Debug)]
struct NetworkDescription {
    start_id: String,
    end_id: String,
    nb_stops: u32,
    stops: Vec<Stop>,
    nb_connections: u32,
    connections: Vec<(String, String)>,
}

impl NetworkDescription {
    fn parse(input_path: &str) -> Self {
        let file = File::open(input_path).unwrap();
        let reader = io::BufReader::new(file);
        let mut lines = reader.lines();

        // Start & end points
        let mut input_line = lines.next().unwrap().unwrap();
        let start_id = input_line.trim().split_once(":").unwrap().1.to_string();
        input_line = lines.next().unwrap().unwrap();
        let end_id = input_line.trim().split_once(":").unwrap().1.to_string();

        // Number of stops in network
        input_line = lines.next().unwrap().unwrap();
        let nb_stops: u32 = input_line.trim().parse().unwrap();

        // List of stops
        let mut stops: Vec<Stop> = vec![];
        for _ in 0..nb_stops {
            input_line = lines.next().unwrap().unwrap();
            let stop = input_line.trim().split_once(":").unwrap().1;
            let stop_infos: Vec<&str> = stop.split(",").collect();

            // Stop ID
            let id: String = stop_infos[0].to_string();

            // Stop name
            let mut name = stop_infos[1].chars();
            name.next(); // Delete opening "
            name.next_back(); // Delete closing "
            let name = name.as_str().to_string();

            // Stop latitude & longitude
            let latitude: f64 = stop_infos[3].parse().unwrap();
            let longitude: f64 = stop_infos[4].parse().unwrap();

            stops.push(Stop {
                id,
                name,
                latitude,
                longitude,
            });
        }

        // Number of connections in network
        input_line = lines.next().unwrap().unwrap();
        let nb_connections: u32 = input_line.trim().parse().unwrap();

        // List of connections
        let mut connections: Vec<(String, String)> = vec![];
        for _ in 0..nb_connections {
            input_line = lines.next().unwrap().unwrap();
            let (start, end) = input_line.split_once(" ").unwrap();
            connections.push((
                start.split_once(":").unwrap().1.to_string(),
                end.split_once(":").unwrap().1.to_string(),
            ));
        }

        let description = NetworkDescription {
            start_id,
            end_id,
            nb_stops,
            stops,
            nb_connections,
            connections,
        };
        description
    }
}
