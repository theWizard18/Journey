#[derive(Clone)]
pub enum Item {
    Weapon(Weapon),
    Armor(Armor),
    Accessory(Accessory),
    Letter(Letter),
    Consumable(Consumable),
}

#[derive(Clone)]
pub enum WpnType {
    Sword, Axe, Lance, Staff, Knife, Hammer, Shield, None,
}

#[derive(Clone)]
pub struct ItemInfo {
    pub name: String,
    pub description: String,
}

#[derive(Clone, Default)]
pub struct ItemAttr {
    pub health: i16,
    pub skill: i16,
    pub strength: i8,
    pub constitution: i8,
    pub intelligence: i8,
    pub luck: i8,
    pub accuracy: i8,
    pub speed: i8,
}

#[derive(Clone)]
pub struct Weapon {
    pub info: ItemInfo,
    pub wpn_type: WpnType,
    pub mod_attr: ItemAttr,
    pub durability: Option<u16>,
}

#[derive(Clone)]
pub struct Armor {
    pub info: ItemInfo,
    pub mod_attr: ItemAttr,
    pub durability: Option<u16>,
}

#[derive(Clone)]
pub struct Accessory {
    pub info: ItemInfo,
    pub mod_attr: ItemAttr,
}

#[derive(Clone)]
pub struct Letter {
    info: ItemInfo,
    message: String,
}

#[derive(Clone)]
pub struct Consumable {
    info: ItemInfo,
    consume_times: u8,
}

pub fn new_weapon(id: &str) -> Weapon {
    match id {
        _ => Weapon {
            info: ItemInfo {
                name: "none".into(),
                description: "...".into(),
            },
            wpn_type: WpnType::None,
            mod_attr: Default::default(),
            durability: None,
        }
    }
}

pub fn new_armor(id: &str) -> Armor {
    match id {
        _ => Armor {
            info: ItemInfo {
                name: "none".into(),
                description: "...".into(),
            },
            mod_attr: Default::default(),
            durability: None,
        }
    }
}

pub fn new_accessory(id: &str) -> Accessory {
    match id {
        _ => Accessory {
            info: ItemInfo {
                name: "none".into(),
                description: "...".into(),
            },
            mod_attr: Default::default(),
        }
    }
}
