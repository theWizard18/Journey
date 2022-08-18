use std::io::{self, Write};

const EXP_MAX: u32 = 1000000;
const LV_MAX: u8 = 100;

#[derive (Copy, Clone)]
enum ItemClass {
    None,
    Weapon,
    Head_armor,
    Body_armor,
    Accessory,
    Letter,
    Throwable,
    Consumable,
    Key,
}

struct Item {
    name: String,
    description: String,
    class: ItemClass,
}
impl Item {
    fn none() -> Item {
        Item {
            name: String::from("None"),
            description: String::from("[...]"),
            class: ItemClass::None,
        }
    }
}
impl Item {
    fn name(&self) -> String { self.name.clone() }
    fn description(&self) -> String { self.description.clone() }
    fn class(&self) -> ItemClass { self.class }
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

pub struct Combatant {
    name: String,
    level: u8,
    lv_up_threshold: u32,
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
            lv_up_threshold: 2u32.pow(3),
            exp: 0,
            current_hp: 80,
            hp: 80,
            attributes: Attributes::new(),
            equipment: Equipments::new(),
        }
    }
    pub fn details(&self) {
        println!("Details:\n  name: {}\tlevel: {}\n  exp: {}\tnext: {}\n  hp: {}/{}",
            self.name, self.level, self.exp, self.lv_up_threshold-self.exp, self.current_hp, self.hp);
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
        while self.exp > self.lv_up_threshold {
            self.level_up();
        }
    }
    fn level_up(&mut self) {
        self.level += 1;
        self.lv_up_threshold = (self.level as u32+1).pow(3);
    }
}

struct Party {
    members: Vec<Combatant>,
    items: Vec<Item>,
}
impl Party {
    fn new(members: Vec<Combatant>, items: Vec<Item>) -> Party {
        Party {
            members,
            items,
        }
    }
    fn recruit(&mut self, new_guy: Combatant) {
        self.members.push(new_guy);
    }
    fn store(&mut self, new_item: Item) {
        self.items.push(new_item);
    }
}

fn input(prompt: String) {
    print!("{prompt}");
    io::stdout().flush().expect("failed to flush the prompt");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed to get input");
}

