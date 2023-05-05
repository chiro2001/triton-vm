// use std::fmt::Display;
// use strum_macros::EnumCount as EnumCountMacro;
// use Ord16::*;
// use Ord8::*;
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, EnumCountMacro)]
// pub enum Ord8 {
//     #[default]
//     IB0,
//     IB1,
//     IB2,
//     IB3,
//     IB4,
//     IB5,
//     IB6,
//     IB7,
// }
//
// impl Display for Ord8 {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let n: usize = (*self).into();
//         write!(f, "{n}")
//     }
// }
//
// impl From<Ord8> for usize {
//     fn from(n: Ord8) -> Self {
//         match n {
//             IB0 => 0,
//             IB1 => 1,
//             IB2 => 2,
//             IB3 => 3,
//             IB4 => 4,
//             IB5 => 5,
//             IB6 => 6,
//             IB7 => 7,
//         }
//     }
// }
//
// impl TryFrom<usize> for Ord8 {
//     type Error = String;
//
//     fn try_from(value: usize) -> Result<Self, Self::Error> {
//         match value {
//             0 => Ok(IB0),
//             1 => Ok(IB1),
//             2 => Ok(IB2),
//             3 => Ok(IB3),
//             4 => Ok(IB4),
//             5 => Ok(IB5),
//             6 => Ok(IB6),
//             7 => Ok(IB7),
//             _ => Err(format!("{value} is out of range for Ord8")),
//         }
//     }
// }
//
// /// `Ord16` represents numbers that are exactly 0--15.
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
// pub enum Ord16 {
//     #[default]
//     ST0,
//     ST1,
//     ST2,
//     ST3,
//     ST4,
//     ST5,
//     ST6,
//     ST7,
//     ST8,
//     ST9,
//     ST10,
//     ST11,
//     ST12,
//     ST13,
//     ST14,
//     ST15,
// }
//
// impl Display for Ord16 {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let n: usize = (*self).into();
//         write!(f, "{n}")
//     }
// }
//
// impl From<Ord16> for u32 {
//     fn from(n: Ord16) -> Self {
//         match n {
//             ST0 => 0,
//             ST1 => 1,
//             ST2 => 2,
//             ST3 => 3,
//             ST4 => 4,
//             ST5 => 5,
//             ST6 => 6,
//             ST7 => 7,
//             ST8 => 8,
//             ST9 => 9,
//             ST10 => 10,
//             ST11 => 11,
//             ST12 => 12,
//             ST13 => 13,
//             ST14 => 14,
//             ST15 => 15,
//         }
//     }
// }
//
// impl From<Ord16> for u64 {
//     fn from(n: Ord16) -> Self {
//         let v: u32 = n.into();
//         v.into()
//     }
// }
//
// impl TryFrom<u32> for Ord16 {
//     type Error = String;
//
//     fn try_from(n: u32) -> Result<Self, Self::Error> {
//         match n {
//             0 => Ok(ST0),
//             1 => Ok(ST1),
//             2 => Ok(ST2),
//             3 => Ok(ST3),
//             4 => Ok(ST4),
//             5 => Ok(ST5),
//             6 => Ok(ST6),
//             7 => Ok(ST7),
//             8 => Ok(ST8),
//             9 => Ok(ST9),
//             10 => Ok(ST10),
//             11 => Ok(ST11),
//             12 => Ok(ST12),
//             13 => Ok(ST13),
//             14 => Ok(ST14),
//             15 => Ok(ST15),
//             _ => Err(format!("{n} is out of range for Ord16")),
//         }
//     }
// }
//
// impl TryFrom<u64> for Ord16 {
//     type Error = String;
//
//     fn try_from(n: u64) -> Result<Self, Self::Error> {
//         let n: u32 = n.try_into().unwrap();
//         n.try_into()
//     }
// }
//
// impl From<&Ord16> for u32 {
//     fn from(n: &Ord16) -> Self {
//         (*n).into()
//     }
// }
//
// impl From<Ord16> for usize {
//     fn from(n: Ord16) -> Self {
//         let n: u32 = n.into();
//         n as usize
//     }
// }
//
// impl From<&Ord16> for usize {
//     fn from(n: &Ord16) -> Self {
//         (*n).into()
//     }
// }
//
// impl TryFrom<usize> for Ord16 {
//     type Error = &'static str;
//
//     fn try_from(value: usize) -> Result<Self, Self::Error> {
//         match value {
//             0 => Ok(ST0),
//             1 => Ok(ST1),
//             2 => Ok(ST2),
//             3 => Ok(ST3),
//             4 => Ok(ST4),
//             5 => Ok(ST5),
//             6 => Ok(ST6),
//             7 => Ok(ST7),
//             8 => Ok(ST8),
//             9 => Ok(ST9),
//             10 => Ok(ST10),
//             11 => Ok(ST11),
//             12 => Ok(ST12),
//             13 => Ok(ST13),
//             14 => Ok(ST14),
//             15 => Ok(ST15),
//             _ => Err("usize out of range for Ord16"),
//         }
//     }
// }

