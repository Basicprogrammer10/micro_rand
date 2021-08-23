/// Random Generator
pub struct Random {
    seed: i64,
    a: i64,
    c: i64,
    m: i64,

    start_m: i64,
}

impl Random {
    /// Make a new random generator
    ///
    /// Used default values for a, c and m
    ///```text
    /// a: 16807
    /// c: 0
    /// m: 2147483647
    ///```
    /// ## Example
    /// ```rust
    /// // Import Lib
    /// use micro_rand::Random;
    ///
    /// // Make a new random generator with seed 1234
    /// let mut r = Random::new(1234);
    /// ```
    pub fn new(seed: i64) -> Random {
        Random {
            seed,
            a: 16_807,
            c: 0,
            m: 2_147_483_647,
            start_m: 2_147_483_647,
        }
    }

    /// Make a new random generator with custom values for a, c and m
    /// ## Example
    /// ```rust
    /// // Import Lib
    /// use micro_rand::Random;
    ///
    /// // Make a new, custom random generator
    /// let mut r = Random::custom_new(1234, 86284, 2, 7263957720);
    /// ```
    pub fn custom_new(seed: i64, a: i64, c: i64, m: i64) -> Random {
        Random {
            seed,
            a,
            c,
            m,
            start_m: m,
        }
    }

    /// Geth the next float 64 from a generator
    /// ## Example
    /// ```rust
    /// // Import Lib
    /// use micro_rand::Random;
    ///
    /// // Make a new, custom random generator
    /// let mut r = Random::new(1234);
    ///
    /// // Get the next float 64
    /// let f = r.next_f64();
    /// ```
    pub fn next_f64(&mut self) -> f64 {
        let seed = self.seed;
        let a = self.a;
        let c = self.c;
        let m = self.m;
        self.seed = (a * seed + c) % m;
        self.seed as f64 / self.start_m as f64
    }

    /// Geth the next float 32 from a generator
    /// ## Example
    /// ```rust
    /// // Import Lib
    /// use micro_rand::Random;
    ///
    /// // Make a new, custom random generator
    /// let mut r = Random::new(1234);
    ///
    /// // Get the next float 32
    /// let f = r.next_f32();
    /// ```
    pub fn next_f32(&mut self) -> f32 {
        self.next_f64() as f32
    }

    /// Geth the next i64 from a generator within a range
    /// ## Example
    /// ```rust
    /// // Import Lib
    /// use micro_rand::Random;
    ///
    /// // Make a new, custom random generator
    /// let mut r = Random::new(1234);
    ///
    /// // Get a random i64 between 0 and 100
    /// let f = r.next_int_i64(0, 100);
    /// ```
    pub fn next_int_i64(&mut self, min: i64, max: i64) -> i64 {
        let x = self.next_f64();
        (x * (max - min + 1) as f64 + min as f64) as i64
    }

    /// Geth the next i32 from a generator within a range
    /// ## Example
    /// ```rust
    /// // Import Lib
    /// use micro_rand::Random;
    ///
    /// // Make a new, custom random generator
    /// let mut r = Random::new(1234);
    ///
    /// // Get a random i32 between 0 and 100
    /// let f = r.next_int_i32(0, 100);
    /// ```
    pub fn next_int_i32(&mut self, min: i32, max: i32) -> i32 {
        let x = self.next_f32();
        (x * (max - min + 1) as f32 + min as f32) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Random;

    #[test]
    fn test_new() {
        let r = Random::new(1234);
        assert_eq!(r.seed, 1234);
        assert_eq!(r.a, 16_807);
        assert_eq!(r.c, 0);
        assert_eq!(r.m, 2_147_483_647);
        assert_eq!(r.start_m, 2_147_483_647);
    }

    #[test]
    fn test_custom_new() {
        let r = Random::custom_new(4321, 86_284, 2, 7_263_957_720);
        assert_eq!(r.seed, 4321);
        assert_eq!(r.a, 86_284);
        assert_eq!(r.c, 2);
        assert_eq!(r.m, 7263957720);
    }

    #[test]
    fn test_next_f64() {
        let mut r = Random::new(1234);
        assert_eq!(r.next_f64(), 0.009657739666131204);
        assert_eq!(r.next_f64(), 0.3176305686671429);
        assert_eq!(r.next_f64(), 0.41696758867100236);
    }

    #[test]
    fn test_next_f32() {
        let mut r = Random::new(1234);
        assert_eq!(r.next_f32(), 0.009657739666131204f32);
        assert_eq!(r.next_f32(), 0.3176305686671429f32);
        assert_eq!(r.next_f32(), 0.41696758867100236f32);
    }

    #[test]
    fn test_next_int_i64() {
        let mut r = Random::new(1234);
        assert_eq!(r.next_int_i64(0, 100), 0);
        assert_eq!(r.next_int_i64(0, 100), 32);
        assert_eq!(r.next_int_i64(0, 100), 42);
    }

    #[test]
    fn test_next_int_i32() {
        let mut r = Random::new(1234);
        assert_eq!(r.next_int_i32(0, 100), 0);
        assert_eq!(r.next_int_i32(0, 100), 32);
        assert_eq!(r.next_int_i32(0, 100), 42);
    }
}
