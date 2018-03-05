/*
    Not used Currently
*/


enum ItemType {
    Interactable, // Can be thrown or interacted with (intent: used for fighting / interacting with world)
    Food, // Can be thrown or consumed (intent: used for healing / buffs)
    Resource, // Can be thrown (intent: used for crafting)
    Other // Can be thrown but provideds passive effect (intent: used for passive effect)
}


struct Item {
    item_type: ItemType,
    damage: usize, // Damage done when thrown
    pickupable: bool
}

impl Item {
    fn new(ItemType) -> Self {

    }

    fn interact(&mut self, creature: Creature) {
        
    }

    fn pick_up() {

    }
}

// Items to add & crafting
// Wood -> Sword
// Plant -> Food
