/*
    Not used Currently
*/


enum ItemType {
    Interactable, // Can be thrown or interacted with (intent: used for fighting / interacting with world)
    Food, // Can be thrown or consumed (intent: used for healing / buffs)
    Resource, // Can be thrown (intent: used for crafting)
    Other // Can be thrown but provideds passive effect (intent: used for passive effect)
}


pub struct Item {
    item_type: ItemType,
    pub damage: i32, // Damage done when thrown
    pub pickupable: bool
}

impl Item {
    fn new(item_t: ItemType) -> Self {
        Item {
            item_type: item_t,
            damage: 0,
            pickupable: true,
        }
    }

    // fn interact(&mut self, creature: Creature) {

    // }

    // fn pick_up() {
    // }
    // fn throw() {
    // }
}

// Items to add & crafting
// Wood -> Sword
// Plant -> Food
