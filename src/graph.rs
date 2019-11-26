use crate::{
    Coord,
    Direction::{self, *},
    Input,
};
use std::collections::HashMap;

use petgraph::{algo::astar, graph::NodeIndex, Graph};
#[derive(Debug, Clone)]
pub struct Solver {
    input: Input,
}

impl Solver {
    pub fn new(input: Input) -> Self {
        Self { input }
    }

    pub fn solve(&self) -> Vec<Coord> {
        let grid = &self.input.grid;

        let mut graph = Graph::new();

        let mut forward_nodes: HashMap<Coord, NodeIndex> = HashMap::new();
        let mut forward_coords: HashMap<NodeIndex, Coord> = HashMap::new();

        let mut backward_nodes: HashMap<Coord, NodeIndex> = HashMap::new();
        let mut backward_coords: HashMap<NodeIndex, Coord> = HashMap::new();

        // Add all of the arrows, forwards and backwards. Associate them
        // with the proper hashmaps.
        grid.iter().flatten().for_each(|&a| {
            let f_idx = graph.add_node(a);
            let b_idx = graph.add_node(a);

            forward_nodes.insert(a.loc, f_idx);
            forward_coords.insert(f_idx, a.loc);

            backward_nodes.insert(a.loc, b_idx);
            backward_coords.insert(b_idx, a.loc);
        });

        let (i, j) = (grid.len(), grid[0].len());

        // Calculate the forward nodes
        for i in 0..i {
            for j in 0..j {
                let arrow = grid[i][j];

                // Skip the target node
                if arrow.is_target() {
                    continue;
                }

                // Determine which map we are reading from. If it is a circle,
                // the map we use is opposite to what pass we are on (forward).
                let map = if arrow.is_circle() {
                    &backward_nodes
                } else {
                    &forward_nodes
                };

                // Get the "velocity" or the vector direction of our arrow's direction.
                let mut vel = get_vel(arrow.direction);

                // If it is a circle, we multiply the velocity by -1.
                if arrow.is_circle() {
                    vel = Coord(-1 * vel.0, -1 * vel.1);
                }

                // Iterate through the nodes, beginning at our current one,
                // adding to the velocity as we go.
                let mut start = arrow.loc;
                let start_idx = forward_nodes[&start].clone();

                start += vel;
                while self.input.in_bounds(&start) {
                    let check_arrow = grid[start.0 as usize][start.1 as usize];

                    // Only create an edge for notes that are the same color.
                    if check_arrow.color != arrow.color {
                        graph.add_edge(start_idx, map[&start].clone(), 0usize);
                    }

                    start += vel;
                }
            }
        }

        // Calculate the backward nodes, this is the same exact code but uses the
        // flipped map and multiples again by -1 since the velocity is different.
        for i in 0..i {
            for j in 0..j {
                let arrow = grid[i][j];

                // Skip the target node
                if arrow.is_target() {
                    continue;
                }

                // Determine which map we are reading from. If it is a circle,
                // the map we use is opposite to what pass we are on (forward).
                let map = if arrow.is_circle() {
                    &forward_nodes
                } else {
                    &backward_nodes
                };

                let mut vel = get_vel(arrow.direction);

                // In the backwards pass, the velocity is opposite.
                vel = Coord(-1 * vel.0, -1 * vel.1);

                // If it is a circle, we multiply the velocity by -1.
                if arrow.is_circle() {
                    vel = Coord(-1 * vel.0, -1 * vel.1);
                }

                let mut start = arrow.loc;
                let start_idx = backward_nodes[&start].clone();

                start += vel;
                while self.input.in_bounds(&start) {
                    let check_arrow = grid[start.0 as usize][start.1 as usize];

                    // Only create an edge for notes that are the same color.
                    if check_arrow.color != arrow.color {
                        graph.add_edge(start_idx, map[&start].clone(), 0usize);
                    }

                    start += vel;
                }
            }
        }

        // Get the starting nodes, and the forwards and backwards
        // target node. We need both since we may end in a backwards state.
        let start = forward_nodes[&self.input.start()];
        let target_f = forward_nodes[&self.input.target()];
        let target_b = backward_nodes[&self.input.target()];

        // Execute A* on the graph searching for our end node,
        // since the weights are the same and the cost is zero,
        // this is literally dijkstra's / BFS.
        let (_, path) = astar(
            &graph,
            start,                              // start
            |n| n == target_f || n == target_b, // is_goal
            |e| *e.weight(),                    // edge_cost
            |_| 0,                              // estimate_cost
        )
        .or_else(|| Some((0, vec![])))
        .unwrap();

        // Get the coordinates, either from the forwards or
        // backwards list. Offset by (1, 1) to make human-readable.
        path.iter()
            .map(|idx| {
                forward_coords
                    .get(idx)
                    .or(backward_coords.get(idx))
                    .cloned()
                    .unwrap()
                    + Coord(1, 1)
            })
            .collect()
    }
}

fn get_vel(dir: Direction) -> Coord {
    match dir {
        N => Coord(-1, 0),
        E => Coord(0, 1),
        S => Coord(1, 0),
        W => Coord(0, -1),
        NE => Coord(-1, 1),
        SE => Coord(1, 1),
        SW => Coord(1, -1),
        NW => Coord(-1, -1),
    }
}
