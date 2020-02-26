use crate::codon::Codon;

struct Protein {
    name: String,
    code: char,
    initiator: bool,
    terminator: bool,
}

pub trait GeneticCode {
    fn new() -> Self;
    fn translate(codons: &[Codon]) -> Vec<Protein>;
}

pub struct StandardCode { }

impl GeneticCode for StandardCode {
    fn new() -> Self {
        StandardCode {}
    }

    fn translate(codons: &[Codon]) -> Vec<Protein> {
        Vec::new()
    }
}