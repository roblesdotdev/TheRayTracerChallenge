#[derive(Debug)]    
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
       Self {
            x, y, z, w,
        } 
    }

    pub(crate) fn point(x: f64, y: f64, z: f64) -> Self {
        Self::new(x, y, z, 1.0)
    }

    pub(crate) fn vector(x: f64, y: f64, z: f64) -> Self {
        Self::new(x, y, z, 0.0)
    }

    pub(crate) fn is_vector(&self) -> bool {
        equal(self.w, 0.0)
    }

    pub(crate) fn is_point(&self) -> bool {
        equal(self.w, 1.0)
    }
}

pub fn equal(left: f64, right: f64) -> bool {
    const EPSILON: f64 = 0.00001;
    (left - right).abs() < EPSILON
}

#[cfg(test)]
mod tests {
    use crate::tuple::*;

    #[test]
    fn can_be_create_a_point() {
        let point = Tuple::point(4.3, -4.2, 3.1);
        assert_eq!(point.x, 4.3);
        assert_eq!(point.y, -4.2);
        assert_eq!(point.z, 3.1);
        assert_eq!(point.w, 1.0);
    }

    #[test]
    fn can_be_create_a_vector() {
        let vector = Tuple::vector(4.3, -4.2, 3.1);
        assert_eq!(vector.x, 4.3);
        assert_eq!(vector.y, -4.2);
        assert_eq!(vector.z, 3.1);
        assert_eq!(vector.w, 0.0);
    }

    #[test]
    fn can_be_check_if_is_point_or_vector() {
        let vector = Tuple::vector(4.3, -4.2, 3.1);
        let point = Tuple::point(4.3, -4.2, 3.1);
        assert!(vector.is_vector());
        assert!(point.is_point());
        assert_eq!(point.is_vector(), false);
        assert_eq!(vector.is_point(), false);
    }

    #[test]
    fn can_be_equal() {
        assert_eq!(equal(3.0001, 3.0001), true);
    }
}
