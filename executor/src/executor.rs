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

    fn move_step(&mut self) {
        match self.pose.heading {
            'N' => self.pose.y += 1,
            'S' => self.pose.y -= 1,
            'E' => self.pose.x += 1,
            'W' => self.pose.x -= 1,
            _ => (),
        };
    }
    
    fn turn(&mut self, direction: char) {
        let headings = ['N', 'E', 'S', 'W'];
        let idx = match self.pose.heading {
            'N' => 0,
            'E' => 1,
            'S' => 2,
            'W' => 3,
            _ => 0,
        };
        let new_idx = match direction {
            'R' => (idx + 1) % 4,
            'L' => (idx + 3) % 4,
            _ => 0,
        };
        self.pose.heading = headings[new_idx];
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