pub fn add(n: f32, m: f32) -> f32 {
    m + n
}

pub fn multiply(n: f32, m: f32) -> f32 {
    n * m
}

#[cfg(test)]
mod tests {
    use crate::math::{add, multiply};


    #[test]
    fn adder_fn_adds_two_numbers() {
        let result = add(23.2, 32.6);
        assert_eq!(result, 55.8);
    }

    #[test]
    fn multiplier_multiplie_two() {
        assert_eq!(multiply(3.2, 3.2), 9.3);
    }
}
