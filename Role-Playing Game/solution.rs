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
        if self.health < 1 {
            let new_player: Player = Player {
                health: 100,
                mana: None,
                level: self.level,
            };
            if self.level >= 10 {
                let new_player: Player = Player {
                    health: 100,
                    mana: Some(100),
                    level: self.level,
                };
                return Some(new_player);
            }
            return Some(new_player);
        }
        None
    }
    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(amount) => {
                if amount < mana_cost {
                    return 0;
                }
                self.mana = Some(self.mana.unwrap() - mana_cost);
                mana_cost * 2
            }
            None => {
                if mana_cost > self.health {
                    self.health = 0;
                    return 0;
                }
                self.health -= mana_cost;
                return 0;
            }
            _ => 0,
        }
    }
}
