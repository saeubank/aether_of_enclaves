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

/**
	Ancestor object for different types of input Commands.
*/
trait Command {
    // Initializer for Command.
    fn new() -> Self;

    // Execute actions based on type of Command.
    // @param Option<Key> And optional key value.
    fn execute(
        &mut self,
        ButtonState,
        Option<Key>,
        player: &mut Creature,
        game_state: &mut GameState,
    );
}

/**
	Implementation of the OpenMenu Command.
*/
struct OpenMenu {}

impl Command for OpenMenu {
    fn new() -> Self {
        OpenMenu {}
    }

    // Unused param _key.
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
	Used when the player presses the action button (Return).
*/
struct Action {}

impl Command for Action {
    fn new() -> Self {
        Action {}
    }

    // Unused param _key.
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
	This will either move the player or move selections
	in a menu.

	@field dir The direction of the Command.
*/
struct Move {}

impl Command for Move {
    fn new() -> Self {
        Move {}
    }

    // @param key The input key.
    fn execute(
        &mut self,
        state: ButtonState,
        key: Option<Key>,
        player: &mut Creature,
        _game_state: &mut GameState,
    ) {
        if state == ButtonState::Press {
            let mut dx: f64 = 0.0;
            let mut dy: f64 = 0.0;

            if key == Some(Key::W) {
                dy -= player.speed;
            }
            if key == Some(Key::A) {
                dx -= player.speed;
            }
            if key == Some(Key::S) {
                dy += player.speed;
            }
            if key == Some(Key::D) {
                dx += player.speed;
            }
            player.change_self_velocity(dx, dy);
        }
        else if state == ButtonState::Release {
            if key == Some(Key::W) || key == Some(Key::S){
                player.reset_self_velocity_y();
            }
            if key == Some(Key::A) || key == Some(Key::D) {
                player.reset_self_velocity_x();
            }
        }

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
        state: ButtonState,
        button: Button,
        player: &mut Creature,
        game_state: &mut GameState,
    ) {
        use self::Key::*;
        match button {
            Button::Keyboard(key) => match key {
                Return => self.action.execute(state, None, player, game_state),
                Tab => self.open_menu.execute(state, None, player, game_state),
                W | A | S | D => self.move_dir.execute(state, Some(key), player, game_state),
                _ => {}
            },
            _ => {}
        }
    }
}
