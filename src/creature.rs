//! The Creature object is the template for any NPC in AOE. Currently this is used for
//! the Player, but default functionality will eventually apply for Monsters and Crew.
//!
//! The object handles Creature generation, position and movement, item interaction,
//! and graphics rendering.

use constants::*;
use item::*;
use misc::*;
use piston_window::*;
use std::collections::HashMap;
use std::mem::swap;

#[derive(Debug, PartialEq)]
pub enum CreatureState {
    Normal,
    ControllingShip,
}

/**
	Implementation of the Creature object.

    @field creature_state Creature's control state - only applicable for Player.
	@field x Creature's horizontal position on screen.
	@field y Creature's vertical position on screen.
    @field self_vel_x Creature's horizontal velocity.
    @field self_vel_y Creature's vertical velocity.
    @field directions Vector recording directions Creature is moving.
    @field other_vel_x Horizontal velocity of other object(s) affecting Creature.
    @field other_vel_y Horizontal velocity of other object(s) affecting Creature.
    @field speed Creature's maximum speed when moving.
    @field health Creature's health.
    @field inventory Creature's item inventory.
    @field dir Creature's direction for drawing grapics.
    @field sprite_index Used for animating the Creature's sprite.
    @field frames_since_last_draw Used for updating Creature animation.
    @field animation_rate Throttles how often the sprites update.
*/
pub struct Creature {
    pub creature_state: CreatureState,
    pub x: f64,
    pub y: f64,
    pub self_vel_x: f64,
    pub self_vel_y: f64,
    pub directions: Vec<Direction>,
    pub other_vel_x: f64,
    pub other_vel_y: f64,
    pub speed: f64,
    pub health: i32,
    pub inventory: Option<Item>,
    dir: Direction,
    sprite_index: i32,
    frames_since_last_draw: i32,
    animation_rate: i32,
}

impl Creature {
    /* 
        Constructor for default Creature 
    */
    pub fn new() -> Creature {
        Creature {
            creature_state: CreatureState::Normal,
            x: 0.0,
            y: 0.0,
            self_vel_x: 0.0,
            self_vel_y: 0.0,
            directions: vec![],
            other_vel_x: 0.0,
            other_vel_y: 0.0,
            speed: 2.0,
            health: 3,
            inventory: None,
            dir: Direction::S,
            sprite_index: 0,
            frames_since_last_draw: 0,
            animation_rate: 5,
        }
    }

    /* 
        Updates the position of the Creature based on other objects acting on it,
        e.g. the airship's speed.
    */
    pub fn update_position_other(&mut self) {
        self.x += self.other_vel_x;
        self.y += self.other_vel_y;
    }

    /* 
        Updates the position of the Creature based on its own velocity,
        e.g. the player's controls.
    */
    pub fn update_position_self(&mut self) {
        self.x += self.self_vel_x;
        self.y += self.self_vel_y;
        if let Some(ref mut item) = self.inventory {
            // Update inventory position as well.
            item.x = self.x;
            item.y = self.y;
        }
    }

    /*
        Updates the direction that the creature is facing for graphics purposes.
    */
    pub fn update_direction(&mut self) {
        if !(self.self_vel_y == 0.0 && self.self_vel_x == 0.0) {
            if self.self_vel_x > 0.0 {
                self.dir = Direction::E;
            } else if self.self_vel_x < 0.0 {
                self.dir = Direction::W;
            }

            // Vertical direction overrides horizontal direction.
            if self.self_vel_y > 0.0 {
                self.dir = Direction::S;
            } else if self.self_vel_y < 0.0 {
                self.dir = Direction::N;
            }
        }
    }

    /*
        Calculates the location where the Creature is approaching -
        used for collision detection.

        @return f64 The future x and y positions of the Creature.
    */
    pub fn x_to_be_location(&self) -> f64 {
        self.x + self.self_vel_x
    }
    pub fn y_to_be_location(&self) -> f64 {
        self.y + self.self_vel_y
    }

    /*
        Draws the Creature-related graphics.

        @param textures The map of sprite / tile textures.
        @param context The context used for drawing, affected by scaling and translation.
        @param graphics Graphics engine.
        @param w_width Width of the window.
        @param w_height Height of the window.
    */
    pub fn draw(
        &mut self,
        textures: &HashMap<String, G2dTexture>,
        context: &Context,
        graphics: &mut G2d,
        w_width: f64,
        w_height: f64,
    ) {
        // Determine the proper file extension based on Creature moving.
        let pic_index = self.sprite_index + 1;
        let extension;
        if self.self_vel_y != 0.0 || self.self_vel_x != 0.0 {
            extension = "player_moving_";
        } else {
            extension = "player_idle_";
        }

        let img = &format!(
            "{}{}{}{}",
            extension,
            self.dir.direction_to_string(),
            "_",
            pic_index.to_string()
        );

        image(
            textures.get(img).expect(&format!("Not found: {:?}", img)),
            context
                .transform
                .trans(w_width / 2.0, w_height / 2.0) // Draw Player at center of screen.
                .scale(IMAGE_SCALE, IMAGE_SCALE),
            graphics,
        );

        // Handle "frame rate" for animation.
        if self.frames_since_last_draw > self.animation_rate {
            self.frames_since_last_draw = 0;
            self.sprite_index = (self.sprite_index + 1) % 3;
        }
        self.frames_since_last_draw += 1;

        // Display Creature's item(s).
        if let Some(_) = self.inventory {
            let item = self.inventory.clone().unwrap();
            match item.item_type {
                ItemType::Food(FoodType::Bisket) => {
                    let img = IMG_ITEM_BISKET;
                    image(
                        textures.get(img).expect(&format!("Not found: {:?}", img)),
                        context
                            .transform
                            .trans(w_width / 2.0, w_height / 2.0 - IMAGE_SIZE_SCALED * 0.7) // Draw above Creature.
                            .scale(IMAGE_SCALE, IMAGE_SCALE),
                        graphics,
                    );
                }
                _ => {}
            }
        }
    }

