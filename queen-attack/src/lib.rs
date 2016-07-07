// Really, rank and file only need three bits apiece.
// I'm really tempted to do some bit-shifting magic to store
// them both within a single u8, but I think that for an
// exercise like this, that optimization would be both
// premature and pointless.
pub struct ChessPosition {
    rank: u8,
    file: u8,
}

impl ChessPosition {
    // not a huge fan of allowing signed integers in the constructor, here.
    // it'd make more sense and save code and expose errors at compile time to
    // require `rank` and `file` to be u8s.
    pub fn new(rank: i8, file: i8) -> Result<ChessPosition, &'static str> {
        if rank >= 8 || file >= 8 || rank < 0 || file < 0 {
            Err("Invalid Position")
        } else {
            Ok(ChessPosition {rank: rank as u8, file: file as u8})
        }
    }
}

pub struct Queen {
    position: ChessPosition,
}

impl Queen {
    pub fn new(position: ChessPosition) -> Queen {
        Queen {position: position}
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        unimplemented!()
    }
}
