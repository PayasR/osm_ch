use super::*;

/// get all up edges from one node
#[allow(dead_code)]
pub fn get_edges_from_id(ids: Vec<EdgeId>, edges: &Vec<Way>) -> Vec<Way> {
    return ids.par_iter().map(|x| edges[*x]).collect();
}

/// get all up edge-ids from one node
#[allow(dead_code)]
pub fn get_up_edge_ids(node: NodeId, up_offset: &Vec<EdgeId>) -> Vec<EdgeId> {
    return (up_offset[node]..up_offset[node + 1]).collect();
}

/// get all down edge-ids from one node
#[allow(dead_code)]
pub fn get_down_edge_ids(
    node: NodeId,
    down_offset: &Vec<EdgeId>,
    down_index: &Vec<EdgeId>,
) -> Vec<EdgeId> {
    let prev: Vec<EdgeId> = (down_offset[node]..down_offset[node + 1]).collect();
    return prev.par_iter().map(|x| down_index[*x]).collect();
}

/// get all down edge-ids from one node
#[allow(dead_code)]
pub fn get_edge_ids(
    node: NodeId,
    up_offset: &Vec<EdgeId>,
    down_offset: &Vec<EdgeId>,
    down_index: &Vec<EdgeId>,
) -> (Vec<EdgeId>, Vec<EdgeId>) {
    let outgoing: Vec<NodeId> = get_up_edge_ids(node, &up_offset);
    let incomming: Vec<NodeId> = get_down_edge_ids(node, &down_offset, &down_index);
    return (outgoing, incomming);
}

/// get all edge-ids from one node
#[allow(dead_code)]
pub fn get_all_edge_ids(
    node: NodeId,
    up_offset: &Vec<EdgeId>,
    down_offset: &Vec<EdgeId>,
    down_index: &Vec<EdgeId>,
) -> Vec<EdgeId> {
    let (outgoing, incomming) = get_edge_ids(node, &up_offset, &down_offset, &down_index);
    let mut connected_edges = outgoing;
    connected_edges.extend(&incomming);
    return connected_edges;
}

/// get all up neighbors from one node
#[allow(dead_code)]
pub fn get_up_neighbors(node: NodeId, edges: &Vec<Way>, up_offset: &Vec<EdgeId>) -> Vec<EdgeId> {
    let next = get_up_edge_ids(node, &up_offset);
    let mut tmp: Vec<EdgeId> = next.par_iter().map(|x| edges[*x].target).collect();
    tmp.dedup();
    return tmp;
}

/// get all up neighbors from one node
#[allow(dead_code)]
pub fn get_down_neighbors(
    node: NodeId,
    edges: &Vec<Way>,
    down_offset: &Vec<EdgeId>,
    down_index: &Vec<EdgeId>,
) -> Vec<EdgeId> {
    let prev = get_down_edge_ids(node, &down_offset, &down_index);
    let mut tmp: Vec<EdgeId> = prev.par_iter().map(|x| edges[*x].source).collect();
    tmp.par_sort_unstable();
    tmp.dedup();
    return tmp;
}

/// returning all previous and next neighbors
#[allow(dead_code)]
pub fn get_neighbours(
    node: NodeId,
    edges: &Vec<Way>,
    up_offset: &Vec<EdgeId>,
    down_offset: &Vec<EdgeId>,
    down_index: &Vec<EdgeId>,
) -> (Vec<usize>, Vec<usize>) {
    let targets: Vec<NodeId> = get_up_neighbors(node, &edges, &up_offset);
    let sources: Vec<NodeId> = get_down_neighbors(node, &edges, &down_offset, &down_index);
    return (targets, sources);
}

/// returning all neighbors
#[allow(dead_code)]
pub fn get_all_neighbours(
    node: NodeId,
    edges: &Vec<Way>,
    up_offset: &Vec<EdgeId>,
    down_offset: &Vec<EdgeId>,
    down_index: &Vec<EdgeId>,
) -> Vec<usize> {
    let (targets, sources) = get_neighbours(node, edges, up_offset, down_offset, down_index);
    let mut neighbours = targets;
    neighbours.extend(&sources);
    return neighbours;
}
