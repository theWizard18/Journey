use std::fmt;

trait Info {
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;
    fn output_data(&self) {
        println!("{}:\n{}", self.get_name(), self.get_description());
    }
}

struct Attributes {
    strength: u8,
    constitution: u8,
    intelligence: u8,
    speed: u8,
    luck: u8,
} impl Attributes {
    fn new() -> Attributes {
        Attributes {
            strength: 20,
            constitution: 15,
            intelligence: 16,
            speed: 18,
            luck: 15,
        }
    }
    fn show(&self) {
        println!("Attributes: ");
        println!("  strength: {}", self.strength);
        println!("  constitution: {}", self.constitution);
        println!("  intelligence: {}", self.intelligence);
        println!("  speed: {}", self.speed);
        println!("  luck: {}", self.luck);
    }
}

struct Weapon {
    name: String,
    description: String,
} impl fmt::Display for Weapon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name.as_str())
    }
}

struct HeadArmor {
    name: String,
    description: String,
}

struct BodyArmor {
    name: String,
    description: String,
}

struct Accessory {
    name: String,
    description: String,
}

struct Equipments {
    weapon: Option<Weapon>,
    head: Option<HeadArmor>,
    body: Option<BodyArmor>,
    accessories: [Option<Accessory>; 4]
} impl Equipments {
    fn new() -> Equipments {
        Equipments {
            weapon: None,
            head: None,
            body: None,
            accessories: [None, None, None, None]
        }
    }

    fn show(&self) {
        println!("Equipment:");
        println!("weapon: {}", self.weapon);
        println!("head: {}");
        println!("body: {}");
        println!("accessories: {}");
    }
}

pub struct Player {
    name: String,
    level: u8,
    hp: u16,
    attributes: Attributes,
    equipment: Equipments,
} impl Player {
    pub fn show(&self) {
        println!("name: {}", self.name);
        println!("\tlevel: {}", self.level);
        println!("\thp: {}", self.hp);
        self.attributes.show();
        self.equipment.show();
    }

    pub fn new(name: String) -> Player {
        Player {
            name,
            level: 1,
            hp: 80,
            attributes: Attributes::new(),
            equipment: Equipments::new(),
        }
    }
}

