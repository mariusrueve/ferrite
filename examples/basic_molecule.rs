// examples/basic_molecule.rs
use ferrite::{Atom, BondType, Molecule};

fn create_caffeine() -> Molecule {
    let mut caffeine = Molecule::new();
    
    // Create atoms: 8 carbons, 10 hydrogens, 4 nitrogens, 2 oxygens
    // Create the ring atoms (purine backbone)
    let n1 = caffeine.add_atom(Atom::new(7)); // nitrogen
    let c2 = caffeine.add_atom(Atom::new(6)); // carbon
    let n3 = caffeine.add_atom(Atom::new(7)); // nitrogen
    let c4 = caffeine.add_atom(Atom::new(6)); // carbon
    let c5 = caffeine.add_atom(Atom::new(6)); // carbon
    let c6 = caffeine.add_atom(Atom::new(6)); // carbon
    let n7 = caffeine.add_atom(Atom::new(7)); // nitrogen
    let c8 = caffeine.add_atom(Atom::new(6)); // carbon
    let n9 = caffeine.add_atom(Atom::new(7)); // nitrogen
    
    // Create carbonyl oxygens
    let o2 = caffeine.add_atom(Atom::new(8)); // oxygen at position 2
    let o6 = caffeine.add_atom(Atom::new(8)); // oxygen at position 6
    
    // Create methyl group carbons
    let c10 = caffeine.add_atom(Atom::new(6)); // methyl carbon on N1
    let c11 = caffeine.add_atom(Atom::new(6)); // methyl carbon on N3
    let c12 = caffeine.add_atom(Atom::new(6)); // methyl carbon on N7
    
    // Create methyl hydrogens (3 on each methyl carbon)
    // Methyl on N1 (C10)
    let h1 = caffeine.add_atom(Atom::new(1));
    let h2 = caffeine.add_atom(Atom::new(1));
    let h3 = caffeine.add_atom(Atom::new(1));
    
    // Methyl on N3 (C11)
    let h4 = caffeine.add_atom(Atom::new(1));
    let h5 = caffeine.add_atom(Atom::new(1));
    let h6 = caffeine.add_atom(Atom::new(1));
    
    // Methyl on N7 (C12)
    let h7 = caffeine.add_atom(Atom::new(1));
    let h8 = caffeine.add_atom(Atom::new(1));
    let h9 = caffeine.add_atom(Atom::new(1));
    
    // Hydrogen at position 8
    let h10 = caffeine.add_atom(Atom::new(1));
    
    // Create bonds for the six-membered ring
    caffeine.add_bond(n1, c2, BondType::Single).unwrap();
    caffeine.add_bond(c2, n3, BondType::Single).unwrap();
    caffeine.add_bond(n3, c4, BondType::Single).unwrap();
    caffeine.add_bond(c4, c5, BondType::Double).unwrap(); // Double bond
    caffeine.add_bond(c5, c6, BondType::Single).unwrap();
    caffeine.add_bond(c6, n1, BondType::Single).unwrap();
    
    // Create bonds for the five-membered ring
    caffeine.add_bond(c4, n9, BondType::Single).unwrap();
    caffeine.add_bond(n9, c8, BondType::Double).unwrap(); // Double bond
    caffeine.add_bond(c8, n7, BondType::Single).unwrap();
    caffeine.add_bond(n7, c5, BondType::Single).unwrap();
    
    // Carbonyl bonds
    caffeine.add_bond(c2, o2, BondType::Double).unwrap();
    caffeine.add_bond(c6, o6, BondType::Double).unwrap();
    
    // Connect methyl groups to the nitrogens
    caffeine.add_bond(n1, c10, BondType::Single).unwrap();
    caffeine.add_bond(n3, c11, BondType::Single).unwrap();
    caffeine.add_bond(n7, c12, BondType::Single).unwrap();
    
    // Connect hydrogens to methyl carbons
    caffeine.add_bond(c10, h1, BondType::Single).unwrap();
    caffeine.add_bond(c10, h2, BondType::Single).unwrap();
    caffeine.add_bond(c10, h3, BondType::Single).unwrap();
    
    caffeine.add_bond(c11, h4, BondType::Single).unwrap();
    caffeine.add_bond(c11, h5, BondType::Single).unwrap();
    caffeine.add_bond(c11, h6, BondType::Single).unwrap();
    
    caffeine.add_bond(c12, h7, BondType::Single).unwrap();
    caffeine.add_bond(c12, h8, BondType::Single).unwrap();
    caffeine.add_bond(c12, h9, BondType::Single).unwrap();
    
    // Connect hydrogen to carbon 8
    caffeine.add_bond(c8, h10, BondType::Single).unwrap();
    
    caffeine
}

fn create_methane() -> Molecule {
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
    
    methane
}

fn main() {
    // Create methane molecule
    println!("\n--- Creating Methane (CH4) ---");
    let methane = create_methane();
    println!("Molecule created: {}", methane.get_molecular_formula());
    println!("Number of atoms: {}", methane.atom_count());
    println!("Number of bonds: {}", methane.bond_count());
    methane.print_connectivity();
    println!("Molecular weight of methane: {:.3} g/mol", methane.get_molecular_weight());

    // Create caffeine molecule
    println!("\n--- Creating Caffeine (C8H10N4O2) ---");
    let caffeine = create_caffeine();
    println!("Molecule created: {}", caffeine.get_molecular_formula());
    println!("Number of atoms: {}", caffeine.atom_count());
    println!("Number of bonds: {}", caffeine.bond_count());
    caffeine.print_connectivity();

    // Calculate molecular weight
    println!("Molecular weight of caffeine: {:.3} g/mol", caffeine.get_molecular_weight());}