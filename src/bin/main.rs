#![feature(slice_concat_trait)]

use maze_project::{graph::Solver, Input};

fn main() {
    let input = Input::from_stdin();

    let solver = Solver::new(input);
    let path = solver.solve();

    println!(
        "{}",
        path.iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .as_slice()
            .join(" ")
    );
}
