#[derive(Debug)]
pub enum TriangleError {
    ZeroLengthSide,
    ImpossibleSideLengths,
}

pub struct Triangle {
    sides: [usize; 3],
}

impl Triangle {
    pub fn build(sides: [usize; 3]) -> Result<Triangle, TriangleError> {
        if sides.iter().any(|s| *s == 0) {
            Err(TriangleError::ZeroLengthSide)
        } else {
            Ok(Triangle { sides: sides })
        }
    }

    pub fn is_isosceles(&self) -> bool {
        unimplemented!()
    }

    pub fn is_scalene(&self) -> bool {
        unimplemented!()
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[1] == self.sides[2]
    }
}
