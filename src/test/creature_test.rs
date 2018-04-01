/**
    Testing of the Creature object.
*/

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

        // Player's position should move according to its velocity.
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

        // Player's velocity should be capped at its speed.
        assert_eq!(test_player.self_vel_x, -1.0 * test_player.speed);
        assert_eq!(test_player.self_vel_y, test_player.speed);

        drop(test_player);
    }

    #[test]
    fn self_velocity_resets() {
        use creature::*;
        let mut test_player = Creature::new(CreatureType::Player);

        test_player.self_vel_x = test_player.speed;
        test_player.self_vel_y = test_player.speed;

        // Set Player's horizontal velocity back to 0.
        test_player.reset_self_velocity_x();
        assert_eq!(test_player.self_vel_x, 0.0);

        // Set Player's vertical velocity back to 0.
        test_player.reset_self_velocity_y();
        assert_eq!(test_player.self_vel_y, 0.0);

        drop(test_player);
    }

}
