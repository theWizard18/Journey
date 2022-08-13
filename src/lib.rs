use std::fmt;

trait Info {
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;
    fn output_data(&self) {
        println!("{}:\n{}", self.get_name(), self.get_description());
    }
}

struct Item {
    name: String,
    description: String,
}
impl Item {
    fn none() -> Item {
        Item {
            name: String::from("None"),
            description: String::from("[...]"),
        }
    }
}
impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
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
            intelligence: 16,
            speed: 18,
            luck: 15,
        }
    }
}
impl fmt::Display for Attributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n  strength: {}\n  constitution: {}\n  intelligence: {}\n  speed: {}\n  luck: {}",
        self.strength, self.constitution, self.intelligence, self.speed, self.luck)
    }
}

struct Equipments {
    weapon: Item,
    head: Item,
    body: Item,
    accessory_1: Item,
    accessory_2: Item,
    accessory_3: Item,
    accessory_4: Item,
}
impl Equipments {
    fn new() -> Equipments {
        Equipments {
            weapon: Item::none(),
            head: Item::none(),
            body: Item::none(),
            accessory_1: Item::none(),
            accessory_2: Item::none(),
            accessory_3: Item::none(),
            accessory_4: Item::none(),
        }
    }
}
impl fmt::Display for Equipments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n  weapon: {}\n  head: {}\n  body: {}\n  accessories: {}\n  {}\n  {}\n  {}",
        self.weapon, self.head, self.body, self.accessory_1, self.accessory_2, self.accessory_3, self.accessory_4)
    }
}

pub struct Player {
    name: String,
    level: u8,
    exp: u32,
    hp: u16,
    attributes: Attributes,
    equipment: Equipments,
}
impl Player {
    pub fn new(name: &str) -> Player {
        Player {
            name: name.to_string(),
            level: 1,
            exp: 0,
            hp: 80,
            attributes: Attributes::new(),
            equipment: Equipments::new(),
        }
    }
}
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name: {}\nlevel: {}\nexp: {}\nhp: {}\nattributes:\n{}\nequipment:\n{}",
        self.name, self.level, self.exp, self.hp, self.attributes, self.equipment)
    }
}

