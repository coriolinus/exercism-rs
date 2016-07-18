const CHESS_MAX: i8 = 8;
const CHESS_MIN: i8 = 0;

// Really, rank and file only need three bits apiece.
// I'm really tempted to do some bit-shifting magic to store
// them both within a single u8, but I think that for an
// exercise like this, that optimization would be both
// premature and pointless.
#[derive(Copy, Clone)]
pub struct ChessPosition {
    rank: u8,
    file: u8,
}

impl ChessPosition {
    // not a huge fan of allowing signed integers in the constructor, here.
    // it'd make more sense and save code and expose errors at compile time to
    // require `rank` and `file` to be u8s.
    pub fn new(rank: i8, file: i8) -> Result<ChessPosition, &'static str> {
        if rank >= CHESS_MAX || file >= CHESS_MAX || rank < CHESS_MIN || file < CHESS_MIN {
            Err("Invalid Position")
        } else {
            Ok(ChessPosition {
                rank: rank as u8,
                file: file as u8,
            })
        }
    }
}

impl std::ops::Sub for ChessPosition {
    type Output = (i8, i8);

    fn sub(self, other: ChessPosition) -> Self::Output {
        (self.rank as i8 - other.rank as i8, self.file as i8 - other.file as i8)
    }
}

pub struct Queen {
    position: ChessPosition,
}

impl Queen {
    pub fn new(position: ChessPosition) -> Queen {
        Queen { position: position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.position.rank == other.position.rank || self.position.file == other.position.file ||
        {
            let (d_rank, d_file) = self.position - other.position;
            d_rank.abs() == d_file.abs()
        }
    }
}
