use core::iter::Iterator;

#[derive(Clone, Copy)]
pub enum Morse {
    Dot,
    Dash,
    Space,
}

macro_rules! morse_seq {
    ($($morse:ident)*) => {
        [$(morse_seq!(@sym $morse)),*]
    };

    (@sym o) => {Morse::Dot};

    (@sym l) => {Morse::Dash};
}

pub fn byte_to_morse(c: u8) -> impl Iterator<Item = Morse> {
    let sequence = match c {
        b'A' | b'a' => morse_seq![o l].iter(),
        b'B' | b'b' => morse_seq![l o o o].iter(),
        b'C' | b'c' => morse_seq![l o l o].iter(),
        b'D' | b'd' => morse_seq![l o o].iter(),
        b'E' | b'e' => morse_seq![o].iter(),
        b'F' | b'f' => morse_seq![o o l o].iter(),
        b'G' | b'g' => morse_seq![l l o].iter(),
        b'H' | b'h' => morse_seq![o o o o].iter(),
        b'I' | b'i' => morse_seq![o o].iter(),
        b'J' | b'j' => morse_seq![o l l l].iter(),
        b'K' | b'k' => morse_seq![l o l].iter(),
        b'L' | b'l' => morse_seq![o l o o].iter(),
        b'M' | b'm' => morse_seq![l l].iter(),
        b'N' | b'n' => morse_seq![l o].iter(),
        b'O' | b'o' => morse_seq![l l l].iter(),
        b'P' | b'p' => morse_seq![o l l o].iter(),
        b'Q' | b'q' => morse_seq![l l o l].iter(),
        b'R' | b'r' => morse_seq![o l o].iter(),
        b'S' | b's' => morse_seq![o o o].iter(),
        b'T' | b't' => morse_seq![l].iter(),
        b'U' | b'u' => morse_seq![o o l].iter(),
        b'V' | b'v' => morse_seq![o o o l].iter(),
        b'W' | b'w' => morse_seq![o l l].iter(),
        b'X' | b'x' => morse_seq![l o o l].iter(),
        b'Y' | b'y' => morse_seq![l o l l].iter(),
        b'Z' | b'z' => morse_seq![l l o o].iter(),

        b'1' => morse_seq![o l l l l].iter(),
        b'2' => morse_seq![o o l l l].iter(),
        b'3' => morse_seq![o o o l l].iter(),
        b'4' => morse_seq![o o o o l].iter(),
        b'5' => morse_seq![o o o o o].iter(),
        b'6' => morse_seq![l o o o o].iter(),
        b'7' => morse_seq![l l o o o].iter(),
        b'8' => morse_seq![l l l o o].iter(),
        b'9' => morse_seq![l l l l o].iter(),
        b'0' => morse_seq![l l l l l].iter(),

        b' ' | b'\n' | b'\r' | b'\t' => [Morse::Space].iter(),

        _ => [].iter(),
    };

    sequence.copied()
}
