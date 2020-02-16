use std::cmp::{Eq, PartialEq};
use std::fmt::{Display, Error, Formatter};

use phf::{Map, phf_map};

static NUCLEOTIDE_VALUES: Map<char, u8> = phf_map! {
    'U' => 0x00u8,
    'C' => 0x01u8,
    'A' => 0x02u8,
    'G' => 0x03u8,
};

static NUCLEOTIDE_NAMES: Map<u8, &'static str> = phf_map! {
    0x00u8 => "U",
    0x01u8 => "C",
    0x02u8 => "A",
    0x03u8 => "G",
};

fn unwrap_nucleotide_name_get(key: u8) -> Result<(&'static str), Error> {
    match NUCLEOTIDE_NAMES.get(&key) {
        Some(name) => Ok(name),
        None => Err(Error{})
    }
}

#[derive(Clone, Debug)]
struct Nucleotide
{
    pub value: u8,
}

impl PartialEq for Nucleotide
{
    fn eq(&self, other: &Self) -> bool
    {
        // This works by first determining if the two nucleotides differ at an binary point.
        // Then we trim the value to the only the first two bits (where we store relevant info).
        // If that isn't zero, then one of the two digits disagree.
        return ((self.value ^ other.value) & 0x3) == 0
    }
}

impl Eq for Nucleotide {}

impl From<char> for Nucleotide
{
    // TODO: Handle an invalid name better
    fn from(name: char) -> Self
    {
        match NUCLEOTIDE_VALUES.get(&name) {
            Some(value) => Nucleotide{value: *value},
            None => panic!("Invalid nucleotide name {}", name),
        }
    }
}

impl Display for Nucleotide
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", unwrap_nucleotide_name_get(self.value)?)
    }
}

enum Position
{
    FIRST,
    SECOND,
    THIRD,
}

struct Codon
{
    nucleotides: u8,
}

impl Codon
{
    pub fn from(a: &Nucleotide, b: &Nucleotide, c: &Nucleotide) -> Codon
    {
        Codon{nucleotides: 0u8 | a.value | (b.value << 2) | (c.value << 4)}
    }

    pub fn to_nucleotides(&self) -> [Nucleotide; 3]
    {
        print!("{}", self);
        [Nucleotide{value: self.get_nucleotide_value(Position::FIRST)}, Nucleotide{value: self.get_nucleotide_value(Position::SECOND)}, Nucleotide{value: self.get_nucleotide_value(Position::THIRD)}]
    }

    fn get_nucleotide_value(&self, position: Position) -> u8
    {
        match position {
            Position::FIRST => self.nucleotides & 0x3,
            Position::SECOND => (self.nucleotides >> 2) & 0x3,
            Position::THIRD => (self.nucleotides >> 4) & 0x3,
        }
    }
}

impl Display for Codon
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let a = unwrap_nucleotide_name_get(self.nucleotides & 0x3)?;
        let b = unwrap_nucleotide_name_get((self.nucleotides >> 2) & 0x3)?;
        let c = unwrap_nucleotide_name_get((self.nucleotides >> 4) & 0x3)?;

        write!(f, "{}{}{}", a, b, c)
    }
}

#[cfg(test)]
mod tests {
    use crate::codon::{NUCLEOTIDE_NAMES, Codon, Nucleotide};

    #[test]
    fn verify_nucleotide_map() {
        match NUCLEOTIDE_NAMES.get(&0x00) {
            Some(name) => assert_eq!(&"U", name),
            None => assert!(false)
        };
        match NUCLEOTIDE_NAMES.get(&0x01) {
            Some(name) => assert_eq!(&"C", name),
            None => assert!(false)
        };
        match NUCLEOTIDE_NAMES.get(&0x02) {
            Some(name) => assert_eq!(&"A", name),
            None => assert!(false)
        };
        match NUCLEOTIDE_NAMES.get(&0x03) {
            Some(name) => assert_eq!(&"G", name),
            None => assert!(false)
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

    #[test]
    fn verify_codon() {
        let a = Nucleotide::from('A');
        let u = Nucleotide::from('U');
        let g = Nucleotide::from('G');

        let codon = Codon::from(&a, &u, &g);
        let retrieved_nucleotides = codon.to_nucleotides();

        assert_eq!(a, retrieved_nucleotides[0]);
        assert_eq!(u, retrieved_nucleotides[1]);
        assert_eq!(g, retrieved_nucleotides[2]);
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
