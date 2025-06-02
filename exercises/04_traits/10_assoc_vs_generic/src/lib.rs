// TODO: Define a new trait, `Power`, that has a method `power` that raises `self`
//  to the power of `n`.
//  The trait definition and its implementations should be enough to get
//  the tests to compile and pass.
//
// Recommendation: you may be tempted to write a generic implementation to handle
// all cases at once. However, this is fairly complicated and requires the use of
// additional crates (i.e. `num-traits`).
// Even then, it might be preferable to use a simple macro instead to avoid
// the complexity of a highly generic implementation. Check out the
// "Little book of Rust macros" (https://veykril.github.io/tlborm/) if you're
// interested in learning more about it.
// You don't have to though: it's perfectly okay to write three separate
// implementations manually. Venture further only if you're curious.

pub trait Power<T> {
    type Target;
    fn power(self, n: T) -> Self::Target;
}

impl Power<u16> for u32 {
    type Target = u32;
    fn power(self, mut n: u16) -> Self::Target {
        let mut i: Self::Target = self;
        while n > 1 {
            i = i * self;
            n = n - 1;
        }
        i
    }
}

impl Power<u32> for u32 {
    type Target = u32;
    fn power(self, mut n: u32) -> Self::Target {
        let mut i: Self::Target = self;
        while n > 1 {
            i = i * self;
            n = n - 1;
        }
        i
    }
}
impl Power<&u32> for u32 {
    type Target = u32;
    fn power(self, n: &u32) -> Self::Target {
        let mut a: u32 = *n;
        let mut i: Self::Target = self;
        while a > 1 {
            i = i * self;
            a = a - 1;
        }
        i
    }
}

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
