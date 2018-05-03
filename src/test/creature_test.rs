//! Testing of the Creature object.

#[cfg(test)]

mod tests {

    #[test]
    fn position_updates() {
        use creature::*;
        use misc::Moveable;
        let mut test_player = Creature::new();

        // Player's position initialized to 0.0.
        assert_eq!(test_player.x, 0.0);
        assert_eq!(test_player.y, 0.0);

        test_player.self_vel_x = 5.0;
        test_player.self_vel_y = 7.0;
        test_player.update_position();

        // Player's position should move according to its velocity.
        assert_eq!(test_player.x, 5.0);
        assert_eq!(test_player.y, 7.0);

        test_player.other_vel_x = 4.0;
        test_player.other_vel_y = 4.0;

        test_player.update_position();

         // Player's position should move according to its velocity and velocities acting on it.
        assert_eq!(test_player.x, 14.0);
        assert_eq!(test_player.y, 18.0);

        test_player.self_vel_x = 0.0;
        test_player.self_vel_y = 0.0;

        test_player.update_position();

        // Player's position should move according only to velocities acting on it.
        assert_eq!(test_player.x, 18.0);
        assert_eq!(test_player.y, 22.0);

        drop(test_player);
    }

    #[test]
    fn self_velocity_updates() {
        use creature::*;
        use misc::*;
        let mut test_player = Creature::new();

        // Player's velocity initialized to 0.0.
        assert_eq!(test_player.self_vel_x, 0.0);
        assert_eq!(test_player.self_vel_y, 0.0);

        test_player.speed = 3.0;
        test_player.directions = vec![Direction::N, Direction::E];

        test_player.update_self_velocity();

        // Player's velocity should update based on direction it's moving.
        assert_eq!(test_player.self_vel_x, 3.0);
        assert_eq!(test_player.self_vel_y, -3.0);

        drop(test_player);
    }


    #[test]
    fn control_state_changes() {
        use creature::*;
        let mut test_player = Creature::new();

        // Player's state initialized to Normal.
        assert_eq!(test_player.creature_state, CreatureState::Normal);

        test_player.self_vel_y = 3.0;
        test_player.self_vel_x = 3.0;
        test_player.change_control_state();

        // State should toggle.
        assert_eq!(test_player.creature_state, CreatureState::ControllingShip);

        // If player was moving, should no longer be.
        assert_eq!(test_player.self_vel_x, 0.0);
        assert_eq!(test_player.self_vel_y, 0.0);

        test_player.change_control_state();

        //Toggle again.
        assert_eq!(test_player.creature_state, CreatureState::Normal);

        drop(test_player);

    }
}
