use super::*;

#[test]
fn constructors() {
    let small_int = U1024::from_small(123456789);
    let u128_int = U1024::from_u128(123456789);
    assert_eq!(small_int, u128_int);
}

#[test]
fn operators() {
    let a = U1024::from_small(562319);
    let b = U1024::from_small(2349);
    assert!(a != b);
    assert!(a > b);
    assert_eq!(U1024::from_small(564668), a + b);
    assert_eq!(U1024::from_small(1320887331), a * b);
    assert_eq!(U1024::from_small(559970), a - b);
    assert_eq!(U1024::from_small(239), a / b);
    assert_eq!(U1024::from_small(908), a % b);
    let mut x = U1024::from_small(562319);
    x += b;
    assert_eq!(U1024::from_small(564668), x);
    let mut x = U1024::from_small(562319);
    x *= b;
    assert_eq!(U1024::from_small(1320887331), x);
    let mut x = U1024::from_small(562319);
    x -= b;
    assert_eq!(U1024::from_small(559970), x);
    let mut x = U1024::from_small(562319);
    x /= b;
    assert_eq!(U1024::from_small(239), x);
    let mut x = U1024::from_small(562319);
    x %= b;
    assert_eq!(U1024::from_small(908), x);
}

#[test]
fn pow() {
    let base: u64 = 7;
    let exp = 11;
    // 1 977 326 743
    println!("{}", base.pow(exp));
    assert_eq!(
        U1024::from_small(base.pow(exp)),
        U1024::from_small(base).pow(exp)
    );
}

#[test]
fn fmt_display() {
    let a = U1024::from_small(u64::MAX);
    let b = U1024::from_u128(u128::MAX);
    let z = U1024::from_small(0);
    let m = U1024::MAX;

    assert_eq!(u64::MAX.to_string(), a.to_string());
    assert_eq!(u128::MAX.to_string(), b.to_string());
    assert_eq!("0", z.to_string());
    assert_eq!("179769313486231590772930519078902473361797697894230657273430081157732675805500963132708477322407536021120113879871393357658789768814416622492847430639474124377767893424865485276302219601246094119453082952085005768838150682342462881473913110540827237163350510684586298239947245938479716304835356329624224137215", m.to_string());
}

///////////////////////////////////////////////////////////////////////////

#[test]
fn test_add() {
    assert_eq!(
        *U1024::from_small(3).plus(&U1024::from_small(4)),
        U1024::from_small(7)
    );
    assert_eq!(
        *U1024::from_small(3).plus(&U1024::from_small(0)),
        U1024::from_small(3)
    );
    assert_eq!(
        *U1024::from_small(0).plus(&U1024::from_small(3)),
        U1024::from_small(3)
    );
    assert_eq!(
        *U1024::from_small(3).plus(&U1024::from_small(0xfffe)),
        U1024::from_small(0x10001)
    );
    assert_eq!(
        *U1024::from_small(0xfedc).plus(&U1024::from_small(0x789)),
        U1024::from_small(0x10665)
    );
    assert_eq!(
        *U1024::from_small(0x789).plus(&U1024::from_small(0xfedc)),
        U1024::from_small(0x10665)
    );
}

#[test]
#[should_panic]
fn test_add_overflow_1() {
    U1024::from_small(1).plus(&U1024::MAX);
}

#[test]
#[should_panic]
fn test_add_overflow_2() {
    let mut x = U1024::MAX;
    x.plus(&U1024::from_small(1));
}

#[test]
fn test_add_small() {
    assert_eq!(*U1024::from_small(3).plus_small(4), U1024::from_small(7));
    assert_eq!(*U1024::from_small(3).plus_small(0), U1024::from_small(3));
    assert_eq!(*U1024::from_small(0).plus_small(3), U1024::from_small(3));
    assert_eq!(
        *U1024::from_small(7).plus_small(250),
        U1024::from_small(257)
    );
    assert_eq!(
        *U1024::from_small(0x7fff).plus_small(1),
        U1024::from_small(0x8000)
    );
    assert_eq!(
        *U1024::from_small(0x2ffe).plus_small(0x35),
        U1024::from_small(0x3033)
    );
    assert_eq!(
        *U1024::from_small(0xdc).plus_small(0x89),
        U1024::from_small(0x165)
    );
}

#[test]
#[should_panic]
fn test_add_small_oveflow() {
    let mut x = U1024::MAX;
    x.plus_small(1);
}

