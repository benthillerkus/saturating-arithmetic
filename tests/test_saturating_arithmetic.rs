extern crate saturating_arithmetic;
use saturating_arithmetic::saturateit;

#[test]
fn test_mul() {
    #[saturateit]
    fn mul() -> u32 {
        let a: u32 = std::u32::MAX;
        a * 2
    }
    assert_eq!(std::u32::MAX, mul());
}

#[test]
fn test_add() {
    #[saturateit]
    fn add() -> u32 {
        let a: u32 = std::u32::MAX;
        a + 2
    }
    assert_eq!(std::u32::MAX, add());
    // TODO: assert_eq!(1u32, a + b as u32); won't work because the macro expansion get's in the way.
}

#[test]
fn test_sub() {
    #[saturateit]
    fn sub() -> u32 {
        let a: u32 = 0;
        a - 1
    }
    assert_eq!(std::u32::MIN, sub());
}

#[test]
fn test_mul_assign() {
    #[saturateit]
    fn mul() -> u32 {
        let mut a: u32 = std::u32::MAX;
        a *= 2;
        a
    }
    assert_eq!(0xffffffffu32, mul());
}

#[test]
fn test_add_assign() {
    #[saturateit]
    fn add() -> u32 {
        let mut a: u32 = std::u32::MAX;
        a += 2;
        a
    }
    assert_eq!(std::u32::MAX, add());
}

#[test]
fn test_sub_assign() {
    #[saturateit]
    fn sub() -> u32 {
        let mut a: u32 = 0;
        a -= 1;
        a
    }
    assert_eq!(std::u32::MIN, sub());
}

#[test]
#[should_panic]
#[allow(arithmetic_overflow)]
fn test_mul_panic() {
    fn mul() -> u32 {
        let a: u32 = std::u32::MAX;
        a * 2
    }
    assert_eq!(std::u32::MAX, mul());
}

#[test]
#[should_panic]
#[allow(arithmetic_overflow)]
fn test_add_panic() {
    fn add() -> u32 {
        let a: u32 = std::u32::MAX;
        a + 2
    }
    assert_eq!(std::u32::MAX, add());
    // TODO: assert_eq!(1u32, a + b as u32); won't work because the macro expansion get's in the way.
}

#[test]
#[should_panic]
#[allow(arithmetic_overflow)]
fn test_mul_assign_panic() {
    fn mul() -> u32 {
        let mut a: u32 = std::u32::MAX;
        a *= 2;
        a
    }
    assert_eq!(std::u32::MAX, mul());
}

#[test]
#[should_panic]
#[allow(arithmetic_overflow)]
fn test_add_assign_panic() {
    fn add() -> u32 {
        let mut a: u32 = std::u32::MAX;
        a += 2;
        a
    }
    assert_eq!(std::u32::MAX, add());
}
