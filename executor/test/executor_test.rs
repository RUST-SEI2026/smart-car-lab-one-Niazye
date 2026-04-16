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