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
        if self.health.eq(&0) {
            let mana = if self.level.ge(&10) {Some(100)} else {None}; 
            Some(Player{health:100, mana, level:self.level,})
        } else { None}
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if self.level.lt(&10) {
            let (sub, flag) = self.health.overflowing_sub(mana_cost);
            self.health = if flag {0} else {sub};
            0
        } else if self.mana.unwrap_or_default().lt(&mana_cost) {
            0
        } 
        else {        
            self.mana = self.mana.and_then(|mana| Some (mana - mana_cost));
            mana_cost * 2
        }
    }
}
