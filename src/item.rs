use piston_window::*;
use std::collections::HashMap;
use constants::*;
/**
    Items have three different types - interactable, food, or resource.
    Resource items are used in crafting. Interactable items are used by the
    creature to fight, gather, etc. Food is to be eaten. Yum.
*/

#[derive(Clone)]
pub enum ItemType {
    Interactable(InteractableType), // Can be thrown or interacted with (intent: used for fighting / interacting with world)
    Food(FoodType),                 // Can be thrown or consumed (intent: used for healing / buffs)
    Resource(ResourceType),         // Can be thrown (intent: used for crafting)
                                    // Other,                           // Can be thrown but provideds passive effect (intent: used for passive effect)
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

/**
    Implementation of the Item object.

    @field x Items's x position on map.
    @field y Item's y position on map.
    @field item_type The item type.
    @field damage Damage done when item is thrown.
    @field pickupable Whether or not the item can be picked up.
    @field x_vel Velocity of item when being thrown.
    @field y_vel Velocity of item when being thrown.
    @field weight The item's weight.
*/
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
    // Item constructor.
    pub fn new(item_type: ItemType, damage: i32, pickupable: bool, w: f64) -> Self {
        Item {
            x: 0.0,
            y: 0.0,
            item_type: item_type,
            damage: damage,
            pickupable: pickupable,
            x_vel: 0.0,
            y_vel: 0.0,
            weight: w,
        }
    }
    pub fn draw(
        &mut self,
        textures: &HashMap<String, G2dTexture>,
        context: &Context,
        graphics: &mut G2d,
        trans_x: f64,
        trans_y: f64,
    ) {
        match self.item_type {
            ItemType::Food(FoodType::Bisket) => {
                let img = IMG_ITEM_BISKET;
                image(
                    textures.get(img).expect(&format!("Not found: {:?}", img)),
                    context
                        .transform
                        .trans(self.x, self.y)
                        .trans(trans_x, trans_y).scale(IMAGE_SCALE, IMAGE_SCALE),
                    graphics,
                );
            }
            _ => {}
        }
    }

    // fn interact(&mut self, creature: Creature) {
    // }
    // fn pick_up() {
    // }
    // fn throw() {
    // }

    // Generates a clone of the item using prototype.
    // @param x The x value where the clone is placed.
    // @param y The y value where the clone is placed.
    // @return Item
    pub fn generate_clone(&self, x: f64, y: f64) -> Self {
        Item {
            x: x,
            y: y,
            ..self.clone()
        }
    }
}

// Items to add & crafting
// Wood -> Sword
// Plant -> Food