#[test]
fn test_sub() {
    assert_eq!(
        *U1024::from_small(7).minus(&U1024::from_small(4)),
        U1024::from_small(3)
    );
    assert_eq!(
        *U1024::from_small(0x10665).minus(&U1024::from_small(0x789)),
        U1024::from_small(0xfedc)
    );
    assert_eq!(
        *U1024::from_small(0x10665).minus(&U1024::from_small(0xfedc)),
        U1024::from_small(0x789)
    );
    assert_eq!(
        *U1024::from_small(0x10665).minus(&U1024::from_small(0x10664)),
        U1024::from_small(1)
    );
    assert_eq!(
        *U1024::from_small(0x10665).minus(&U1024::from_small(0x10665)),
        U1024::from_small(0)
    );
}

#[test]
#[should_panic]
fn test_sub_underflow_1() {
    U1024::from_small(0x10665).minus(&U1024::from_small(0x10666));
}

#[test]
#[should_panic]
fn test_sub_underflow_2() {
    U1024::from_small(0).minus(&U1024::from_small(0x123456));
}

#[test]
fn test_mul_small() {
    assert_eq!(*U1024::from_small(7).mul_small(5), U1024::from_small(35));
    assert_eq!(
        *U1024::from_small(0xff).mul_small(0xff),
        U1024::from_small(0xfe01)
    );
    assert_eq!(
        *U1024::from_small(0xffffff / 13).mul_small(13),
        U1024::from_small(0xffffff)
    );
}

#[test]
#[should_panic]
fn test_mul_small_overflow() {
    U1024 {
        size: 16,
        base: [
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0x8000000000000000,
        ],
    }
    .mul_small(2);
}

#[test]
fn test_mul_pow2() {
    assert_eq!(*U1024::from_small(0x7).mul_pow2(4), U1024::from_small(0x70));
    assert_eq!(
        *U1024::from_small(0xff).mul_pow2(1),
        U1024::from_small(0x1fe)
    );
    assert_eq!(
        *U1024::from_small(0xff).mul_pow2(12),
        U1024::from_small(0xff000)
    );
    assert_eq!(
        *U1024::from_small(0x1).mul_pow2(23),
        U1024::from_small(0x800000)
    );
    assert_eq!(
        *U1024::from_small(0x123).mul_pow2(0),
        U1024::from_small(0x123)
    );
    assert_eq!(
        *U1024::from_small(0x123).mul_pow2(7),
        U1024::from_small(0x9180)
    );
    assert_eq!(
        *U1024::from_small(0x123).mul_pow2(15),
        U1024::from_small(0x918000)
    );
    assert_eq!(*U1024::from_small(0).mul_pow2(23), U1024::from_small(0));
}

#[test]
#[should_panic]
fn test_mul_pow2_overflow_1() {
    U1024::from_small(0x1).mul_pow2(1024);
}

#[test]
#[should_panic]
fn test_mul_pow2_overflow_2() {
    U1024::from_small(0x123).mul_pow2(1016);
}

#[test]
fn test_mul_pow5() {
    assert_eq!(*U1024::from_small(42).mul_pow5(0), U1024::from_small(42));
    assert_eq!(*U1024::from_small(1).mul_pow5(2), U1024::from_small(25));
    assert_eq!(
        *U1024::from_small(1).mul_pow5(4),
        U1024::from_small(25 * 25)
    );
    assert_eq!(*U1024::from_small(4).mul_pow5(3), U1024::from_small(500));
    assert_eq!(
        *U1024::from_small(140).mul_pow5(2),
        U1024::from_small(25 * 140)
    );
    assert_eq!(*U1024::from_small(25).mul_pow5(1), U1024::from_small(125));
    assert_eq!(
        *U1024::from_small(125).mul_pow5(7),
        U1024::from_small(9765625)
    );
    assert_eq!(*U1024::from_small(0).mul_pow5(127), U1024::from_small(0));
}

#[test]
#[should_panic]
fn test_mul_pow5_overflow_1() {
    U1024::from_small(1).mul_pow5(442);
}

#[test]
#[should_panic]
fn test_mul_pow5_overflow_2() {
    U1024::from_small(230).mul_pow5(438);
}

