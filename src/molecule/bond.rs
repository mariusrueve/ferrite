#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BondType {
    Single,
    Double,
    Triple,
    Aromatic,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bond {
    pub bond_type: BondType,
    pub begin_atom_idx: usize,
    pub end_atom_idx: usize,
    pub is_in_ring: bool,
}

impl Bond {
    pub fn new(begin_atom_idx: usize, end_atom_idx: usize, bond_type: BondType) -> Self {
        Bond {
            bond_type,
            begin_atom_idx,
            end_atom_idx,
            is_in_ring: false,
        }
    }
    
    pub fn is_aromatic(&self) -> bool {
        matches!(self.bond_type, BondType::Aromatic)
    }
    
    pub fn order(&self) -> u8 {
        match self.bond_type {
            BondType::Single => 1,
            BondType::Double => 2,
            BondType::Triple => 3,
            BondType::Aromatic => 1, // Conventionally counted as 1.5, but using 1 for simplicity
        }
    }
}