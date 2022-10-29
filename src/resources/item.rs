#[derive(Copy, Clone)]
pub struct AttrsModifiers {
    pub hp: i16,
    pub strength: i8,
    pub constitution: i8,
    pub intelligence: i8,
    pub speed: i8,
    pub luck: i8,
}
impl AttrsModifiers {
    fn new() -> AttrsModifiers {
        AttrsModifiers {
            hp: 0,
            strength: 0,
            constitution: 0,
            intelligence: 0,
            speed: 0,
            luck: 0,
        }
    }
}

#[derive(Clone)]
pub enum Item {
    Equippable(Equippable),
    Letter(Letter),
    Consumable(Consumable),
}

#[derive(Copy, Clone)]
pub enum EqpbleType {
    Weapon,
    Helmet,
    Armor,
    Accessory,
    None,
}

#[derive(Clone)]
pub struct ItemInfo {
    pub name: String,
    pub description: String,
}

#[derive(Clone)]
pub struct Equippable {
    pub equip_type: EqpbleType,
    pub info: ItemInfo,
    pub effects: AttrsModifiers,
    pub weariness: Option<u16>,
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
    effects: AttrsModifiers,
}

pub fn new_equippable(id: String)-> Equippable {
    match id.as_str() {
        _ => Equippable {
            equip_type: EqpbleType::None,
            info: ItemInfo {
                name: "none".into(),
                description: "...".into(),
            },
            effects: AttrsModifiers::new(),
            weariness: None,
        }
    }
}