    /*
        Determines if the Creature is dead.

        @return bool Whether health is 0.
    */
    pub fn is_dead(&self) -> bool {
        return self.health <= 0;
    }

    /*
        Deducts from the Creature's health.

        @param damage The damage to be deducted.
    */
    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }

    /*
        Removes item from Creature's inventory.

        @return Option<Item> The item that is dropped.
    */
    pub fn drop_item(&mut self) -> Option<Item> {
        let mut dropped_item = None;
        swap(&mut self.inventory, &mut dropped_item);
        dropped_item
    }

    /*
        Adds item to Creature's inventory, if empty.

        @param item The item being picked up.
        @return bool Whether pickup was successful. 
    */
    pub fn pickup_item(&mut self, item: Item) -> bool {
        if let None = self.inventory {
            self.inventory = Some(item);
            return true;
        }
        false
    }

    /*
        Handles results of Creature using an item.
    */
    pub fn use_item(&mut self) {
        let mut item_used = false;
        if let Some(ref item) = self.inventory {
            match item.item_type {
                ItemType::Food(_) => {
                    // Heal Creature.
                    self.health += 1;
                    item_used = true;
                }
                _ => {}
            }
        }
        if item_used {
            self.inventory = None; // Empty inventory.
        }
    }

    /*
        Executed for Player when changing between control of ship.
    */
    pub fn change_control_state(&mut self) {
        match self.creature_state {
            CreatureState::Normal => self.state_normal(),
            CreatureState::ControllingShip => self.state_controlling_ship(),
        }
    }

    /*
        Toggles out of ship-control.
    */
    fn state_controlling_ship(&mut self) {
        self.creature_state = CreatureState::Normal;
    }

    /*
        Toggles into ship-control.
    */
    fn state_normal(&mut self) {
        self.directions = vec![];
        self.self_vel_x = 0.0;
        self.self_vel_y = 0.0;
        self.creature_state = CreatureState::ControllingShip;
    }
}

// Moving of creature.
impl Moveable for Creature {
    /*
        Handles input from user for moving the Player.

        @param state The Button State of some button (pressed or released).
        @param key The Key pressed by the user (e.g. W, A, S, or D).
    */
    fn handle_input(&mut self, state: &ButtonState, key: &Option<Key>) {
        match *key {
            // Add directions for Player.
            Some(Key::W) => {
                let dir = Direction::N;
                if let Some(index) = self.directions.iter().position(|&x| x == dir) {
                    if *state == ButtonState::Release {
                        self.directions.remove(index);
                    }
                } else {
                    if *state == ButtonState::Press {
                        self.directions.push(dir);
                    }
                }
            }
            Some(Key::A) => {
                let dir = Direction::W;
                if let Some(index) = self.directions.iter().position(|&x| x == dir) {
                    if *state == ButtonState::Release {
                        self.directions.remove(index);
                    }
                } else {
                    if *state == ButtonState::Press {
                        self.directions.push(dir);
                    }
                }
            }
            Some(Key::S) => {
                let dir = Direction::S;
                if let Some(index) = self.directions.iter().position(|&x| x == dir) {
                    if *state == ButtonState::Release {
                        self.directions.remove(index);
                    }
                } else {
                    if *state == ButtonState::Press {
                        self.directions.push(dir);
                    }
                }
            }
            Some(Key::D) => {
                let dir = Direction::E;
                if let Some(index) = self.directions.iter().position(|&x| x == dir) {
                    if *state == ButtonState::Release {
                        self.directions.remove(index);
                    }
                } else {
                    if *state == ButtonState::Press {
                        self.directions.push(dir);
                    }
                }
            }
            _ => {}
        }
    }

    /*
        Updates Creature position based on velocities.
        Override for Creature.
    */
    fn update_position(&mut self) {
        self.x += self.other_vel_x;
        self.y += self.other_vel_y;
        self.x += self.self_vel_x;
        self.y += self.self_vel_y;
    }

    /*
        Changes the Creature's personal velocity (unrelated to other velocities acting on
        the creature).
        
        @param dx The difference in x velocity.
        @param dy The difference in y velocity.
    */
    fn update_self_velocity(&mut self) {
        let mut dx = 0.0;
        let mut dy = 0.0;

        for dir in &self.directions {
            match *dir {
                Direction::N => dy -= self.speed,
                Direction::S => dy += self.speed,
                Direction::W => dx -= self.speed,
                Direction::E => dx += self.speed,
            }
        }
        self.self_vel_x = dx;
        self.self_vel_y = dy;
    }
}
