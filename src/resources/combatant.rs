const EXP_MAX: u32 = 1000000;

pub struct Combatant {
    name: String,
    level: u8,
    exp: u32,
    current_hp: u16,
    hp: u16,
    attributes: Attributes,
    equipment: Equipments,
}
impl Combatant {
    pub fn new(name: &str) -> Combatant {
        Combatant {
            name: name.to_string(),
            level: 1,
            exp: 0,
            current_hp: 80,
            hp: 80,
            attributes: Attributes::new(),
            equipment: Equipments::new(),
        }
    }
    pub fn details(&self) {
        println!("Details:\n  name: {}\tlevel: {}\n  exp: {}\tnext: {}\n  hp: {}/{}",
            self.name, self.level, self.exp, self.lv_up_threshold()-self.exp, self.current_hp, self.hp);
    }
    fn lv_up_threshold(&self) -> u32 {
        (self.level as u32 + 1).pow(3)
    }
    fn equip(&mut self, index: usize, items: &mut Vec<Item>) {
        match items[index].class {
            ItemClass::Weapon => self.equipment.weapon = items.remove(index),
            ItemClass::Head_armor => self.equipment.head = items.remove(index),
            ItemClass::Body_armor => self.equipment.body = items.remove(index),
            ItemClass::Accessory => self.equipment.weapon = items.remove(index),
            _ => println!("this item cannot be equipped."),
        };
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
    strength: u8,
    constitution: u8,
    intelligence: u8,
    speed: u8,
    luck: u8,
}
impl Attributes {
    fn new() -> Attributes {
        Attributes {
            strength: 20,
            constitution: 15,
            intelligence: 15,
            speed: 18,
            luck: 15,
        }
    }
    fn show(&self) {
        println!(
        "attributes:\n  strength: {}\n  constitution: {}\n  intelligence: {}\n  speed: {}\n  luck: {}",
        self.strength, self.constitution, self.intelligence, self.speed, self.luck);
    }
}

struct Equipments {
    weapon: Item,
    head: Item,
    body: Item,
    accessory: [Item; 4],
}
impl Equipments {
    fn new() -> Equipments {
        Equipments {
            weapon: Item::none(),
            head: Item::none(),
            body: Item::none(),
            accessory: [Item::none(), Item::none(), Item::none(), Item::none()]
        }
    }
}

