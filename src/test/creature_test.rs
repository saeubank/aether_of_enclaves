#[cfg(test)]
mod tests {

    #[test]
    fn position_updates() {
        use creature::*;
        let mut test_player = Creature::new(CreatureType::Player);

        // Player's pos initialized to 0.0.
        assert_eq!(test_player.x, 0.0);
        assert_eq!(test_player.y, 0.0);

        test_player.self_vel_x = 5.0;
        test_player.self_vel_y = 7.0;

        test_player.update_position();
        assert_eq!(test_player.x, 5.0);
        assert_eq!(test_player.y, 7.0);

        drop(test_player);
    }
}
