use std::collections::HashMap;
use std::io;
use std::cmp::Ordering;

fn main() {
    let input = NetworkDescription::parse();
    println!("{input:#?}");
}

#[derive(Debug)]
struct Stop {
    name: String,
    latitude: f64,
    longitude: f64,
}

impl Stop {
    fn distance(&self, other: &Self) -> f64 {
        let x =
            (other.longitude - self.longitude) * f64::cos((self.latitude + other.latitude) / 2.);
        let y = other.latitude - self.latitude;
        f64::sqrt(x * x + y * y) * 6371.
    }
}

#[derive(Debug, PartialEq)]
struct Node {
    id: String,
    distance: f64,
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Other and self inversed to do a MIN-heap
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug)]
struct NetworkDescription {
    start_id: String,
    end_id: String,
    stops: HashMap<String, Stop>,
    graph: HashMap<String, Vec<Node>>,
}

impl NetworkDescription {
    fn parse() -> Self {
        // Start & end points
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let start_id = input_line.trim().split_once(":").unwrap().1.to_string();
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let end_id = input_line.trim().split_once(":").unwrap().1.to_string();

        // Number of stops in network
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let nb_stops: u32 = input_line.trim().parse().unwrap();

        // List of stops
        let mut stops: HashMap<String, Stop> = HashMap::new();
        for _ in 0..nb_stops {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
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

            stops.insert(
                id,
                Stop {
                    name,
                    latitude,
                    longitude,
                },
            );
        }

        // Number of connections in network
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let nb_connections: u32 = input_line.trim().parse().unwrap();

        // List of connections
        let mut graph: HashMap<String, Vec<Node>> = HashMap::new();
        for _ in 0..nb_connections {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let (start, end) = input_line.trim().split_once(" ").unwrap();
            let start_id = start.split_once(":").unwrap().1.to_string();
            let start_stop = stops.get(&start_id).unwrap();
            let end_id = end.split_once(":").unwrap().1.to_string();
            let end_stop = stops.get(&end_id).unwrap();
            let e = graph.entry(start_id).or_default();
            e.push(Node { id: end_id, distance: start_stop.distance(end_stop) });
        }

        NetworkDescription {
            start_id,
            end_id,
            stops,
            graph,
        }
    }
}
