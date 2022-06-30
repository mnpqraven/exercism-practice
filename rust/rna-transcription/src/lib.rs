#[derive(Debug, PartialEq)]
pub struct Dna {
    a: String,
    c: String,
    g: String,
    t: String
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    c: String,
    g: String,
    a: String,
    u: String
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        unimplemented!("Construct new Dna from '{}' string. If string contains invalid nucleotides return index of first invalid nucleotide", dna);
    }

    pub fn into_rna(self) -> Rna {
        unimplemented!("Transform Dna {:?} into corresponding Rna", self);
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        unimplemented!("Construct new Rna from '{}' string. If string contains invalid nucleotides return index of first invalid nucleotide", rna);
    }
}
