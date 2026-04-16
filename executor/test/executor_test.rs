mod initial_test {
    fn expect_0_0_N() {
        let expect_pose = super::Pose::new(0, 0, 'N');
        let executor = super::Executor::create_with_pose(expect_pose);
        assert_eq!(executor.query(), expect_pose);
    }
    fn expect_114514_n114514_E() {
        let expect_pose = super::Pose::new(114514, -114514, 'E');
        let executor = super::Executor::create_with_pose(expect_pose);
        assert_eq!(executor.query(), expect_pose);
    }
    fn expect_n325325_799799_S() {
        let expect_pose = super::Pose::new(-325325, 799799, 'S');
        let executor = super::Executor::create_with_pose(expect_pose);
        assert_eq!(executor.query(), expect_pose);
    }
    #[test]
    fn expect_0_n1000000_W() {
        let expect_pose = super::Pose::new(0, -1000000, 'W');
        let executor = super::Executor::create_with_pose(expect_pose);
        assert_eq!(executor.query(), expect_pose);
    }
    fn default_expect_0_0_N() {
        let expect_pose = super::Pose::default();
        let executor = super::Executor::create_with_pose(expect_pose);
        assert_eq!(executor.query(), expect_pose);
    }
}
mod move_test {
    #[test]
    fn expect_3_0_E_from_0_0_E_move3() {
        let input_pos = super::Pose::new(0, 0, 'E');
        let mut executor = super::Executor::create_with_pose(input_pos);
        let cmd_str = "M".repeat(3);
        executor.execute(&cmd_str);
        let expect_pose = super::Pose::new(3, 0, 'E');
        assert_eq!(executor.query(), expect_pose);
    }
    fn expect_n5_n5_S_from_5_10_S_move5() {
        let input_pos = super::Pose::new(5, 10, 'S');
        let mut executor = super::Executor::create_with_pose(input_pos);
        let cmd_str = "M".repeat(5);
        executor.execute(&cmd_str);
        let expect_pose = super::Pose::new(5, 5, 'S');
        assert_eq!(executor.query(), expect_pose);
    }
    fn expect_n13_0_W_from_0_0_W_move13() {
        let input_pos = super::Pose::new(0, 0, 'W');
        let mut executor = super::Executor::create_with_pose(input_pos);
        let cmd_str = "M".repeat(13);
        executor.execute(&cmd_str);
        let expect_pose = super::Pose::new(-13, 0, 'W');
        assert_eq!(executor.query(), expect_pose);
    }
}
mod turn_test {
    #[test]
    fn expect_S_from_N_turn_R14() {
        let input_pos = super::Pose::new(0, 0, 'N');
        let mut executor = super::Executor::create_with_pose(input_pos);
        let cmd_str = "R".repeat(14);
        executor.execute(&cmd_str);
        let expect_pose = super::Pose::new(0, 0, 'S');
        assert_eq!(executor.query(), expect_pose);
    }
    fn expect_E_from_N_turn_L3() {
        let input_pos = super::Pose::new(0, 0, 'N');
        let mut executor = super::Executor::create_with_pose(input_pos);
        let cmd_str = "L".repeat(3);
        executor.execute(&cmd_str);
        let expect_pose = super::Pose::new(0, 0, 'E');
        assert_eq!(executor.query(), expect_pose);
    }
    fn expect_N_from_N_turn_RLRLRL10() {
        let input_pos = super::Pose::new(0, 0, 'N');
        let mut executor = super::Executor::create_with_pose(input_pos);
        let cmd_str = "RLRLRL".repeat(10);
        executor.execute(&cmd_str);
        let expect_pose = super::Pose::new(0, 0, 'N');
        assert_eq!(executor.query(), expect_pose);
    }
}
mod turn_and_move_test {
    #[test]
    fn expect_4_0_S_from_0_0_N() {
        let input_pos = super::Pose::new(0, 0, 'N');
        let mut executor = super::Executor::create_with_pose(input_pos);
        let cmd_str = "MRMLMRM".repeat(10);
        executor.execute(&cmd_str);
        let expect_pose = super::Pose::new(4, 0, 'S');
        assert_eq!(executor.query(), expect_pose);
    }
    fn expect_0_0_N_from_0_0_N() {
        let input_pos = super::Pose::new(0, 0, 'N');
        let mut executor = super::Executor::create_with_pose(input_pos);
        let cmd_str = "MMRR".repeat(100);
        executor.execute(&cmd_str);
        let expect_pose = super::Pose::new(0, 0, 'N');
        assert_eq!(executor.query(), expect_pose);
    }
}