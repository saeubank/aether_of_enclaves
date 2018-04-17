#[derive(Clone)]

pub enum ItemType {
    Interactable(InteractableType), // Can be thrown or interacted with (intent: used for fighting / interacting with world)
    Food(FoodType),         // Can be thrown or consumed (intent: used for healing / buffs)
    Resource(ResourceType),     // Can be thrown (intent: used for crafting)
    Other,        // Can be thrown but provideds passive effect (intent: used for passive effect)
}

#[derive(Clone)]

pub enum FoodType {
    Bisket,
}

#[derive(Clone)]

pub enum InteractableType {
    Sword,
}

#[derive(Clone)]

pub enum ResourceType {
    Logs,
    Grune,
}

#[derive(Clone)]

pub struct Item {
    pub x: f64,
    pub y: f64,
    pub item_type: ItemType,
    pub damage: i32, // Damage done when thrown
    pub pickupable: bool,
    pub x_vel: f64,
    pub y_vel: f64,
    pub weight: f64,
}


impl Item {
    pub fn new(item_t: ItemType, dam: i32, pickup: bool, w: f64) -> Self {
        Item {
            x: 0.0,
            y: 0.0,
            item_type: item_t,
            damage: dam,
            pickupable: pickup,
            x_vel: 0.0,
            y_vel: 0.0,
            weight: w,
        }
    }

    // fn interact(&mut self, creature: Creature) {

    // }

    // fn pick_up() {
    // }
    // fn throw() {
    // }

    pub fn generate_clone(&self, x: f64, y: f64) -> Self {
        Item {
            x: x,
            y: y,
            ..self.clone()
            // item_type: self.item_type,
            // damage: self.damage,
            // pickupable: self.pickupable,
            // x_vel: self.x_vel,
            // y_vel: self.y_vel,
            // weight: self.weight,
        }
    }
}

// Items to add & crafting
// Wood -> Sword
// Plant -> Food
