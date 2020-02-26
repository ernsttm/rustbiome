use std::cmp::{max, min, Eq, PartialEq};
use std::fmt::{Display, Error, Formatter};

use crate::buffer::SIMDBuffer;
use crate::genetic_code::{GeneticCode, StandardCode};
use crate::nucleotide::{unwrap_nucleotide_name_get, unwrap_nucleotide_value_get, Nucleotide};

enum Position {
    FIRST,
    SECOND,
    THIRD,
    FOURTH,
}

impl Position {
    fn get(nucleotide_position: usize) -> Position {
        let moduloed_position = nucleotide_position % 4;
        match moduloed_position {
            0 => Position::FIRST,
            1 => Position::SECOND,
            2 => Position::THIRD,
            3 => Position::FOURTH,
            _ => panic!("Rust's modulo is broken, AAHHHHHHH! {}", moduloed_position),
        }
    }
}

fn get_nucleotide_value(nucleotides: u8, position: Position) -> u8 {
    match position {
        Position::FIRST => nucleotides & 0x3,
        Position::SECOND => (nucleotides >> 2) & 0x3,
        Position::THIRD => (nucleotides >> 4) & 0x3,
        Position::FOURTH => (nucleotides >> 6) & 0x3,
    }
}

pub struct Codon {
    nucleotides: u8,
}

impl Codon {
    pub fn from_nucleotides(a: &Nucleotide, b: &Nucleotide, c: &Nucleotide) -> Codon {
        Codon {
            nucleotides: 0u8 | a.value | (b.value << 2) | (c.value << 4),
        }
    }

    pub fn from_nucleotide_array(nucleotides: &[Nucleotide; 3]) -> Codon {
        Codon::from_nucleotides(&nucleotides[0], &nucleotides[1], &nucleotides[2])
    }

    pub fn from(a: u8, b: u8, c: u8) -> Codon {
        Codon {
            nucleotides: 0u8 | a | (b << 2) | (c << 4),
        }
    }

    pub fn to_nucleotides(&self) -> [Nucleotide; 3] {
        [
            Nucleotide {
                value: get_nucleotide_value(self.nucleotides, Position::FIRST),
            },
            Nucleotide {
                value: get_nucleotide_value(self.nucleotides, Position::SECOND),
            },
            Nucleotide {
                value: get_nucleotide_value(self.nucleotides, Position::THIRD),
            },
        ]
    }
}

impl Display for Codon {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let a =
            unwrap_nucleotide_name_get(get_nucleotide_value(self.nucleotides, Position::FIRST))?;
        let b =
            unwrap_nucleotide_name_get(get_nucleotide_value(self.nucleotides, Position::SECOND))?;
        let c =
            unwrap_nucleotide_name_get(get_nucleotide_value(self.nucleotides, Position::THIRD))?;

        write!(f, "{}{}{}", a, b, c)
    }
}

const NUM_NUCLEOTIDES_IN_U8: usize = 4;

fn populate_value(input_chars: &[char]) -> u8 {
    let mut value: u8 = 0;

    let max = min(input_chars.len(), NUM_NUCLEOTIDES_IN_U8);
    for index in 0..max {
        value |= match unwrap_nucleotide_value_get(input_chars[index]) {
            Ok(value) => value << (index as u8 * 2),
            Err(e) => panic!(
                "Invalid nucleotide character in input: {}",
                input_chars[index]
            ),
        };
    }

    value
}

struct CodonBuffer<T: GeneticCode> {
    buffer: SIMDBuffer,
    count: usize,
    protein_translator: T,
}

impl<T: GeneticCode> CodonBuffer<T> {
    pub fn new(buffer: SIMDBuffer, count: usize) -> Self {
            CodonBuffer {
                buffer: buffer,
                count: count,
                protein_translator: T::new(),
            }
    }

    pub fn to_codons(&self) -> Vec<Codon> {
        let buffer_view = self.buffer.as_slice();
        let mut codons: Vec<Codon> = Vec::new();
        codons.reserve(self.count % 3);
        let mut nucleotides = [Nucleotide { value: 0 }; 3];
        for nucleotide_count in 0..self.count {
            nucleotides[nucleotide_count % 3] = Nucleotide {
                value: get_nucleotide_value(
                    buffer_view[nucleotide_count / 4],
                    Position::get((nucleotide_count)),
                ),
            };

            if nucleotide_count % 3 == 2 {
                codons.push(Codon::from_nucleotide_array(&nucleotides))
            }
        }

        codons
    }
}

impl<T: GeneticCode> From<&[char]> for CodonBuffer<T> {
    fn from(input: &[char]) -> Self {
        // Require the input slice is divisible by three and thus can be split into codons.
        if input.len() % 3 != 0 {
            panic!(
                "Number of nucleotides cannot be formed into codon: {}",
                input.len()
            )
        }

        let mut storage_buffer: Vec<u8> = Vec::new();
        storage_buffer.reserve(input.len() / 4 + 1);

        for index in (0..input.len()).step_by(4) {
            storage_buffer.push(populate_value(&input[index..]));
        }

        CodonBuffer::new(SIMDBuffer {buffer: storage_buffer}, input.len())
    }
}

impl<T: GeneticCode> Display for CodonBuffer<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        // Translate the codon buffer into Codons, and map them into their string representations.
        let codon_output: Vec<String> = self
            .to_codons()
            .into_iter()
            .map(|c| format!("{}", c))
            .collect();

        // Output the codons with a comment separating them.
        write!(f, "[{}]", codon_output.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use crate::codon::{Codon, CodonBuffer};
    use crate::nucleotide::Nucleotide;
    use crate::genetic_code::StandardCode;

    #[test]
    fn verify_codon() {
        let a = Nucleotide::from('A');
        let u = Nucleotide::from('U');
        let g = Nucleotide::from('G');

        let codon = Codon::from_nucleotides(&a, &u, &g);
        let retrieved_nucleotides = codon.to_nucleotides();

        assert_eq!(a, retrieved_nucleotides[0]);
        assert_eq!(u, retrieved_nucleotides[1]);
        assert_eq!(g, retrieved_nucleotides[2]);
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_codon_buffer_simple() {
        let input_buffer = ['A', 'U', 'G', 'G', 'A', 'C'];
        let codon_buffer: CodonBuffer<StandardCode> = CodonBuffer::from(&input_buffer[..]);

        assert_eq!("[AUG, GAC]", format!("{}", codon_buffer));
    }
}
