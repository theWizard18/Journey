use crate::resources::item::Item;
struct Place {
    north: Box<Something>,
    north_east: Box<Something>,
    east: Box<Something>,
    south_east: Box<Something>,
    south: Box<Something>,
    south_west: Box<Something>,
    west: Box<Something>,
    north_west: Box<Something>,
}

struct Door {
    side_a: Place,
    side_b: Place,
    lock: u8, // if a lock and a key has the same u8 number then the key can open the lock
}
struct Person;
struct Chest {
    content: Item,
    lock: u8, // if a lock and a key has the same u8 number then the key can open the lock
}

enum Something {
    Place(Place),
    Wall,
    Entrance(Door),
    Person(Person),
    Chest(Chest),
    Item(Item)
}
