use crate::resources::item::{EqpbleType, Equippable, new_equippable};

const EXP_MAX: u32 = 1000000;

pub struct Combatant {
    name: String,
    level: u8,
    exp: u32,
    attributes: Attributes,
    equipment: Equipments,
}
impl Combatant {
    pub fn new(name: &str) -> Combatant {
        Combatant {
            name: name.into(),
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

    fn get_hp(&mut self) -> i16 {
        self.equipment.weapon.effects.hp
            .saturating_add(self.equipment.head.effects.hp)
            .saturating_add(self.equipment.body.effects.hp)
            .saturating_add(self.equipment.accessory[0].effects.hp)
            .saturating_add(self.equipment.accessory[1].effects.hp)
            .saturating_add(self.equipment.accessory[2].effects.hp)
            .saturating_add(self.equipment.accessory[3].effects.hp)
    }

    fn get_strength(&mut self) -> i8 {
        self.equipment.weapon.effects.strength
            .saturating_add(self.equipment.head.effects.strength)
            .saturating_add(self.equipment.body.effects.strength)
            .saturating_add(self.equipment.accessory[0].effects.strength)
            .saturating_add(self.equipment.accessory[1].effects.strength)
            .saturating_add(self.equipment.accessory[2].effects.strength)
            .saturating_add(self.equipment.accessory[3].effects.strength)
    }

    fn get_constitution(&mut self) -> i8 {
        self.equipment.weapon.effects.constitution
            .saturating_add(self.equipment.head.effects.constitution)
            .saturating_add(self.equipment.body.effects.constitution)
            .saturating_add(self.equipment.accessory[0].effects.constitution)
            .saturating_add(self.equipment.accessory[1].effects.constitution)
            .saturating_add(self.equipment.accessory[2].effects.constitution)
            .saturating_add(self.equipment.accessory[3].effects.constitution)
    }

    fn get_intelligence(&mut self) -> i8 {
        self.equipment.weapon.effects.intelligence
            .saturating_add(self.equipment.head.effects.intelligence)
            .saturating_add(self.equipment.body.effects.intelligence)
            .saturating_add(self.equipment.accessory[0].effects.intelligence)
            .saturating_add(self.equipment.accessory[1].effects.intelligence)
            .saturating_add(self.equipment.accessory[2].effects.intelligence)
            .saturating_add(self.equipment.accessory[3].effects.intelligence)
    }

    fn get_speed(&mut self) -> i8 {
        self.equipment.weapon.effects.speed
            .saturating_add(self.equipment.head.effects.speed)
            .saturating_add(self.equipment.body.effects.speed)
            .saturating_add(self.equipment.accessory[0].effects.speed)
            .saturating_add(self.equipment.accessory[1].effects.speed)
            .saturating_add(self.equipment.accessory[2].effects.speed)
            .saturating_add(self.equipment.accessory[3].effects.speed)
    }

    fn get_luck(&mut self) -> i8 {
        self.equipment.weapon.effects.luck
            .saturating_add(self.equipment.head.effects.luck)
            .saturating_add(self.equipment.body.effects.luck)
            .saturating_add(self.equipment.accessory[0].effects.luck)
            .saturating_add(self.equipment.accessory[1].effects.luck)
            .saturating_add(self.equipment.accessory[2].effects.luck)
            .saturating_add(self.equipment.accessory[3].effects.luck)
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
            weapon: new_equippable("none".into()),
            head: new_equippable("none".into()),
            body: new_equippable("none".into()),
            accessory: [
                new_equippable("none".into()),
                new_equippable("none".into()),
                new_equippable("none".into()),
                new_equippable("none".into()),
            ],
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
