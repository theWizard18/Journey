use crate::resources::party::Party;

struct Battle {
    party_a: Party,
    party_b: Party,
}
impl Battle {
    fn cycle(&self) {
        while self.party_a.can_fight() && self.party_b.can_fight() {
            todo!()
        }
        self.end();
    }
    fn end(&self) {
        todo!();
    }
}
