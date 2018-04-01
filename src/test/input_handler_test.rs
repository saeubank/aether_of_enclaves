/**
    Testing of the Input Handler object.
*/

#[cfg(test)]

mod tests {

    #[test]
    fn toggles_menu() {
        use input_handler::*;
        use creature::*;
        use piston_window::{Button, ButtonState, Key};
        use game::GameState;

        let mut test_handler = InputHandler::new();
        let mut test_player = Creature::new(CreatureType::Player);
        let mut test_gs = GameState::InGame;

        // Should open menu.
        test_handler.handle_input(
            ButtonState::Press,
            Button::Keyboard(Key::Tab),
            &mut test_player,
            &mut test_gs,
        );
        assert_eq!(test_gs, GameState::InMenu);

        // Should close menu.
        test_handler.handle_input(
            ButtonState::Press,
            Button::Keyboard(Key::Tab),
            &mut test_player,
            &mut test_gs,
        );
        assert_eq!(test_gs, GameState::InGame);

        drop(test_handler);
        drop(test_player);
        drop(test_gs);
    }

    #[test]
    fn begins_game() {
        use input_handler::*;
        use creature::*;
        use piston_window::{Button, ButtonState, Key};
        use game::GameState;

        let mut test_handler = InputHandler::new();
        let mut test_player = Creature::new(CreatureType::Player);
        let mut test_gs = GameState::Title;

        // Should begin game.
        test_handler.handle_input(
            ButtonState::Press,
            Button::Keyboard(Key::Return),
            &mut test_player,
            &mut test_gs,
        );
        assert_eq!(test_gs, GameState::InGame);

        drop(test_handler);
        drop(test_player);
        drop(test_gs);
    }

    #[test]
    fn moves_player() {
        use input_handler::*;
        use creature::*;
        use piston_window::{Button, ButtonState, Key};
        use game::GameState;

        let mut test_handler = InputHandler::new();
        let mut test_player = Creature::new(CreatureType::Player);
        let mut test_gs = GameState::InGame;

        // Should move Player to the right.
        test_handler.handle_input(
            ButtonState::Press,
            Button::Keyboard(Key::D),
            &mut test_player,
            &mut test_gs,
        );
        assert_eq!(test_player.self_vel_x, test_player.speed);

        // Should move Player down.
        test_handler.handle_input(
            ButtonState::Press,
            Button::Keyboard(Key::S),
            &mut test_player,
            &mut test_gs,
        );
        assert_eq!(test_player.self_vel_y, test_player.speed);

        // Player should stop moving horizontally.
        test_handler.handle_input(
            ButtonState::Release,
            Button::Keyboard(Key::D),
            &mut test_player,
            &mut test_gs,
        );
        assert_eq!(test_player.self_vel_x, 0.0);

        // Player should stop moving vertically.
        test_handler.handle_input(
            ButtonState::Release,
            Button::Keyboard(Key::S),
            &mut test_player,
            &mut test_gs,
        );
        assert_eq!(test_player.self_vel_y, 0.0);

        drop(test_handler);
        drop(test_player);
        drop(test_gs);
    }
}
