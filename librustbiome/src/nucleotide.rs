use std::cmp::{Eq, PartialEq};
use std::fmt::{Display, Error, Formatter};

use phf::{phf_map, Map};

use crate::buffer::SIMDBuffer;

static NUCLEOTIDE_VALUES: Map<char, u8> = phf_map! {
    'U' => 0x00u8,
    'C' => 0x01u8,
    'A' => 0x02u8,
    'G' => 0x03u8,
};

pub fn unwrap_nucleotide_value_get(key: char) -> Result<u8, Error> {
    match NUCLEOTIDE_VALUES.get(&key) {
        Some(value) => Ok(*value),
        None => Err(Error {}),
    }
}

static NUCLEOTIDE_NAMES: Map<u8, &'static str> = phf_map! {
    0x00u8 => "U",
    0x01u8 => "C",
    0x02u8 => "A",
    0x03u8 => "G",
};

pub fn unwrap_nucleotide_name_get(key: u8) -> Result<(&'static str), Error> {
    match NUCLEOTIDE_NAMES.get(&key) {
        Some(name) => Ok(name),
        None => Err(Error {}),
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Nucleotide {
    pub value: u8,
}

impl PartialEq for Nucleotide {
    fn eq(&self, other: &Self) -> bool {
        // This works by first determining if the two nucleotides differ at any binary point.
        // Then we trim the value to the only the first two bits (where we store relevant info).
        // If that isn't zero, then one of the two digits disagree.
        return ((self.value ^ other.value) & 0x3) == 0;
    }
}

impl Eq for Nucleotide {}

impl From<char> for Nucleotide {
    fn from(name: char) -> Self {
        match NUCLEOTIDE_VALUES.get(&name) {
            Some(value) => Nucleotide { value: *value },
            None => panic!("Invalid nucleotide name {}", name),
        }
    }
}

impl Display for Nucleotide {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", unwrap_nucleotide_name_get(self.value)?)
    }
}

struct NucleotideBuffer {
    buffer: SIMDBuffer,
}

#[cfg(test)]
mod tests {
    use crate::nucleotide::{Nucleotide, NUCLEOTIDE_NAMES};

    #[test]
    fn verify_nucleotide_map() {
        match NUCLEOTIDE_NAMES.get(&0x00) {
            Some(name) => assert_eq!(&"U", name),
            None => assert!(false),
        };
        match NUCLEOTIDE_NAMES.get(&0x01) {
            Some(name) => assert_eq!(&"C", name),
            None => assert!(false),
        };
        match NUCLEOTIDE_NAMES.get(&0x02) {
            Some(name) => assert_eq!(&"A", name),
            None => assert!(false),
        };
        match NUCLEOTIDE_NAMES.get(&0x03) {
            Some(name) => assert_eq!(&"G", name),
            None => assert!(false),
        };
    }

    #[test]
    fn verify_nucleotide_equality() {
        let u = Nucleotide::from('U');
        let u_clone = u.clone();
        let a = Nucleotide::from('A');
        let other_a = Nucleotide::from('A');
        let g = Nucleotide::from('G');

        assert_eq!(u, u_clone);
        assert_eq!(a, other_a);
        assert_ne!(a, g);
    }
}
