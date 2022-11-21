use crate::resources::item;

const EXP_MAX: u32 = 1000000;

pub struct Combatant {
    name: String,
    level: u8,
    exp: u32,
    attributes: Attributes,
    equipment: Equipments,
}
impl Combatant {
    pub fn new(name: &str) -> Self {
        Self {
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
        self.level.saturating_add(1);
    }

    fn health(&self) -> u16 {
        let sum = self.equipment.weapon_a.mod_attr.health
            .saturating_add(self.equipment.weapon_b.mod_attr.health)
            .saturating_add(self.equipment.armor.mod_attr.health)
            .saturating_add(self.equipment.accessory_a.mod_attr.health)
            .saturating_add(self.equipment.accessory_b.mod_attr.health);
        let attr = self.attributes.base_health;
        self.hp_sp_mod(attr, sum)
    }

    fn skill(&self) -> u16 {
        let sum = self.equipment.weapon_a.mod_attr.skill
            .saturating_add(self.equipment.weapon_b.mod_attr.skill)
            .saturating_add(self.equipment.armor.mod_attr.skill)
            .saturating_add(self.equipment.accessory_a.mod_attr.skill)
            .saturating_add(self.equipment.accessory_b.mod_attr.skill);
        let attr = self.attributes.base_skill;
        self.hp_sp_mod(attr, sum)
    }

    fn strength(&self) -> u8 {
        let sum = self.equipment.weapon_a.mod_attr.strength
            .saturating_add(self.equipment.weapon_b.mod_attr.strength)
            .saturating_add(self.equipment.armor.mod_attr.strength)
            .saturating_add(self.equipment.accessory_a.mod_attr.strength)
            .saturating_add(self.equipment.accessory_b.mod_attr.strength);
        let attr = self.attributes.base_strength;
        self.attr_mod(attr, sum)
    }

    fn constitution(&self) -> u8 {
        let sum = self.equipment.weapon_a.mod_attr.constitution
            .saturating_add(self.equipment.weapon_b.mod_attr.constitution)
            .saturating_add(self.equipment.armor.mod_attr.constitution)
            .saturating_add(self.equipment.accessory_a.mod_attr.constitution)
            .saturating_add(self.equipment.accessory_b.mod_attr.constitution);
        let attr = self.attributes.base_constitution;
        self.attr_mod(attr, sum)
    }

    fn intelligence(&self) -> u8 {
        let sum = self.equipment.weapon_a.mod_attr.intelligence
            .saturating_add(self.equipment.weapon_b.mod_attr.intelligence)
            .saturating_add(self.equipment.armor.mod_attr.intelligence)
            .saturating_add(self.equipment.accessory_a.mod_attr.intelligence)
            .saturating_add(self.equipment.accessory_b.mod_attr.intelligence);
        let attr = self.attributes.base_intelligence;
        self.attr_mod(attr, sum)
    }

    fn luck(&self) -> u8 {
        let sum = self.equipment.weapon_a.mod_attr.luck
            .saturating_add(self.equipment.weapon_b.mod_attr.luck)
            .saturating_add(self.equipment.armor.mod_attr.luck)
            .saturating_add(self.equipment.accessory_a.mod_attr.luck)
            .saturating_add(self.equipment.accessory_b.mod_attr.luck);
        let attr = self.attributes.base_luck;
        self.attr_mod(attr, sum)
    }

    fn accuracy(&self) -> u8 {
        let sum = self.equipment.weapon_a.mod_attr.accuracy
            .saturating_add(self.equipment.weapon_b.mod_attr.accuracy)
            .saturating_add(self.equipment.armor.mod_attr.accuracy)
            .saturating_add(self.equipment.accessory_a.mod_attr.accuracy)
            .saturating_add(self.equipment.accessory_b.mod_attr.accuracy);
        let attr = self.attributes.base_accuracy;
        self.attr_mod(attr, sum)
    }

    fn speed(&self) -> u8 {
        let sum = self.equipment.weapon_a.mod_attr.speed
            .saturating_add(self.equipment.weapon_b.mod_attr.speed)
            .saturating_add(self.equipment.armor.mod_attr.speed)
            .saturating_add(self.equipment.accessory_a.mod_attr.speed)
            .saturating_add(self.equipment.accessory_b.mod_attr.speed);
        let attr = self.attributes.base_speed;
        self.attr_mod(attr, sum)
    }

    fn attr_mod(&self, attr: u8, modify: i8) -> u8 {
        match modify < 0 {
            false => attr.saturating_add(modify as u8),
            true => attr.saturating_sub(-modify as u8),
        }
    }

    fn hp_sp_mod(&self, attr: u16, modify: i16) -> u16 {
        match modify < 0 {
            false => attr.saturating_add(modify as u16),
            true => attr.saturating_sub(-modify as u16),
        }
    }
}

struct Attributes {
    base_health: u16,
    current_health: u16,
    base_skill: u16,
    current_skill: u16,
    base_strength: u8,
    base_constitution: u8,
    base_intelligence: u8,
    base_luck: u8,
    base_accuracy: u8,
    base_speed: u8,
}
impl Attributes {
    fn new() -> Self {
        Self {
            base_health: 80,
            current_health: 80,
            base_skill: 14,
            current_skill: 14,
            base_strength: 20,
            base_constitution: 15,
            base_intelligence: 15,
            base_luck: 15,
            base_accuracy: 20,
            base_speed: 18,
        }
    }
}

struct Equipments {
    weapon_a: item::Weapon,
    weapon_b: item::Weapon,
    armor: item::Armor,
    accessory_a: item::Accessory,
    accessory_b: item::Accessory,
}
impl Equipments {
    fn new() -> Self {
        Self {
            weapon_a: item::new_weapon("none"),
            weapon_b: item::new_weapon("none"),
            armor: item::new_armor("none"),
            accessory_a: item::new_accessory("none"),
            accessory_b: item::new_accessory("none"),
        }
    }

    fn equip_weapon(&mut self, item: &item::Weapon) {
    }
}
