#[macro_use]
extern crate lazy_static;

use std::fmt;

#[derive(Copy, Clone)]
pub enum Numeral {
    M = 1000,
    D = 500,
    C = 100,
    L = 50,
    X = 10,
    V = 5,
    I = 1,
}

impl fmt::Display for Numeral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Numeral::M => 'M',
            Numeral::D => 'D',
            Numeral::C => 'C',
            Numeral::L => 'L',
            Numeral::X => 'X',
            Numeral::V => 'V',
            Numeral::I => 'I',
        })
    }
}

lazy_static! {
    static ref REGISTERS: Vec<(Numeral, Numeral, Numeral)> = vec![(Numeral::M, Numeral::D, Numeral::C),
                                                (Numeral::C, Numeral::L, Numeral::X),
                                                (Numeral::X, Numeral::V, Numeral::I)];

}

pub struct Roman(u16);

impl Roman {
    pub fn from(num: u16) -> Roman {
        if num > 3000 {
            panic!("These Roman numerals don't support values > 3000");
        }
        Roman(num)
    }

    pub fn to_string(&self) -> String {
        let mut value = self.0;
        let mut numerals = Vec::new();
        for &(high, mid, low) in REGISTERS.iter() {
            let mut rv = Roman::to_str_range(value, high, mid, low);
            numerals.append(&mut rv.0);
            value = rv.1;
        }
        assert_eq!(value, 0);
        numerals.iter().map(|n| n.to_string()).collect()
    }

    /// compute a roman numeral given a particular register of values
    /// Each register comprises a trio of numerals, each of which interact distinctly
    fn to_str_range(mut value: u16, high: Numeral, mid: Numeral, low: Numeral) -> (Vec<Numeral>, u16) {
        let mut ret = Vec::new();
        while value >= high as u16 {
            ret.push(high);
            value -= high as u16;
        }
        if value >= high as u16 - low as u16 {
            ret.push(low);
            ret.push(high);
            value -= high as u16 - low as u16;
        }
        if value >= mid as u16 {
            ret.push(mid);
            while value >= mid as u16 + low as u16 {
                ret.push(low);
                value -= low as u16;
            }
            value -= mid as u16;
        }
        if value >= mid as u16 - low as u16 {
            ret.push(low);
            ret.push(mid);
            value -= mid as u16 - low as u16;
        }
        while value >= low as u16 {
            ret.push(low);
            value -= low as u16;
        }
        (ret, value)
    }
}
