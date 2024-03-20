pub struct Allergies(Vec<u8>);

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
    Default,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let mut res: Vec<u8> = format!("{score:b}").into();
        res = res.into_iter().rev().collect();
        res.resize(8, 48);
        Self(res)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let idx = *allergen as usize;
        self.0[idx].eq(&49)
    }
    pub fn allergies(&self) -> Vec<Allergen> {
        self.0
            .iter()
            .enumerate()
            .map(|(i, c)| match (i, c) {
                (0, 49) => Allergen::Eggs,
                (1, 49) => Allergen::Peanuts,
                (2, 49) => Allergen::Shellfish,
                (3, 49) => Allergen::Strawberries,
                (4, 49) => Allergen::Tomatoes,
                (5, 49) => Allergen::Chocolate,
                (6, 49) => Allergen::Pollen,
                (7, 49) => Allergen::Cats,
                (_, 48) => Allergen::Default,
                (_, _) => Allergen::Default,
            })
            .filter(|s| s != &Allergen::Default)
            .collect()
    }
}
