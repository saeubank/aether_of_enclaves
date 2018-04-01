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

/**
	Ancestor object for different types of input Commands.
*/
trait Command {
    // Constructor for Command.
    fn new() -> Self;

    // Execute actions based on type of Command.
    // @param ButtonState Either "pressed" or "released".
    // @param Option<Key> And optional key value.
    // @param &mut Creature The Player.
    // @param GameState The current Game State.
    fn execute(&mut self, ButtonState, Option<Key>, &mut Creature, &mut GameState);
}

/**
	Implementation of the OpenMenu Command.
*/
struct OpenMenu {}

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
        _player: &mut Creature,
        game_state: &mut GameState,
    ) {
        if state == ButtonState::Press {
            match *game_state {
                GameState::InGame => {
                    println!("Menu opened.");
                    *game_state = GameState::InMenu;
                }
                GameState::InMenu => {
                    println!("Menu closed.");
                    *game_state = GameState::InGame;
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
struct Action {}

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
        _player: &mut Creature,
        game_state: &mut GameState,
    ) {
        if state == ButtonState::Press {
            match *game_state {
                GameState::Title => {
                    println!("Changing state to InGame.");
                    *game_state = GameState::InGame;
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
struct Move {}

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
        player: &mut Creature,
        _game_state: &mut GameState,
    ) {
        player.handle_input(state, key);
        player.update_self_velocity();
    }
}

/**
	Implementation of the Input Handler.
	Executes respective Commands given Button input.

	@field move_dir The Move Command handler.
	@field action The Action Command handler.
	@field open_menu The OpenMenu Command handler.
*/
pub struct InputHandler {
    move_dir: Move,
    action: Action,
    open_menu: OpenMenu,
}

impl InputHandler {
    // Constructor.
    pub fn new() -> Self {
        InputHandler {
            move_dir: Move::new(),
            action: Action::new(),
            open_menu: OpenMenu::new(),
        }
    }

    // @param state The ButtonState.
    // @param button The input button arguments.
    // @param player The player.
    // @param game_state The current Game State.
    pub fn handle_input(
        &mut self,
        state: ButtonState,
        button: Button,
        player: &mut Creature,
        game_state: &mut GameState,
    ) {
        use self::Key::*;
        match button {
            Button::Keyboard(key) => match key {
                // Action button.
                Return => self.action.execute(state, None, player, game_state),
                // Menu toggle.
                Tab => self.open_menu.execute(state, None, player, game_state),
                // Move.
                W | A | S | D => self.move_dir.execute(state, Some(key), player, game_state),
                _ => {}
            },
            _ => {}
        }
    }
}

/*
fn execute<T: Moveable>(thing: T) {
    println!("yeet");
}
*/
