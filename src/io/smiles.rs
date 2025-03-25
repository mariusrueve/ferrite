use crate::Molecule;

pub fn smiles_parser(smiles: &str) -> Molecule {
    print!("{}", smiles);
    let mol = Molecule::new();
    mol
}

#[test]
fn test_smiles_parser() {
    let caffeine_smiles = "CN1C=NC2=C1C(=O)N(C(=O)N2C)C";
    let caffeine_mol = smiles_parser(caffeine_smiles);
    
    // Verify the molecule was created successfully
    assert!(caffeine_mol.is_ok(), "Failed to parse caffeine SMILES");
    let caffeine_mol = caffeine_mol.unwrap();
    
    // Check basic structural properties
    assert_eq!(caffeine_mol.atom_count(), 14, "Expected 14 atoms in caffeine");
    
    // Verify chemical composition
    assert_eq!(caffeine_mol.formula(), "C8H10N4O2", "Incorrect molecular formula");
    
    // Verify ring structure (caffeine has 2 rings)
    assert_eq!(caffeine_mol.ring_count(), 2, "Caffeine should have 2 rings");
    
    // Test specific atom types
    let c_count = caffeine_mol.atoms_of_element("C").count();
    let n_count = caffeine_mol.atoms_of_element("N").count();
    let o_count = caffeine_mol.atoms_of_element("O").count();
    
    assert_eq!(c_count, 8, "Should have 8 carbon atoms");
    assert_eq!(n_count, 4, "Should have 4 nitrogen atoms");
    assert_eq!(o_count, 2, "Should have 2 oxygen atoms");
}