use std::fmt::{Display, Error, Formatter};

use phf::{Map, phf_map};

static NUCLEOTIDE_NAMES: Map<u8, &'static str> = phf_map! {
    0x00u8 => "U",
    0x01u8 => "C",
    0x02u8 => "A",
    0x03u8 => "G",
};

struct Nucleotide
{
    pub value: u8,
}

impl Display for Nucleotide
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match NUCLEOTIDE_NAMES.get(&self.value) {
            Some(name) => write!(f, "{}", name),
            None => Err(Error {})
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::NUCLEOTIDE_NAMES;

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
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
