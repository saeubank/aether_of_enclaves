#[cfg(test)]
mod tests {

    #[test]
    fn position_updates() {
        use creature::*;
        let mut test_player = Creature::new(CreatureType::Player);

        // Player's position initialized to 0.0.
        assert_eq!(test_player.x, 0.0);
        assert_eq!(test_player.y, 0.0);

        test_player.self_vel_x = 5.0;
        test_player.self_vel_y = 7.0;
        test_player.update_position();

        assert_eq!(test_player.x, 5.0);
        assert_eq!(test_player.y, 7.0);

        drop(test_player);
    }

    #[test]
    fn self_velocity_changes() {
        use creature::*;
        let mut test_player = Creature::new(CreatureType::Player);

        // Player's velocity initialized to 0.0.
        assert_eq!(test_player.self_vel_x, 0.0);
        assert_eq!(test_player.self_vel_y, 0.0);

        test_player.change_self_velocity(-10.0, 5.0);

        assert_eq!(test_player.self_vel_x, -5.0);
        assert_eq!(test_player.self_vel_y, 5.0);

        drop(test_player);
    }

    #[test]
    fn self_velocity_resets() {
        use creature::*;
        let mut test_player = Creature::new(CreatureType::Player);

        test_player.self_vel_x = 5.0;
        test_player.self_vel_y = 5.0;

        test_player.reset_self_velocity_x();
        assert_eq!(test_player.self_vel_x, 0.0);

        test_player.reset_self_velocity_y();
        assert_eq!(test_player.self_vel_y, 0.0);

        drop(test_player);
    }

}
