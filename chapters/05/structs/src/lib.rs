#[allow(dead_code)]
struct Rectangle {
    height: u64,
    width: u64,
}

#[allow(dead_code)]
impl Rectangle {
    fn new(height: u64, width: u64) -> Self {
        Self { height, width }
    }

    fn area(&self) -> u64 {
        self.height * self.width
    }

    fn can_cover(&self, rect: &Rectangle) -> bool {
        self.width >= rect.width && self.height >= rect.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let rect = Rectangle::new(5, 10);
        assert_eq!(rect.height, 5);
        assert_eq!(rect.width, 10);
    }

    #[test]
    fn test_basic() {
        let rect = Rectangle::new(5, 10);
        assert_eq!(rect.area(), 50);

        let rect2 = Rectangle::new(3, 8);
        assert!(rect.can_cover(&rect2));
    }
}

