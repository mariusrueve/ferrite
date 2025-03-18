use super::{Atom, Bond, BondType};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Molecule {
    atoms: Vec<Atom>,
    bonds: Vec<Bond>,
    // Optional: adjacency list for efficient neighbor lookups
    adjacency_list: Vec<Vec<usize>>,
}

impl Molecule {
    pub fn new() -> Self {
        Molecule {
            atoms: Vec::new(),
            bonds: Vec::new(),
            adjacency_list: Vec::new(),
        }
    }

    pub fn add_atom(&mut self, atom: Atom) -> usize {
        let idx = self.atoms.len();
        self.atoms.push(atom);
        self.adjacency_list.push(Vec::new());
        idx
    }

    pub fn add_bond(
        &mut self,
        begin_atom_idx: usize,
        end_atom_idx: usize,
        bond_type: BondType,
    ) -> Result<usize, &'static str> {
        if begin_atom_idx >= self.atoms.len() || end_atom_idx >= self.atoms.len() {
            return Err("Atom index out of bounds");
        }

        let bond = Bond::new(begin_atom_idx, end_atom_idx, bond_type);
        let bond_idx = self.bonds.len();
        self.bonds.push(bond);

        // Update adjacency list
        self.adjacency_list[begin_atom_idx].push(end_atom_idx);
        self.adjacency_list[end_atom_idx].push(begin_atom_idx);

        Ok(bond_idx)
    }

    pub fn atom_count(&self) -> usize {
        self.atoms.len()
    }

    pub fn bond_count(&self) -> usize {
        self.bonds.len()
    }

    pub fn get_atom(&self, idx: usize) -> Option<&Atom> {
        self.atoms.get(idx)
    }

    pub fn get_atom_mut(&mut self, idx: usize) -> Option<&mut Atom> {
        self.atoms.get_mut(idx)
    }

    pub fn get_bond(&self, idx: usize) -> Option<&Bond> {
        self.bonds.get(idx)
    }

    pub fn get_neighbors(&self, atom_idx: usize) -> Option<&[usize]> {
        self.adjacency_list.get(atom_idx).map(|v| v.as_slice())
    }

    pub fn get_molecular_formula(&self) -> String {
        let mut element_counts: HashMap<&str, usize> = HashMap::new();

        for atom in &self.atoms {
            *element_counts.entry(atom.symbol()).or_insert(0) += 1;
        }

        // Format the molecular formula (C first, then H, then others alphabetically)
        let mut formula = String::new();

        // Carbon first
        if let Some(&count) = element_counts.get("C") {
            formula.push_str("C");
            if count > 1 {
                formula.push_str(&count.to_string());
            }
        }

        // Hydrogen second
        if let Some(&count) = element_counts.get("H") {
            formula.push_str("H");
            if count > 1 {
                formula.push_str(&count.to_string());
            }
        }

        // Other elements alphabetically
        let mut others: Vec<_> = element_counts
            .iter()
            .filter(|&(symbol, _)| *symbol != "C" && *symbol != "H")
            .collect();
        others.sort_by_key(|&(symbol, _)| *symbol);

        for (symbol, count) in others {
            formula.push_str(symbol);
            if *count > 1 {
                formula.push_str(&count.to_string());
            }
        }

        formula
    }

    pub fn print_connectivity(&self) {
        println!("Molecule Connectivity:");
        println!("Atoms:");
        for (idx, atom) in self.atoms.iter().enumerate() {
            println!(
                "  [{}] {} (atomic number: {})",
                idx,
                atom.symbol(),
                atom.atomic_number
            );
        }

        println!("\nBonds:");
        for (idx, bond) in self.bonds.iter().enumerate() {
            let begin_atom = &self.atoms[bond.begin_atom_idx];
            let end_atom = &self.atoms[bond.end_atom_idx];
            let bond_symbol = match bond.bond_type {
                BondType::Single => "-",
                BondType::Double => "=",
                BondType::Triple => "≡",
                BondType::Aromatic => "~",
            };

            println!(
                "  [{}] {}({}) {} {}({})",
                idx,
                begin_atom.symbol(),
                bond.begin_atom_idx,
                bond_symbol,
                end_atom.symbol(),
                bond.end_atom_idx
            );
        }

        println!("\nConnectivity:");
        for (atom_idx, neighbors) in self.adjacency_list.iter().enumerate() {
            if !neighbors.is_empty() {
                print!("  {}({}) → ", self.atoms[atom_idx].symbol(), atom_idx);

                let connections: Vec<String> = neighbors
                    .iter()
                    .map(|&n| format!("{}({})", self.atoms[n].symbol(), n))
                    .collect();

                println!("{}", connections.join(", "));
            }
        }

        println!("\nMolecular Formula: {}", self.get_molecular_formula());
    }
    /// Returns the atomic weight for a given atomic number in g/mol
    fn get_atomic_weight(atomic_number: u8) -> f64 {
        match atomic_number {
            1 => 1.008,    // H
            6 => 12.011,   // C
            7 => 14.007,   // N
            8 => 15.999,   // O
            9 => 18.998,   // F
            15 => 30.974,  // P
            16 => 32.065,  // S
            17 => 35.453,  // Cl
            35 => 79.904,  // Br
            53 => 126.904, // I
            // Add more elements as needed
            _ => 0.0, // Default case for unknown elements
        }
    }

    /// Calculates the molecular weight of the molecule in g/mol
    pub fn get_molecular_weight(&self) -> f64 {
        self.atoms
            .iter()
            .map(|atom| Self::get_atomic_weight(atom.atomic_number))
            .sum()
    }
}
