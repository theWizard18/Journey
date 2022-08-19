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

