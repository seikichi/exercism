use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    map: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.map.get(codon).cloned()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut polypeptide = vec![];
        for i in (0..rna.len()).step_by(3) {
            match self.name_for(&rna[i..i + 3]) {
                None => return None,
                Some("stop codon") => return Some(polypeptide),
                Some(name) => polypeptide.push(name),
            }
        }
        Some(polypeptide)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let map = pairs.into_iter().collect();
    CodonsInfo { map }
}
