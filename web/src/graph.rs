// based on https://rosettacode.org/wiki/Dijkstra%27s_algorithm#Rust
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::usize;

use Node;
use Way;

#[derive(Clone)]
pub struct Graph {
    nodes: Vec<Node>,
    ways: Vec<Way>,
    offset: Vec<usize>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    node: usize,
    cost: usize,
}
// Manually implement Ord so we get a min-heap instead of a max-heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Graph {
    pub fn new(nodes: Vec<Node>, ways: Vec<Way>, offset: Vec<usize>) -> Self {
        Graph {
            nodes: nodes,
            ways: ways,
            offset: offset,
        }
    }

    /// returns closes point of given long & lat
    pub fn get_point_id(&self, lat: f32, long: f32) -> usize {
        let mut min_distance: f32 = std::f32::MAX;
        let mut min_distance_id: usize = 0;

        for i in 0..self.nodes.len() {
            let distance =
                calc_distance(lat, long, self.nodes[i].latitude, self.nodes[i].longitude);
            if distance < min_distance {
                min_distance = distance;
                min_distance_id = i;
            }
        }
        return min_distance_id;
    }

    /// converts node ids to node-coordinates
    pub fn get_coordinates(&self, nodes: Vec<usize>) -> Vec<Node> {
        let mut result: Vec<Node> = Vec::with_capacity(nodes.len());
        for (i, node) in nodes.iter().enumerate() {
            result[i] = Node {
                latitude: self.nodes[i].latitude,
                longitude: self.nodes[i].longitude,
            };
        }
        return result;
    }

    /// returns the edge weight from source to target
    pub fn get_edge_weight(&self, source: usize, target: usize, weight: usize) -> usize {
        let first_edge = self.offset[source];
        let last_edge = self.offset[source + 1];
        for i in first_edge..last_edge {
            if self.ways[i].target == target {
                return self.ways[i].weight;
            }
        }
        return usize::max_value();
    }

    /// executes dijkstra
    pub fn find_path(
        &self,
        start: usize,
        end: usize,
        kind: usize,
        use_distance: bool,
    ) -> Option<(Vec<usize>, usize)> {
        println!("{:?}", self.nodes.len());
        let mut dist = vec![(usize::MAX, None); self.nodes.len()];

        let mut heap = BinaryHeap::new();
        dist[start] = (0, None);
        heap.push(State {
            node: start,
            cost: 0,
        });

        while let Some(State { node, cost }) = heap.pop() {
            if node == end {
                let mut path = Vec::with_capacity(dist.len() / 2);
                let mut current_dist = dist[end];
                path.push(end);
                while let Some(prev) = current_dist.1 {
                    path.push(prev);
                    current_dist = dist[prev];
                }
                path.reverse();
                return Some((path, cost));
            }

            if cost > dist[node].0 {
                continue;
            }
            for edge in self.offset[node]..self.offset[node + 1] {
                let current_way: Way = self.ways[edge];
                let next = State {
                    node: current_way.target,
                    // TODO get new costs here
                    // kind, use_distance
                    cost: cost + current_way.weight,
                };
                if next.cost < dist[next.node].0 {
                    dist[next.node] = (next.cost, Some(node));
                    heap.push(next);
                }
            }
        }
        None
    }
}

/// get distance on earth surface using haversine formula
fn calc_distance(lat_1: f32, long_1: f32, lat_2: f32, long_2: f32) -> f32 {
    let r: f32 = 6371.0; // constant used for meters
    let d_lat: f32 = (lat_2 - lat_1).to_radians();
    let d_lon: f32 = (long_2 - long_1).to_radians();
    let lat1: f32 = (lat_1).to_radians();
    let lat2: f32 = (lat_2).to_radians();

    let a: f32 = ((d_lat / 2.0).sin()) * ((d_lat / 2.0).sin())
        + ((d_lon / 2.0).sin()) * ((d_lon / 2.0).sin()) * (lat1.cos()) * (lat2.cos());
    let c: f32 = 2.0 * ((a.sqrt()).atan2((1.0 - a).sqrt()));
    return r * c;
}

/* just notes from felix:
pub fn dijkstra(usize: start) {}
pub fn find_way(usize: goal) -> List<usize> {}
fn get_shortest_path(target: usize, visited: &Vec<usize>) -> Vec<usize> {}
pub fn edgesToNodes(edges: Vec<usize>) -> Vec<usize> {}
pub fn getDistance(goal: usize) -> usize {}
pub fn setStart(start: usize) -> bool {}
pub fn initDistanceTable() {}
*/
