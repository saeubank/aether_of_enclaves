/**
	Input Handler uses *inheritance to handle the sets of
	commands which might be inputted from the Game and user.

	Modeled with the Command programming design pattern.

	*Rust doesn't truly support inheritance & classes, so this
	has been manually recreated to the best of our ability.
*/

use piston_window::{Button, Key};
use game::GameState;
use player::Player;

#[derive(Debug)]

// Used for Move sub-struct.
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/**
	Ancestor object for different types of input Commands.
*/
trait Command {
    // Initializer for Command.
    fn new() -> Self;

    // Execute actions based on type of Command.
    // @param Option<Key> And optional key value.
    fn execute(&mut self, Option<Key>, player: &mut Player, game_state: &mut GameState);
}

/**
	Implementation of the OpenMenu Command.
*/
struct OpenMenu {}

impl OpenMenu {
    pub fn new() -> Self {
        OpenMenu {}
    }
}

impl Command for OpenMenu {
    fn new() -> Self {
        OpenMenu {}
    }

    // Unused param _key.
    fn execute(&mut self, _key: Option<Key>, _player: &mut Player, game_state: &mut GameState) {
        match *game_state {
            GameState::Main => {
                println!("Menu opened.");
                *game_state = GameState::InMenu;
            },
            GameState::InMenu => {
                println!("Menu closed.");
                *game_state = GameState::Main;
            },
            _ => {}
        }
    }
}

/**
	Implementation of the Action Command.
	Used when the player presses the action button (Return).
*/
struct Action {}

impl Action {
    pub fn new() -> Self {
        Action {}
    }
}

impl Command for Action {
    fn new() -> Self {
        Action {}
    }

    // Unused param _key.
    fn execute(&mut self, _key: Option<Key>, _player: &mut Player, game_state: &mut GameState) {
        match *game_state {
            GameState::Title => {
                println!("Changing state to Main.");
                *game_state = GameState::Main;
            },
            _ => {}
        }
    }
}

/**
	Implementation of the Move Command.
	This will either move the player or move selections
	in a menu.

	@field dir The direction of the Command.
*/
struct Move {}

impl Move {
    pub fn new() -> Self {
        Move {}
    }
}

impl Command for Move {
    fn new() -> Self {
        Move {}
    }

    // @param key The input key.
    fn execute(&mut self, key: Option<Key>, player: &mut Player, _game_state: &mut GameState) {
        use self::Direction::*;
        let mut dir = None;
        match key {
            Some(Key::W) => dir = Some(Up),
            Some(Key::A) => dir = Some(Left),
            Some(Key::S) => dir = Some(Down),
            Some(Key::D) => dir = Some(Right),
            _ => {}
        }
        println!("Moving {:?}.", dir);
        player.update_position(dir, 15.0);
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

    // @param button The input button arguments.
    pub fn handle_input(
        &mut self,
        button: Button,
        player: &mut Player,
        game_state: &mut GameState,
    ) {
        use self::Key::*;
        match button {
            Button::Keyboard(key) => match key {
                Return => self.action.execute(None, player, game_state),
                Tab => self.open_menu.execute(None, player, game_state),
                W | A | S | D => self.move_dir.execute(Some(key), player, game_state),
                _ => {}
            },
            _ => {}
        }
    }
}
