#[cfg(test)]
mod tests {

    #[test]
    fn toggles_menu() {
        use input_handler::*;
        use creature::*;
        use piston_window::{Button, Key};
        use game::GameState;

        let mut test_handler = InputHandler::new();
        let mut test_player = Creature::new(CreatureType::Player);
        let mut test_gs = GameState::Main;

        // Should open menu.
        test_handler.handle_input(Button::Keyboard(Key::Tab), &mut test_player, &mut test_gs);
        assert_eq!(test_gs, GameState::InMenu);

        // Should close menu.
        test_handler.handle_input(Button::Keyboard(Key::Tab), &mut test_player, &mut test_gs);
        assert_eq!(test_gs, GameState::Main);

        drop(test_handler);
        drop(test_player);
        drop(test_gs);
    }

    #[test]
    fn moves_player() {
        use input_handler::*;
        use creature::*;
        use piston_window::{Button, Key};
        use game::GameState;

        let mut test_handler = InputHandler::new();
        let mut test_player = Creature::new(CreatureType::Player);
        let mut test_gs = GameState::Main;

        // Should move Player to the right.
        test_handler.handle_input(Button::Keyboard(Key::D), &mut test_player, &mut test_gs);

        assert_eq!(test_player.x, 15.0);
        assert_eq!(test_gs, GameState::Main);

        drop(test_handler);
        drop(test_player);
        drop(test_gs);
    }
}
