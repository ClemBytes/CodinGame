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
struct StateWithParent<'a> {
    id: &'a str,
    distance_from_start: f64,
    parent_stop_id: &'a str,
}

impl Eq for StateWithParent<'_> {}

#[allow(clippy::non_canonical_partial_ord_impl)]
// Had to cheat to define Ord on f64
impl PartialOrd for StateWithParent<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Other and self inversed to do a MIN-heap
        other
            .distance_from_start
            .partial_cmp(&self.distance_from_start)
    }
}

impl Ord for StateWithParent<'_> {
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
        input_line.clear();
        io::stdin().read_line(&mut input_line).unwrap();
        let end_id = input_line.trim().split_once(":").unwrap().1.to_string();

        // Number of stops in network
        input_line.clear();
        io::stdin().read_line(&mut input_line).unwrap();
        let nb_stops: u32 = input_line.trim().parse().unwrap();

        // List of stops
        let mut stops: HashMap<String, Stop> = HashMap::new();
        for _ in 0..nb_stops {
            input_line.clear();
            io::stdin().read_line(&mut input_line).unwrap();
            let stop = input_line.trim().split_once(":").unwrap().1;
            let stop_infos: Vec<&str> = stop.split(",").collect();

            // Stop ID
            let id: String = stop_infos[0].to_string();

            // Stop name
            let name = stop_infos[1]
                .strip_prefix('"')
                .unwrap()
                .strip_suffix('"')
                .unwrap()
                .to_string();

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
            input_line.clear();
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

    fn distance_between_two_stops(&self, id1: &str, id2: &str) -> f64 {
        self.stops
            .get(id1)
            .unwrap()
            .distance(self.stops.get(id2).unwrap())
    }

    fn shortest_path(&self) {
        let mut visited_nodes = BTreeSet::new();
        let mut min_heap: BinaryHeap<StateWithParent> = BinaryHeap::new();
        let mut previous: BTreeMap<&str, &str> = BTreeMap::new();
        min_heap.push(StateWithParent {
            id: &self.start_id,
            distance_from_start: 0.,
            parent_stop_id: "",
        });
        while let Some(current_state) = min_heap.pop() {
            if visited_nodes.contains(&current_state.id) {
                continue;
            }
            visited_nodes.insert(current_state.id);
            previous.insert(current_state.id, current_state.parent_stop_id);

            // Found the end ?
            if current_state.id == self.end_id {
                // Backtrace the path and store it
                let mut path: Vec<&str> = vec![];
                path.push(current_state.id);
                let mut id = current_state.id;
                while id != self.start_id {
                    let prev = *previous.get(id).unwrap();
                    path.push(prev);
                    id = prev;
                }

                // Then print it
                while let Some(id) = path.pop() {
                    println!("{}", self.stops.get(id).unwrap().name);
                }
                return;
            }

            // Otherwise
            let neighbors = self.graph.get(current_state.id).unwrap();
            for neighbor in neighbors {
                // Update previous
                assert_ne!(neighbor, current_state.id, "{neighbor:?}");
                // Add node to heap
                min_heap.push(StateWithParent {
                    id: neighbor,
                    distance_from_start: current_state.distance_from_start
                        + self.distance_between_two_stops(neighbor, current_state.id),
                    parent_stop_id: current_state.id,
                });
            }
        }
        println!("IMPOSSIBLE");
    }
}
