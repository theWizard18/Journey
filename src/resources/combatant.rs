use crate::resources::item::{EqpbleType, Equippable};
use crate::resources::party::Party;

const EXP_MAX: u32 = 1000000;

pub struct Combatant {
    name: String,
    party: Option<Party>,
    level: u8,
    exp: u32,
    attributes: Attributes,
    equipment: Equipments,
}
impl Combatant {
    pub fn new(name: &str) -> Combatant {
        Combatant {
            name: name.into(),
            party: None,
            level: 1,
            exp: 0,
            attributes: Attributes::new(),
            equipment: Equipments::new(),
        }
    }

    fn lv_up_threshold(&self) -> u32 {
        (self.level as u32 + 1).pow(3)
    }

    fn increment_exp_by(&mut self, num: u32) {
        let sum = self.exp + num;
        self.exp = match sum <= EXP_MAX {
            true => sum,
            false => EXP_MAX,
        };
        while self.exp > self.lv_up_threshold() {
            self.level_up();
        }
    }

    fn level_up(&mut self) {
        self.level += 1;
    }
}

struct Attributes {
    hp: u16,
    strength: u8,
    constitution: u8,
    intelligence: u8,
    speed: u8,
    luck: u8,
}
impl Attributes {
    fn new() -> Attributes {
        Attributes {
            hp: 80,
            strength: 20,
            constitution: 15,
            intelligence: 15,
            speed: 18,
            luck: 15,
        }
    }
}

struct Equipments {
    weapon: Equippable,
    head: Equippable,
    body: Equippable,
    accessory: [Equippable; 4],
}
impl Equipments {
    fn new() -> Equipments {
        Equipments {
            weapon: Equippable::none(),
            head: Equippable::none(),
            body: Equippable::none(),
            accessory: [Equippable::none(); 4],
        }
    }

    fn equip_weapon(&mut self, item: &Equippable) {
        let buffer = self.weapon.clone();
        match item.equip_type {
            EqpbleType::Weapon => self.weapon = item.clone(),
            _ => println!("This item cannot be equipped as a Weapon"),
        };
    }
}
