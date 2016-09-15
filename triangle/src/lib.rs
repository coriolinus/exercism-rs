extern crate num_traits;
use num_traits::Zero;

#[derive(Debug)]
pub enum TriangleError {
    ZeroLengthSide,
    ImpossibleSideLengths,
}

pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T> Triangle<T>
    where T: Clone + ::std::fmt::Debug + Ord + PartialEq + Zero
{
    /// True if the longest side is shorter than the sum of the other two sides
    pub fn is_possible_triangle(sides: &mut [T; 3]) -> bool {
        sides.sort();
        sides[0].clone() + sides[1].clone() >= *&sides[2]
    }

    pub fn build(mut sides: [T; 3]) -> Result<Triangle<T>, TriangleError> {
        if sides.iter().any(|s| *s <= T::zero()) {
            Err(TriangleError::ZeroLengthSide)
        } else if !Triangle::is_possible_triangle(&mut sides) {
            Err(TriangleError::ImpossibleSideLengths)
        } else {
            Ok(Triangle { sides: sides })
        }
    }

    pub fn document(&self) {
        println!("Sides: {:?}", self.sides);
        println!(" Eq. Sides:   {}", self.count_equal_sides());
        println!(" Equilateral: {}", self.is_equilateral());
        println!(" Isosceles:   {}", self.is_isosceles());
        println!(" Scalene:     {}", self.is_scalene());
    }

    /// Counts how many sides are of equal length
    fn count_equal_sides(&self) -> u8 {
        let mut equals = 0;

        for (i1, i2) in vec![(0, 1), (1, 2), (2, 0)] {
            if self.sides[i1] == self.sides[i2] {
                equals += 1;
            }
        }

        equals
    }

    /// True when exactly two sides have the same length.
    ///
    /// This isn't strictly mathematically right--a triangle
    /// can be both isosceles and equilateral--but it's what
    /// the tests expect.
    pub fn is_isosceles(&self) -> bool {
        self.count_equal_sides() == 1
    }

    /// True when all three sides have unique lengths.
    pub fn is_scalene(&self) -> bool {
        self.count_equal_sides() == 0
    }

    /// True when all three sides have the same length.
    pub fn is_equilateral(&self) -> bool {
        self.count_equal_sides() == 3
    }
}
