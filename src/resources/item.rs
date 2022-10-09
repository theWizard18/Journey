#[derive(Copy, Clone)]
struct AttrsModifiers {
    hp: i16,
    strength: i8,
    constitution: i8,
    intelligence: i8,
    speed: i8,
    luck: i8,
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
}

#[derive(Clone)]
struct ItemInfo {
    name: String,
    description: String,
}

#[derive(Clone)]
pub struct Equippable {
    equip_type: EqpbleType,
    info: ItemInfo,
    effects: AttrsModifiers,
    weariness: Option<u16>,
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
