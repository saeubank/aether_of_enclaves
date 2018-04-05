/**
	Input Handler uses *inheritance to handle the sets of
	commands which might be inputted from the Game and user.

	Modeled with the Command programming design pattern.

	*Rust doesn't truly support inheritance & classes, so this
	has been manually recreated to the best of our ability.
*/

use piston_window::{Button, ButtonState, Key};
use game::GameState;
use creature::Creature;
use misc::*;
use game::Game;

/**
	Ancestor object for different types of input Commands.
*/
pub trait Command {
    // Constructor for Command.
    fn new() -> Self;

    // Execute actions based on type of Command.
    // @param ButtonState Either "pressed" or "released".
    // @param Option<Key> And optional key value.
    // @param &mut Creature The Player.
    // @param GameState The current Game State.
    fn execute(&mut self, ButtonState, Option<Key>, &mut Game);
}

/**
	Implementation of the OpenMenu Command.
*/
pub struct OpenMenu {}

impl Command for OpenMenu {
    fn new() -> Self {
        OpenMenu {}
    }

    // @param state The ButtonState (pressed or released).
    // @param _key Unused.
    // @param _player Unused.
    // @param game_state The current Game State.
    fn execute(
        &mut self,
        state: ButtonState,
        _key: Option<Key>,
        game: &mut Game
    ) {
        if state == ButtonState::Press {
            match game.game_state {
                GameState::InGame => {
                    println!("Menu opened.");
                    game.game_state = GameState::InMenu;
                }
                GameState::InMenu => {
                    println!("Menu closed.");
                    game.game_state = GameState::InGame;
                }
                _ => {}
            }
        }
    }
}

/**
	Implementation of the Action Command.
	Used when the player presses the action button (Enter).
*/
pub struct Action {}

impl Command for Action {
    fn new() -> Self {
        Action {}
    }

    // @param state The ButtonState (pressed or released).
    // @param _key Unused.
    // @param _player Unused.
    // @param game_state The current Game State.
    fn execute(
        &mut self,
        state: ButtonState,
        _key: Option<Key>,
        game: &mut Game
    ) {
        if state == ButtonState::Press {
            match game.game_state {
                GameState::Title => {
                    println!("Changing state to InGame.");
                    game.game_state = GameState::InGame;
                }
                _ => {}
            }
        }
    }
}

/**
	Implementation of the Move Command.
	This will either move the player/ship or move selections
	in a menu.
*/
pub struct Move {}

impl Command for Move {
    fn new() -> Self {
        Move {}
    }

    // @param state The ButtonState (pressed or released).
    // @param key The input key.
    // @param player The player.
    // @param game_state The current Game State.
    fn execute(
        &mut self,
        state: ButtonState,
        key: Option<Key>,
        game: &mut Game
    ) {
        game.player.handle_input(state, key);
        game.player.update_self_velocity();
    }
}

/*
fn execute<T: Moveable>(thing: T) {
    println!("yes");
}
*/
