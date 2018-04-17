/*
    Not used Currently
*/

enum ItemType {
    Interactable(InteractableType), // Can be thrown or interacted with (intent: used for fighting / interacting with world)
    Food(FoodType),         // Can be thrown or consumed (intent: used for healing / buffs)
    Resource(ResourceType),     // Can be thrown (intent: used for crafting)
    Other,        // Can be thrown but provideds passive effect (intent: used for passive effect)
}

enum FoodType {
    Bisket,
}

enum InteractableType {
    Sword,
}

enum ResourceType {
    Wood,
    Grune,
}

pub struct Item {
    item_type: ItemType,
    pub damage: i32, // Damage done when thrown
    pub pickupable: bool,
    pub x: f64,
    pub y: f64,
    pub x_vel: f64,
    pub y_vel: f64,
}

impl Item {
    fn new(item_t: ItemType) -> Self {
        Item {
            item_type: item_t,
            damage: 0,
            pickupable: true,
            x: 0.0,
            y: 0.0,
            x_vel: 0.0,
            y_vel: 0.0,
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
