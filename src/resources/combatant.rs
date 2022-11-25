use crate::resources::item;
use rand::{thread_rng, Rng};

const EXP_MAX: u32 = 1000000;

pub struct Combatant {
    name: String,
    level: u8,
    exp: u32,
    attributes: Attributes,
    equipment: Equipments,
    proficiencies: Proficiencies,
}
impl Combatant {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            level: 1,
            exp: 0,
            attributes: Attributes::new(),
            equipment: Equipments::new(),
            proficiencies: Proficiencies::new(),
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
        self.attributes.base_health.increment();
        self.attributes.base_skill.increment();
        self.attributes.base_strength.increment();
        self.attributes.base_constitution.increment();
        self.attributes.base_intelligence.increment();
        self.attributes.base_luck.increment();
        self.attributes.base_speed.increment();
        self.attributes.base_accuracy.increment();
    }

    fn health(&self) -> u16 {
        let sum = self.equipment.weapon_a.mod_attr.health
            .saturating_add(self.equipment.weapon_b.mod_attr.health)
            .saturating_add(self.equipment.armor.mod_attr.health)
            .saturating_add(self.equipment.accessory_a.mod_attr.health)
            .saturating_add(self.equipment.accessory_b.mod_attr.health);
        let attr = self.attributes.base_health;
        attr.attr_mod(sum)
    }

    fn skill(&self) -> u16 {
        let sum = self.equipment.weapon_a.mod_attr.skill
            .saturating_add(self.equipment.weapon_b.mod_attr.skill)
            .saturating_add(self.equipment.armor.mod_attr.skill)
            .saturating_add(self.equipment.accessory_a.mod_attr.skill)
            .saturating_add(self.equipment.accessory_b.mod_attr.skill);
        let attr = self.attributes.base_skill;
        attr.attr_mod(sum)
    }

    fn strength(&self) -> u8 {
        let sum = self.equipment.weapon_a.mod_attr.strength
            .saturating_add(self.equipment.weapon_b.mod_attr.strength)
            .saturating_add(self.equipment.armor.mod_attr.strength)
            .saturating_add(self.equipment.accessory_a.mod_attr.strength)
            .saturating_add(self.equipment.accessory_b.mod_attr.strength);
        let attr = self.attributes.base_strength;
        attr.attr_mod(sum)
    }

    fn constitution(&self) -> u8 {
        let sum = self.equipment.weapon_a.mod_attr.constitution
            .saturating_add(self.equipment.weapon_b.mod_attr.constitution)
            .saturating_add(self.equipment.armor.mod_attr.constitution)
            .saturating_add(self.equipment.accessory_a.mod_attr.constitution)
            .saturating_add(self.equipment.accessory_b.mod_attr.constitution);
        let attr = self.attributes.base_constitution;
        attr.attr_mod(sum)
    }

    fn intelligence(&self) -> u8 {
        let sum = self.equipment.weapon_a.mod_attr.intelligence
            .saturating_add(self.equipment.weapon_b.mod_attr.intelligence)
            .saturating_add(self.equipment.armor.mod_attr.intelligence)
            .saturating_add(self.equipment.accessory_a.mod_attr.intelligence)
            .saturating_add(self.equipment.accessory_b.mod_attr.intelligence);
        let attr = self.attributes.base_intelligence;
        attr.attr_mod(sum)
    }

    fn luck(&self) -> u8 {
        let sum = self.equipment.weapon_a.mod_attr.luck
            .saturating_add(self.equipment.weapon_b.mod_attr.luck)
            .saturating_add(self.equipment.armor.mod_attr.luck)
            .saturating_add(self.equipment.accessory_a.mod_attr.luck)
            .saturating_add(self.equipment.accessory_b.mod_attr.luck);
        let attr = self.attributes.base_luck;
        attr.attr_mod(sum)
    }

    fn accuracy(&self) -> u8 {
        let sum = self.equipment.weapon_a.mod_attr.accuracy
            .saturating_add(self.equipment.weapon_b.mod_attr.accuracy)
            .saturating_add(self.equipment.armor.mod_attr.accuracy)
            .saturating_add(self.equipment.accessory_a.mod_attr.accuracy)
            .saturating_add(self.equipment.accessory_b.mod_attr.accuracy);
        let attr = self.attributes.base_accuracy;
        attr.attr_mod(sum)
    }

    fn speed(&self) -> u8 {
        let sum = self.equipment.weapon_a.mod_attr.speed
            .saturating_add(self.equipment.weapon_b.mod_attr.speed)
            .saturating_add(self.equipment.armor.mod_attr.speed)
            .saturating_add(self.equipment.accessory_a.mod_attr.speed)
            .saturating_add(self.equipment.accessory_b.mod_attr.speed);
        let attr = self.attributes.base_speed;
        attr.attr_mod(sum)
    }
}

