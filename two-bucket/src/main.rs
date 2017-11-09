extern crate two_bucket;

use two_bucket::{solve, Bucket};

fn main() {
    let bucket_one = 53;
    let bucket_two = 60;
    let goal = 59;
    let initial = Bucket::Two;
    println!(
        "Computing two bucket solution for ({}, {}, {}, {:?})...",
        bucket_one,
        bucket_two,
        goal,
        initial
    );
    let solution = solve(bucket_one, bucket_two, goal, &initial);
    println!(
        "Solved: Bucket {:?} contains {} after {} steps; the other contains {}",
        solution.goal_bucket,
        goal,
        solution.moves,
        solution.other_bucket
    );
}
