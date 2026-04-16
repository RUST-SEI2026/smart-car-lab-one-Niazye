#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pose {
    pub x: i32,
    pub y: i32,
    pub heading: char,
}

impl Pose {
    pub fn new(x: i32, y: i32, heading: char) -> Self {
        Pose { x, y, heading }
    }
}

impl Default for Pose {
    fn default() -> Self {
        Pose {
            x: 0,
            y: 0,
            heading: 'N',
        }
    }
}

pub struct Executor {
    pose: Pose,
}

impl Executor {
    pub fn create_with_pose(pose: Pose) -> Self {
        Executor { pose }
    }

    pub fn execute(&mut self, cmds: &str) {
        
    }

    pub fn query(&self) -> Pose {
        self.pose
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output() {
        println!("Hello, Smart Car! Current pose is: {:?}", Pose::default());
        let a = 1;
        let b = 2;
        dbg!("a: {}, b: {}", a, b);
        dbg!("for test");
        assert_eq!(a + b, 3);
    }
}