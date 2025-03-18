// examples/basic_molecule.rs
use ferrite::{Atom, BondType, Molecule};

fn main() {
    // Create a molecule (methane - CH4)
    let mut methane = Molecule::new();
    
    // Add carbon atom
    let c_idx = methane.add_atom(Atom::new(6));
    
    // Add four hydrogen atoms
    let h1_idx = methane.add_atom(Atom::new(1));
    let h2_idx = methane.add_atom(Atom::new(1));
    let h3_idx = methane.add_atom(Atom::new(1));
    let h4_idx = methane.add_atom(Atom::new(1));
    
    // Add bonds
    methane.add_bond(c_idx, h1_idx, BondType::Single).unwrap();
    methane.add_bond(c_idx, h2_idx, BondType::Single).unwrap();
    methane.add_bond(c_idx, h3_idx, BondType::Single).unwrap();
    methane.add_bond(c_idx, h4_idx, BondType::Single).unwrap();
    
    println!("Molecule created: {}", methane.get_molecular_formula());
    println!("Number of atoms: {}", methane.atom_count());
    println!("Number of bonds: {}", methane.bond_count());
}