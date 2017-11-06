#[macro_use]
extern crate try_opt;

use std::hash::Hasher;
use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::collections::vec_deque::VecDeque;


/// Solve the bucket problem
pub fn solve(capacity_1: u8, capacity_2: u8, goal: u8, start_bucket: &Bucket) -> BucketStats {
    let initial = BucketState::new(capacity_1, capacity_2, goal, *start_bucket);
    if let Some(bs) = initial.try_into_bucket_stats() {
        return bs;
    }

    let mut visited_states = HashSet::new();
    // initialize the visited states with initial and the reverse
    // this prohibits us from retraversing prohibited terrain
    visited_states.insert(initial.hash_value());
    {
        // we don't actually want to keep the reverse value around, though,
        // so we make a little sub-context here
        let reversed = BucketState::new(capacity_1, capacity_2, goal, start_bucket.other());
        visited_states.insert(reversed.hash_value());
    }

    let mut bfs_queue = VecDeque::with_capacity(8);
    bfs_queue.extend(initial.subsequents(&visited_states));
    while bfs_queue.len() > 0 {
        // unwrap is safe here because we _just_ checked that len() > 0
        let current = bfs_queue.pop_front().unwrap();
        if let Some(bs) = current.try_into_bucket_stats() {
            return bs;
        }
        visited_states.insert(current.hash_value());
        bfs_queue.extend(current.subsequents(&visited_states));
    }
    unreachable!()
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Bucket {
    One,
    Two,
}

impl Bucket {
    fn other(&self) -> Bucket {
        match *self {
            Bucket::One => Bucket::Two,
            Bucket::Two => Bucket::One,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct BucketState {
    capacity_1: u8,
    capacity_2: u8,
    value_1: u8,
    value_2: u8,
    goal: u8,
    moves: u8,
}

impl BucketState {
    fn new(capacity_1: u8, capacity_2: u8, goal: u8, start: Bucket) -> BucketState {
        BucketState {
            capacity_1: capacity_1,
            capacity_2: capacity_2,
            value_1: if start == Bucket::One { capacity_1 } else { 0 },
            value_2: if start == Bucket::Two { capacity_2 } else { 0 },
            goal: goal,
            moves: 1,
        }
    }

    fn value(&self, b: Bucket) -> u8 {
        match b {
            Bucket::One => self.value_1,
            Bucket::Two => self.value_2,
        }
    }

    fn capacity(&self, b: Bucket) -> u8 {
        match b {
            Bucket::One => self.capacity_1,
            Bucket::Two => self.capacity_2,
        }
    }

    /// generate a subsequent state from the current state
    fn subsequent(&self, b: Bucket, b_value: u8, o_value: u8) -> BucketState {
        match b {
            Bucket::One => {
                BucketState {
                    value_1: b_value,
                    value_2: o_value,
                    moves: self.moves + 1,
                    ..*self
                }
            }
            Bucket::Two => {
                BucketState {
                    value_2: b_value,
                    value_1: o_value,
                    moves: self.moves + 1,
                    ..*self
                }
            }
        }
    }

    /// Pour everything from the from bucket into the other, stopping
    /// only when the from bucket is empty.
    fn pour_overflow(&self, from: Bucket) -> BucketState {
        let to = from.other();
        let mut to_value = self.value(to) + self.value(from);
        if to_value > self.capacity(to) {
            to_value = self.capacity(to);
        }
        self.subsequent(from, 0, to_value)
    }

    /// Pour from the from bucket into the other bucket, stopping when
    /// the from bucket is empty or the other bucket is full.
    fn pour_careful(&self, from: Bucket) -> BucketState {
        let to = from.other();
        let mut to_value = self.value(to) + self.value(from);
        let mut from_value = 0;
        if to_value > self.capacity(to) {
            from_value = to_value - self.capacity(to);
            to_value = self.capacity(to);
        }
        self.subsequent(from, from_value, to_value)
    }

    /// Empty the specified bucket.
    fn empty(&self, b: Bucket) -> BucketState {
        self.subsequent(b, 0, self.value(b.other()))
    }

    /// Fill the specified bucket.
    fn fill(&self, b: Bucket) -> BucketState {
        self.subsequent(b, self.capacity(b), self.value(b.other()))
    }

    /// Produce a hash value for this BucketState.
    ///
    /// This value considers _only_ the contents of each bucket.
    fn hash_value(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        hasher.write_u8(self.value_1);
        hasher.write_u8(self.value_2);
        hasher.finish()
    }

    /// Generate unvisited subsequent states from the current state
    fn subsequents(&self, previously_visited: &HashSet<u64>) -> Vec<BucketState> {
        let mut output = Vec::with_capacity(8);
        for &bucket in [Bucket::One, Bucket::Two].into_iter() {
            if self.value(bucket) != 0 {
                output.push(self.empty(bucket));
            }
            if self.value(bucket) != self.capacity(bucket) {
                output.push(self.fill(bucket));
            }
            if self.value(bucket.other()) != self.capacity(bucket.other()) {
                output.push(self.pour_overflow(bucket));
            }
            if self.value(bucket) != 0 &&
                self.value(bucket.other()) != self.capacity(bucket.other())
            {
                output.push(self.pour_careful(bucket));
            }
        }
        output.retain(|s| !previously_visited.contains(&s.hash_value()));
        output
    }

    /// Some(bucket) if that bucket has reached the goal
    fn is_goal(&self) -> Option<Bucket> {
        if self.value_1 == self.goal {
            Some(Bucket::One)
        } else if self.value_2 == self.goal {
            Some(Bucket::Two)
        } else {
            None
        }
    }

    fn try_into_bucket_stats(&self) -> Option<BucketStats> {
        let goal = try_opt!(self.is_goal());
        Some(BucketStats {
            moves: self.moves,
            goal_bucket: goal,
            other_bucket: self.value(goal.other()),
        })
    }
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}
