pub struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {

    fn new(width: &i32, height: &i32) -> Self {
        Self { width: *width, height: *height }
    }

    fn fits(&self, inner_rectangle: &Rectangle) -> bool {
        self.width >= inner_rectangle.width && self.height >= inner_rectangle.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smaller_fits_in_larger() {
        let smaller = Rectangle::new(&32, &32);
        let larger = Rectangle::new(&43, &56);
        assert!(larger.fits(&smaller))
    }

    #[test]
    fn larger_doesnot_fit_in_smaller() {
        let smaller = Rectangle::new(&32, &32);
        let larger = Rectangle::new(&43, &56);
        assert!(!smaller.fits(&larger))
    }
}