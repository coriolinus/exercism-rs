#[derive(Debug, Clone, Copy, PartialEq)]
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

impl Allergen {
    fn iter() -> std::vec::IntoIter<Allergen> {
        vec![
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ].into_iter()
    }
}

pub struct Allergies {
    score: u8,
}

impl Allergies {
    pub fn new(score: u8) -> Allergies {
        Allergies {score: score}
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score & *allergen as u8 != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::iter().filter(|allergen| self.is_allergic_to(allergen)).collect()
    }
}
