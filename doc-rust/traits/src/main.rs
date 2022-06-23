/*
    Traits are similar to a feature often called interfaces in other languages,
    although with some differences.
*/

// In a game selling stuff can be considering as shared behavior between different race
pub trait Sell {
    fn sell_weapons(&self) -> String;
    fn sell_books(&self) -> String;
    fn sell_armor(&self) -> String;
}

pub struct LizardWarrior {
    pub level: u8,
    pub name: String,
}

pub struct TigerWarrior {
    pub level: u8,
    pub name: String,
}

impl Sell for LizardWarrior {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

fn main() {}
