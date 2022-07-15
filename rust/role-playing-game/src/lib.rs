// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            match self.level >= 10 {
                true => Some(Player { health: 100, mana: Some(100), level: self.level }),
                _ => Some(Player {health: 100, mana: None, level: self.level})
            }
        } else { None }
    }

    /// try to cast a spell based doing 2x damage of the mana cost
    /// NOTE: exceptions:
    /// doesn't have any mana > hp is reduced by the same amount as mana cost
    /// damage dealt is 0
    ///
    /// not enough mana, mana pool stays the same, damage dealt is 0
    /// * `mana_cost`: mana cost
    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {

        match self.mana {
            Some(pool) if pool > mana_cost => {
                self.mana = Some(pool - mana_cost);
                2 * mana_cost
            },
            Some(pool) if pool < mana_cost => 0,
            _ => {
                self.health -= std::cmp::min(self.health, mana_cost);
                0
            }
        }
    }
}
