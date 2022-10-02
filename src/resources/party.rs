use crate::resources::{combatant::Combatant, item::Item};

pub struct Party {
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
    fn recruit(&mut self, new_member: Combatant) {
        self.members.push(new_member);
    }
    fn store(&mut self, new_item: Item) {
        self.items.push(new_item);
    }
}