#[test]
fn test_mul_digits() {
    assert_eq!(
        *U1024::from_small(3).mul_digits(&[5]),
        U1024::from_small(15)
    );
    assert_eq!(
        *U1024::from_small(0xff).mul_digits(&[0xff]),
        U1024::from_small(0xfe01)
    );
    assert_eq!(
        *U1024::from_small(0x123).mul_digits(&[0x56, 0x4]),
        U1024::from_u128(0x48c_00000000000061c2)
    );
    assert_eq!(
        *U1024::from_small(0x12345).mul_digits(&[0x67]),
        U1024::from_small(0x7530c3)
    );
    assert_eq!(
        *U1024::from_small(0xff).mul_digits(&[0x1a, 0x2b, 0x3c]),
        U1024 {
            size: 3,
            base: [
                0x19e6, 0x2ad5, 0x3bc4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        }
    );
    assert_eq!(
        *U1024::from_small(0xffffff / 13).mul_digits(&[13]),
        U1024::from_small(0xffffff)
    );
    assert_eq!(
        *U1024::from_small(13).mul_digits(&[0x3b, 0xb1, 0x13]),
        U1024 {
            size: 3,
            base: [0x2ff, 0x8fd, 0xf7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        }
    );
}

#[test]
#[should_panic]
fn test_mul_digits_overflow_1() {
    U1024 {
        size: 16,
        base: [
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0x8000000000000000,
        ],
    }
    .mul_digits(&[2]);
}

#[test]
#[should_panic]
fn test_mul_digits_overflow_2() {
    U1024 {
        size: 16,
        base: [
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0x800000000000000,
        ],
    }
    .mul_digits(&[0, 0x10]);
}

#[test]
fn test_div_rem_small() {
    let as_val = |(q, r): (&mut U1024, u64)| (*q, r);
    assert_eq!(
        as_val(U1024::from_small(0xff).div_rem_small(15)),
        (U1024::from_small(17), 0)
    );
    assert_eq!(
        as_val(U1024::from_small(0xff).div_rem_small(16)),
        (U1024::from_small(15), 15)
    );
    assert_eq!(
        as_val(U1024::from_small(3).div_rem_small(40)),
        (U1024::from_small(0), 3)
    );
    assert_eq!(
        as_val(U1024::from_small(0xffffff).div_rem_small(123)),
        (U1024::from_small(0xffffff / 123), (0xffffffu64 % 123))
    );
    assert_eq!(
        as_val(U1024::from_small(0x10000).div_rem_small(123)),
        (U1024::from_small(0x10000 / 123), (0x10000u64 % 123))
    );
}

#[test]
fn test_is_zero() {
    assert!(U1024::from_small(0).is_zero());
    assert!(!U1024::from_small(3).is_zero());
    assert!(!U1024::from_small(0x123).is_zero());
    assert!(!U1024::from_small(0xffffff)
        .minus(&U1024::from_small(0xfffffe))
        .is_zero());
    assert!(U1024::from_small(0xffffff)
        .minus(&U1024::from_small(0xffffff))
        .is_zero());
}

#[test]
fn test_get_bit() {
    let x = U1024::from_small(0b1101);
    assert_eq!(x.get_bit(0), 1);
    assert_eq!(x.get_bit(1), 0);
    assert_eq!(x.get_bit(2), 1);
    assert_eq!(x.get_bit(3), 1);
    let y = U1024::from_small(1 << 15);
    assert_eq!(y.get_bit(14), 0);
    assert_eq!(y.get_bit(15), 1);
    assert_eq!(y.get_bit(16), 0);
}

#[test]
#[should_panic]
fn test_get_bit_out_of_range() {
    U1024::from_small(4201).get_bit(1024);
}

#[test]
fn test_bit_length() {
    for i in 0..8 * 3 {
        // 010000...000
        assert_eq!(U1024::from_small(1).mul_pow2(i).bit_length(), i + 1);
    }
    for i in 1..8 * 3 - 1 {
        // 010000...001
        assert_eq!(
            U1024::from_small(1)
                .mul_pow2(i)
                .plus(&U1024::from_small(1))
                .bit_length(),
            i + 1
        );
        // 110000...000
        assert_eq!(U1024::from_small(3).mul_pow2(i).bit_length(), i + 2);
    }
    assert_eq!(U1024::from_small(0).bit_length(), 0);
    assert_eq!(U1024::from_small(1).bit_length(), 1);
    assert_eq!(U1024::from_small(5).bit_length(), 3);
    assert_eq!(U1024::from_small(0x18).bit_length(), 5);
    assert_eq!(U1024::from_small(0x4073).bit_length(), 15);
    assert_eq!(U1024::from_small(0xffffff).bit_length(), 24);
}

#[test]
fn test_ord() {
    assert!(U1024::from_small(0) < U1024::from_small(0xffffff));
    assert!(U1024::from_small(0x102) < U1024::from_small(0x201));
}

#[test]
fn test_fmt() {
    assert_eq!(format!("{:?}", U1024::from_small(0)), "0x0");
    assert_eq!(
        format!("{:?}", U1024::from_u128(0x123456789)),
        "0x123456789"
    );
    assert_eq!(
        format!(
            "{:?}",
            U1024::from_u128(0x12345678_12345678_12345678_12345678)
        ),
        "0x1234567812345678_1234567812345678"
    )
}
