use std::ops::Add;

#[allow(dead_code)]
pub trait ModularAdd: Sized + Add<Self, Output = Self> {
    fn modular_add(&self, val: Self, modulus: Self) -> Self;
    fn modular_inc(&mut self, modulus: Self);
}

#[allow(dead_code)]
pub trait ModularSub: Sized + Add<Self, Output = Self> {
    fn modular_sub(&self, val: Self, modulus: Self) -> Self;
    fn modular_dec(&mut self, modulus: Self);
}

impl ModularAdd for usize {
    fn modular_add(&self, val: Self, modulus: Self) -> Self {
        (*self + val) % modulus
    }

    fn modular_inc(&mut self, modulus: Self) {
        *self = self.modular_add(1, modulus);
    }
}

impl ModularSub for usize {
    fn modular_sub(&self, val: Self, modulus: Self) -> Self {
        if *self >= val {
            (*self - val) % modulus
        } else {
            (modulus + *self - val) % modulus
        }
    }

    fn modular_dec(&mut self, modulus: Self) {
        *self = self.modular_sub(1, modulus)
    }
}

impl ModularAdd for u32 {
    fn modular_add(&self, val: Self, modulus: Self) -> Self {
        (*self + val) % modulus
    }

    fn modular_inc(&mut self, modulus: Self) {
        *self = self.modular_add(1, modulus);
    }
}

impl ModularSub for u32 {
    fn modular_sub(&self, val: Self, modulus: Self) -> Self {
        if *self >= val {
            *self - (val % modulus)
        } else {
            (modulus + *self - (val % modulus)) % modulus
        }
    }

    fn modular_dec(&mut self, modulus: Self) {
        *self = self.modular_sub(1, modulus)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modular_add_usize() {
        for (val, magnitude, modulus, expected) in [
            (52_usize, 48_usize, 100_usize, 0_usize),
            (95_usize, 60_usize, 100_usize, 55_usize),
            (0_usize, 14_usize, 100_usize, 14_usize),
            (0_usize, 5400_usize, 100_usize, 0_usize),
        ] {
            let actual = val.modular_add(magnitude, modulus);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_modular_add_u32() {
        for (val, magnitude, modulus, expected) in [
            (52_u32, 48_u32, 100_u32, 0_u32),
            (95_u32, 60_u32, 100_u32, 55_u32),
            (0_u32, 14_u32, 100_u32, 14_u32),
            (0_u32, 5400_u32, 100_u32, 0_u32),
        ] {
            let actual = val.modular_add(magnitude, modulus);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_modular_sub_usize() {
        for (val, magnitude, modulus, expected) in [
            (50_usize, 68_usize, 100_usize, 82_usize),
            (82_usize, 30_usize, 100_usize, 52_usize),
            (0_usize, 5_usize, 100_usize, 95_usize),
            (55_usize, 55_usize, 100_usize, 0_usize),
            (0_usize, 1_usize, 100_usize, 99_usize),
            (99_usize, 99_usize, 100_usize, 0_usize),
            (14_usize, 82_usize, 100_usize, 32_usize),
        ] {
            let actual = val.modular_sub(magnitude, modulus);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_modular_sub_u32() {
        for (val, magnitude, modulus, expected) in [
            (50_u32, 68_u32, 100_u32, 82_u32),
            (82_u32, 30_u32, 100_u32, 52_u32),
            (0_u32, 5_u32, 100_u32, 95_u32),
            (55_u32, 55_u32, 100_u32, 0_u32),
            (0_u32, 1_u32, 100_u32, 99_u32),
            (99_u32, 99_u32, 100_u32, 0_u32),
            (14_u32, 82_u32, 100_u32, 32_u32),
            (85_u32, 4404_u32, 100_u32, 81_u32),
        ] {
            let actual = val.modular_sub(magnitude, modulus);
            assert_eq!(actual, expected);
        }
    }
}
