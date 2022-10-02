
#[derive (Copy, Clone)]
enum Effect {

}

#[derive (Clone)]
pub enum Item {
    Equippable(Equippable),
    Letter(Letter),
    Consumable(Consumable),
}

#[derive (Copy, Clone)]
pub enum EqpbleType {
    Weapon,
    Helmet,
    Armor,
    Accessory,
}

#[derive (Clone)]
pub struct Equippable {
    equip_type:  EqpbleType,
    name:        String,
    description: String,
    effects:     Effect,
}

#[derive (Clone)]
pub struct Letter {
    name:    String,
    message: String,
}

#[derive (Clone)]
pub struct Consumable {
    name:           String,
    description:    String,
    consume_times:  u8,
    effect:         Effect
}

