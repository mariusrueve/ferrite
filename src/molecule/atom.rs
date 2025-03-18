use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Atom {
    pub atomic_number: u8,
    pub formal_charge: i8,
    pub implicit_hydrogen: u8,
    pub is_aromatic: bool,
    pub isotope: Option<u16>,
}

impl Atom {
    pub fn new(atomic_number: u8) -> Atom {
        Atom {
            atomic_number,
            formal_charge: 0,
            implicit_hydrogen: 0,
            is_aromatic: false,
            isotope: None,
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self.atomic_number {
            1 => "H",
            6 => "C",
            7 => "N",
            8 => "O",
            9 => "F",
            15 => "P",
            16 => "S",
            17 => "Cl",
            35 => "Br",
            53 => "I",
            _ => "X",
        }
    }

    pub fn valence(&self) -> u8 {
        // Calculate valence based on atomic number and charge
        // This is a simplification
        match self.atomic_number {
            1 => 1,  // H
            6 => 4,  // C
            7 => 3,  // N
            8 => 2,  // O
            // Add more elements
            _ => 0,
        }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}
            