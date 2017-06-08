#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Frame {
    Empty, // Frame has not yet been played
    OneRoll(u8), // One ball rolled, no strike
    Open(u8, u8), // Two balls rolled, no spare (first, total)
    Spare(u8),
    Strike,
}
use Frame::*;

impl Frame {
    fn allows_fill(&self) -> bool {
        match *self {
            Spare(_) | Strike => true,
            _ => false,
        }
    }

    fn first_roll(&self) -> usize {
        match *self {
            Empty => 0,
            OneRoll(v) | Open(v, _) | Spare(v) => v as usize,
            Strike => 10,
        }
    }

    fn score(&self, next: &Frame, second_next: &Frame) -> usize {
        match *self {
            Empty => 0,
            OneRoll(v) | Open(_, v) => v as usize,
            Spare(_) => 10 + next.first_roll(),
            Strike => {
                10 +
                match *next {
                    Strike => 10 + second_next.first_roll(),
                    Spare(_) => 10,
                    Open(_, t) => t as usize,
                    _ => next.first_roll(),
                }
            }
        }
    }

    fn is_complete(&self) -> bool {
        match *self {
            Empty | OneRoll(_) => false,
            _ => true,
        }
    }
}

const NUM_PINS: u8 = 10;
const FRAMES_IN_GAME: usize = 10;
const FILL_FRAMES: usize = 2;

pub struct BowlingGame {
    frames: [Frame; FRAMES_IN_GAME + FILL_FRAMES],
    current: usize,
}

impl BowlingGame {
    pub fn new() -> BowlingGame {
        BowlingGame {
            frames: [Frame::Empty; FRAMES_IN_GAME + FILL_FRAMES],
            current: 0,
        }
    }

    pub fn roll(&mut self, pins: u8) -> Result<(), &'static str> {
        if self.current >= FRAMES_IN_GAME {
            if !self.frames[FRAMES_IN_GAME - 1].allows_fill() ||
               self.current >= FRAMES_IN_GAME + FILL_FRAMES ||
               (self.current == FRAMES_IN_GAME + 1 && self.frames[FRAMES_IN_GAME] != Strike) {
                return Err("Game is over");
            }
        }
        let too_many_pins = Err("Can't roll more pins than exist");
        let (new_frame, current_incr) = match self.frames[self.current] {
            Empty => {
                if pins < NUM_PINS {
                    (OneRoll(pins), 0)
                } else if pins == NUM_PINS {
                    (Strike, 1)
                } else {
                    return too_many_pins;
                }
            }
            OneRoll(p) => {
                let total = p + pins;
                if total < NUM_PINS {
                    (Open(p, total), 1)
                } else if total == NUM_PINS {
                    (Spare(p), 1)
                } else {
                    return too_many_pins;
                }
            }
            _ => unreachable!("Current frame should never be completed"),
        };

        self.frames[self.current] = new_frame;

        println!("Rolled {} in frame {} for {:?} scoring {}",
                 pins,
                 self.current + 1,
                 new_frame,
                 self.incremental_score(),
             );

        self.current += current_incr;
        Ok(())
    }

    pub fn score(&self) -> Result<usize, &'static str> {
        let incomplete = Err("Can only score a complete game");
        if self.frames.iter().take(FRAMES_IN_GAME).any(|frame| *frame == Empty) {
            return incomplete;
        }
        match self.frames[FRAMES_IN_GAME - 1] {
            Spare(_) => {
                if self.frames[FRAMES_IN_GAME] == Empty {
                    return incomplete;
                }
            }
            Strike => {
                if !self.frames[FRAMES_IN_GAME].is_complete() ||
                   (self.frames[FRAMES_IN_GAME] == Strike &&
                    self.frames[FRAMES_IN_GAME + 1] == Empty) {
                    return incomplete;
                }
            }
            _ => {}
        }
        Ok(self.incremental_score())
    }

    /// Return the minimum final score possible given the current game state
    ///
    /// You know, the way real bowling software works
    pub fn incremental_score(&self) -> usize {
        let mut tally = 0;
        for three_frames in self.frames.windows(3) {
            tally += three_frames[0].score(&three_frames[1], &three_frames[2]);
        }
        tally
    }
}
