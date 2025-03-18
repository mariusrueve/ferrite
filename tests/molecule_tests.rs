use ferrite::{Atom, BondType, Molecule};

#[test]
fn test_molecule_creation() {
    let mut mol = Molecule::new();
    assert_eq!(mol.atom_count(), 0);
    assert_eq!(mol.bond_count(), 0);
    
    let a1 = mol.add_atom(Atom::new(6)); // Carbon
    let a2 = mol.add_atom(Atom::new(8)); // Oxygen
    
    assert_eq!(mol.atom_count(), 2);
    mol.add_bond(a1, a2, BondType::Single).unwrap();
    assert_eq!(mol.bond_count(), 1);
}

#[test]
fn test_molecular_formula() {
    // Create water (H2O)
    let mut water = Molecule::new();
    let o_idx = water.add_atom(Atom::new(8));
    let h1_idx = water.add_atom(Atom::new(1));
    let h2_idx = water.add_atom(Atom::new(1));
    
    water.add_bond(o_idx, h1_idx, BondType::Single).unwrap();
    water.add_bond(o_idx, h2_idx, BondType::Single).unwrap();
    
    assert_eq!(water.get_molecular_formula(), "H2O");
}

#[test]
fn test_molecular_weight() {
    // Create methane (CH4)
    let mut methane = Molecule::new();
    let c_idx = methane.add_atom(Atom::new(6));
    
    for _ in 0..4 {
        let h_idx = methane.add_atom(Atom::new(1));
        methane.add_bond(c_idx, h_idx, BondType::Single).unwrap();
    }
    
    // Expected weight: C (12.011) + 4*H (4*1.008) = 16.043
    let expected_weight = 16.043;
    let calculated_weight = methane.get_molecular_weight();
    
    assert!((calculated_weight - expected_weight).abs() < 0.001);
}