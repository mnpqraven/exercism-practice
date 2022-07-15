pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    /// "Given the '{}' score, construct a new Allergies struct.",
    /// * `score`:
    pub fn new(score: u32) -> Self {
        Allergies { score: score % 256 }
    }

    /// "Determine if the patient is allergic to the '{:?}' allergen.",
    /// * `allergen`:
    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies().contains(allergen)
    }

    /// Return the list of allergens contained within the score with which the Allergies struct was made.;
    pub fn allergies(&self) -> Vec<Allergen> {
        let mut score = self.score as i32;
        [
            Allergen::Eggs,
            Allergen::Peanuts ,
            Allergen::Shellfish ,
            Allergen::Strawberries ,
            Allergen::Tomatoes ,
            Allergen::Chocolate ,
            Allergen::Pollen ,
            Allergen::Cats,
            // iter going from 1 to 128
        ].iter().rev().filter_map(|allergen| {
            // get value of allergen
            let numcompare = *allergen as i32;
            if score - numcompare >= 0 {
                score -= numcompare;
                Some(*allergen)
            } else { None }
        }).collect()
    }
}