#[derive(Copy, Clone)]
struct Attr<T> {
    value: T,
}
impl<T> Attr<T> {
    fn new(value: T) -> Self {
        Self {value}
    }
}
impl Attr<u8> {
    fn increment(&mut self) {
        let mut rng = thread_rng();
        let incr = 1 + ((self.value/100)*rng.gen_range(2..=5));
        self.value = self.value.saturating_add(incr);
    }
    fn attr_mod(&self, modify: i8) -> u8 {
        match modify < 0 {
            false => self.value.saturating_add(modify as u8),
            true => self.value.saturating_sub(-modify as u8),
        }
    }
}
impl Attr<u16> {
    fn increment(&mut self) {
        let mut rng = thread_rng();
        let incr = 1 + ((self.value/100)*rng.gen_range(2..=5));
        self.value = self.value.saturating_add(incr);
    }
    fn attr_mod(&self, modify: i16) -> u16 {
        match modify < 0 {
            false => self.value.saturating_add(modify as u16),
            true => self.value.saturating_sub(-modify as u16),
        }
    }
}

struct Attributes {
    base_health: Attr<u16>,
    current_health: u16,
    base_skill: Attr<u16>,
    current_skill: u16,
    base_strength: Attr<u8>,
    base_constitution: Attr<u8>,
    base_intelligence: Attr<u8>,
    base_luck: Attr<u8>,
    base_accuracy: Attr<u8>,
    base_speed: Attr<u8>,
}
impl Attributes {
    fn new() -> Self {
        Self {
            base_health: Attr::new(80_u16),
            current_health: 80,
            base_skill: Attr::new(14_u16),
            current_skill: 14,
            base_strength: Attr::new(20_u8),
            base_constitution: Attr::new(15_u8),
            base_intelligence: Attr::new(15_u8),
            base_luck: Attr::new(15_u8),
            base_accuracy: Attr::new(20_u8),
            base_speed: Attr::new(18_u8),
        }
    }
    fn base_health(&self) -> u16 {self.base_health.value}
    fn current_health(&self) -> u16 {self.current_health}
    fn base_skill(&self) -> u16 {self.base_skill.value}
    fn current_skill(&self) -> u16 {self.current_skill}
    fn base_strength(&self) -> u8 {self.base_strength.value}
    fn base_constitution(&self) -> u8 {self.base_accuracy.value}
    fn base_intelligence(&self) -> u8 {self.base_intelligence.value}
    fn base_luck(&self) -> u8 {self.base_luck.value}
    fn base_accuracy(&self) -> u8 {self.base_accuracy.value}
    fn base_speed(&self) -> u8 {self.base_speed.value}
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

#[derive(Default)]
struct Proficiencies {
    sword: u8,
    axe: u8,
    lance: u8,
    staff: u8,
    knife: u8,
    hammer: u8,
    shield: u8,
}
impl Proficiencies {
    fn new() -> Self {
        Default::default()
    }
}
