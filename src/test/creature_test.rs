#[cfg(test)]
mod tests {

    #[test]
    fn position_updates() {
        use creature::*;
        use input_handler::Direction;
        let mut test_player = Creature::new(CreatureType::Player);
        
        // Player's pos initialized to 0.0.
        assert_eq!(test_player.x, 0.0);
        assert_eq!(test_player.y, 0.0);

        // Player moves Down 5.0.
        test_player.update_position(Some(Direction::Down), 5.0);
        assert_eq!(test_player.y, 5.0);

        // Player moves Right 10.5.
        test_player.update_position(Some(Direction::Right), 10.5);
        assert_eq!(test_player.x, 10.5);

        // Position should not change.
        test_player.update_position(None, 5.0);
        assert_eq!(test_player.x, 10.5);
        assert_eq!(test_player.y, 5.0);

        drop(test_player);
    }
}