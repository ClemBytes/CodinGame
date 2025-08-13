use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap};
use std::io;

fn main() {
    let input = NetworkDescription::parse();
    input.shortest_path();
}

#[derive(Debug, Clone)]
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
        f64::hypot(x, y) * 6371.
    }
}

#[derive(Debug, PartialEq, Clone)]
struct StateWithParent {
    id: String,
    distance_from_start: f64,
    parent_stop_id: String,
}

impl Eq for StateWithParent {}

impl PartialOrd for StateWithParent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Other and self inversed to do a MIN-heap
        other.distance_from_start.partial_cmp(&self.distance_from_start)
    }
}

impl Ord for StateWithParent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone)]
struct NetworkDescription {
    start_id: String,
    end_id: String,
    stops: HashMap<String, Stop>,
    graph: HashMap<String, Vec<String>>,
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
                    latitude: latitude.to_radians(),
                    longitude: longitude.to_radians(),
                },
            );
        }

        // Number of connections in network
        input_line.clear();
        io::stdin().read_line(&mut input_line).unwrap();
        let nb_connections: u32 = input_line.trim().parse().unwrap();

        // List of connections
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        for _ in 0..nb_connections {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let (start, end) = input_line.trim().split_once(" ").unwrap();
            let start_id = start.split_once(":").unwrap().1.to_string();
            let end_id = end.split_once(":").unwrap().1.to_string();
            if start_id == end_id {
                continue;
            }
            let e = graph.entry(start_id).or_default();
            e.push(end_id);
        }

        NetworkDescription {
            start_id,
            end_id,
            stops,
            graph,
        }
    }

    fn distance_between_two_stops(&self, id1: String, id2: String) -> f64 {
        self.stops.get(&id1).unwrap().distance(self.stops.get(&id2).unwrap())
    }

    fn shortest_path(&self) {
        let mut visited_nodes = BTreeSet::new();
        let mut min_heap: BinaryHeap<StateWithParent> = BinaryHeap::new();
        let mut previous: BTreeMap<String, String> = BTreeMap::new();
        min_heap.push(StateWithParent {
            id: self.start_id.clone(),
            distance_from_start: 0.,
            parent_stop_id: "".to_string(),
        });
        while let Some(current_state) = min_heap.pop() {
            if visited_nodes.contains(&current_state.id) {
                continue;
            }
            visited_nodes.insert(current_state.id.clone());
            previous.insert(current_state.id.clone(), current_state.parent_stop_id.clone());

            // Found the end ?
            if current_state.id == self.end_id {
                // Backtrace the path and store it
                let mut path: Vec<String> = vec![];
                let current = current_state.clone();
                path.push(current.id.clone());
                let mut id = current.id;
                while id != self.start_id {
                    let prev = previous.get(&id).unwrap();
                    path.push(prev.clone());
                    id = prev.clone();
                }

                // Then print it
                while let Some(id) = path.pop() {
                    println!("{}", self.stops.get(&id).unwrap().name);
                }
                return;
            }

            // Otherwise
            let neighbors = self.graph.get(&current_state.id).unwrap();
            for neighbor in neighbors {
                // Update previous
                assert_ne!(neighbor.clone(), current_state.id.clone(), "{:?}", neighbor);
                // Add node to heap
                min_heap.push(StateWithParent {
                    id: neighbor.clone(),
                    distance_from_start: current_state.distance_from_start + self.distance_between_two_stops(neighbor.to_string(), current_state.id.clone()),
                    parent_stop_id: current_state.id.clone(),
                });
            }
        }
        println!("IMPOSSIBLE");
    }
}