use std::fmt::{Display, Formatter};
use twenty_first::shared_math::b_field_element::BFieldElement;
// use std::num::ParseIntError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Regs {
    #[default]
    Zero,
    At,
    V0,
    V1,
    A0,
    A1,
    A2,
    A3,
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    T8,
    T9,
    K0,
    K1,
    Gp,
    Sp,
    Fp,
    Ra,
}
pub const REGS: [Regs; 32] = [
    Regs::Zero,
    Regs::At,
    Regs::V0,
    Regs::V1,
    Regs::A0,
    Regs::A1,
    Regs::A2,
    Regs::A3,
    Regs::T0,
    Regs::T1,
    Regs::T2,
    Regs::T3,
    Regs::T4,
    Regs::T5,
    Regs::T6,
    Regs::T7,
    Regs::S0,
    Regs::S1,
    Regs::S2,
    Regs::S3,
    Regs::S4,
    Regs::S5,
    Regs::S6,
    Regs::S7,
    Regs::T8,
    Regs::T9,
    Regs::K0,
    Regs::K1,
    Regs::Gp,
    Regs::Sp,
    Regs::Fp,
    Regs::Ra,
];

impl From<&str> for Regs {
    fn from(value: &str) -> Self {
        if value.len() < 2 {
            panic!("Invalid register name: {}", value);
        }
        let value = &value[1..];
        match value.parse::<usize>() {
            Ok(n) if n < 32 => return REGS[n],
            _ => {}
        }
        match value {
            "zero" => Regs::Zero,
            "at" => Regs::At,
            "v0" => Regs::V0,
            "v1" => Regs::V1,
            "a0" => Regs::A0,
            "a1" => Regs::A1,
            "a2" => Regs::A2,
            "a3" => Regs::A3,
            "t0" => Regs::T0,
            "t1" => Regs::T1,
            "t2" => Regs::T2,
            "t3" => Regs::T3,
            "t4" => Regs::T4,
            "t5" => Regs::T5,
            "t6" => Regs::T6,
            "t7" => Regs::T7,
            "s0" => Regs::S0,
            "s1" => Regs::S1,
            "s2" => Regs::S2,
            "s3" => Regs::S3,
            "s4" => Regs::S4,
            "s5" => Regs::S5,
            "s6" => Regs::S6,
            "s7" => Regs::S7,
            "t8" => Regs::T8,
            "t9" => Regs::T9,
            "k0" => Regs::K0,
            "k1" => Regs::K1,
            "gp" => Regs::Gp,
            "sp" => Regs::Sp,
            "fp" => Regs::Fp,
            "ra" => Regs::Ra,
            _ => panic!("Unknown register: {}", value),
        }
    }
}
impl From<&String> for Regs {
    fn from(value: &String) -> Self {
        value.as_str().into()
    }
}
impl From<&Regs> for String {
    fn from(value: &Regs) -> Self {
        format!("{:?}", value)
    }
}
impl Display for Regs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s: String = self.into();
        write!(f, "{}", s)
    }
}
impl From<&Regs> for u32 {
    fn from(value: &Regs) -> Self {
        match value {
            Regs::Zero => 0,
            Regs::At => 1,
            Regs::V0 => 2,
            Regs::V1 => 3,
            Regs::A0 => 4,
            Regs::A1 => 5,
            Regs::A2 => 6,
            Regs::A3 => 7,
            Regs::T0 => 8,
            Regs::T1 => 9,
            Regs::T2 => 10,
            Regs::T3 => 11,
            Regs::T4 => 12,
            Regs::T5 => 13,
            Regs::T6 => 14,
            Regs::T7 => 15,
            Regs::S0 => 16,
            Regs::S1 => 17,
            Regs::S2 => 18,
            Regs::S3 => 19,
            Regs::S4 => 20,
            Regs::S5 => 21,
            Regs::S6 => 22,
            Regs::S7 => 23,
            Regs::T8 => 24,
            Regs::T9 => 25,
            Regs::K0 => 26,
            Regs::K1 => 27,
            Regs::Gp => 28,
            Regs::Sp => 29,
            Regs::Fp => 30,
            Regs::Ra => 31,
        }
    }
}
impl From<&Regs> for BFieldElement {
    fn from(value: &Regs) -> Self {
        let n: u32 = value.into();
        n.into()
    }
}
