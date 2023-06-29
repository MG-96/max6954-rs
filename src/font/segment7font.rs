use super::Segments;
use enumflags2::make_bitflags;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Segment7 {
    Zero = make_bitflags!(Segments::{A|B|C|D|E|F}).bits_c(),
    One = make_bitflags!(Segments::{B|C}).bits_c(),
    Two = make_bitflags!(Segments::{A|B|G|E|D}).bits_c(),
    Three = make_bitflags!(Segments::{A|B|G|C|D}).bits_c(),
    Four = make_bitflags!(Segments::{F|B|G|C}).bits_c(),
    Five = make_bitflags!(Segments::{A|F|G|C|D}).bits_c(),
    Six = make_bitflags!(Segments::{A|F|E|D|C|G}).bits_c(),
    Seven = make_bitflags!(Segments::{A|B|C}).bits_c(),
    Eight = make_bitflags!(Segments::{A|B|C|D|E|F|G}).bits_c(),
    Nine = make_bitflags!(Segments::{A|B|C|D|F|G}).bits_c(),
    A = make_bitflags!(Segments::{A|B|C|E|F|G}).bits_c(),
    B = make_bitflags!(Segments::{C|D|E|F|G}).bits_c(),
    C = make_bitflags!(Segments::{A|D|E|F}).bits_c(),
    D = make_bitflags!(Segments::{B|C|D|E|G}).bits_c(),
    E = make_bitflags!(Segments::{A|D|E|F|G}).bits_c(),
    F = make_bitflags!(Segments::{A|E|F|G}).bits_c(),
    G = make_bitflags!(Segments::{A|C|D|E|F}).bits_c(),
    H = make_bitflags!(Segments::{C|E|F|G}).bits_c(),
    I = make_bitflags!(Segments::{E|F}).bits_c(),
    J = make_bitflags!(Segments::{B|C|D|E}).bits_c(),
    K = make_bitflags!(Segments::{A|C|E|F|G}).bits_c(),
    L = make_bitflags!(Segments::{D|E|F}).bits_c(),
    M = make_bitflags!(Segments::{B|C|E|G}).bits_c(),
    N = make_bitflags!(Segments::{C|E|G}).bits_c(),
    O = make_bitflags!(Segments::{C|D|E|G}).bits_c(),
    P = make_bitflags!(Segments::{A|B|E|F|G}).bits_c(),
    Q = make_bitflags!(Segments::{A|B|C|F|G}).bits_c(),
    R = make_bitflags!(Segments::{E|G}).bits_c(),
    S = make_bitflags!(Segments::{C|F|G}).bits_c(),
    T = make_bitflags!(Segments::{D|E|F|G}).bits_c(),
    U = make_bitflags!(Segments::{B|C|D|E|F}).bits_c(),
    V = make_bitflags!(Segments::{C|D|E}).bits_c(),
    W = make_bitflags!(Segments::{B|C|D|E|F|G}).bits_c(),
    X = make_bitflags!(Segments::{B|C|E|F|G}).bits_c(),
    Y = make_bitflags!(Segments::{B|C|D|F|G}).bits_c(),
    Z = make_bitflags!(Segments::{A|D|G}).bits_c(),
    QuestionMark = make_bitflags!(Segments::{A|B|E|G}).bits_c(),
    Minus = make_bitflags!(Segments::{G}).bits_c(),
    Underscore = make_bitflags!(Segments::{D}).bits_c(),
    Equal = make_bitflags!(Segments::{A|G}).bits_c(),
    Apostrophe = make_bitflags!(Segments::{F}).bits_c(),
    QuotationMark = make_bitflags!(Segments::{B|F}).bits_c(),
    Degree = make_bitflags!(Segments::{A|B|F|G}).bits_c(),
    Blank = 0x00,
}

use Segment7::*;
impl From<char> for Segment7 {
    fn from(value: char) -> Self {
        let value = value.to_ascii_uppercase();
        match value {
            '0' => Zero,
            '1' => One,
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'A' => A,
            'B' => B,
            'C' => C,
            'D' => D,
            'E' => E ,
            'F' => F ,
            'G' => G ,
            'H' => H ,
            'I' => I ,
            'J' => J ,
            'K' => K ,
            'L' => L ,
            'M' => M ,
            'N' => N ,
            'O' => O ,
            'P' => P ,
            'Q' => Q ,
            'R' => R ,
            'S' => S ,
            'T' => T ,
            'U' => U ,
            'V' => V ,
            'W' => W ,
            'X' => X ,
            'Y' => Y ,
            'Z' => Z ,
            '?' => QuestionMark ,
            '-' => Minus ,
            '_' => Underscore ,
            '=' => Equal ,
            '\'' => Apostrophe ,
            '"' => QuotationMark ,
            ' ' => Blank ,
            _ => Blank,
        }
    }
}