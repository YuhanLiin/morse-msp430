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

#[derive(Clone, Copy)]
pub enum FsmState {
    Start,
    E,
    T,

    N,
    I,
    A,
    M,

    D,
    G,
    K,
    O,
    R,
    S,
    U,
    W,

    B,
    C,
    F,
    H,
    J,
    L,
    P,
    Q,
    V,
    X,
    Y,
    Z,

    // Intermediate states
    LLLO,
    LLLL,
    OOLL,
}

impl FsmState {
    pub fn next(self, morse: Morse) -> (Self, u8) {
        use FsmState::*;

        match morse {
            Morse::Dot => match self {
                Start => (E, 0),
                E => (I, 0),
                T => (N, 0),
                N => (D, 0),
                I => (S, 0),
                A => (R, 0),
                M => (G, 0),
                D => (B, 0),
                G => (Z, 0),
                K => (C, 0),
                O => (LLLO, 0),
                R => (L, 0),
                S => (H, 0),
                U => (F, 0),
                W => (P, 0),
                B => (Start, b'6'),
                C => (E, b'C'),
                F => (E, b'F'),
                H => (Start, b'5'),
                J => (E, b'J'),
                L => (E, b'L'),
                P => (E, b'P'),
                Q => (E, b'Q'),
                V => (E, b'V'),
                X => (E, b'X'),
                Y => (E, b'Y'),
                Z => (Start, b'7'),
                LLLO => (Start, b'8'),
                LLLL => (Start, b'9'),
                OOLL => (N, b'U'),
            },

            Morse::Dash => match self {
                Start => (T, 0),
                E => (A, 0),
                T => (M, 0),
                N => (K, 0),
                I => (U, 0),
                A => (W, 0),
                M => (O, 0),
                D => (X, 0),
                G => (Q, 0),
                K => (Y, 0),
                O => (LLLL, 0),
                R => (T, b'R'),
                S => (V, 0),
                U => (OOLL, 0),
                W => (J, 0),
                B => (T, b'B'),
                C => (T, b'C'),
                F => (T, b'F'),
                H => (Start, b'4'),
                J => (Start, b'1'),
                L => (T, b'L'),
                P => (T, b'P'),
                Q => (T, b'Q'),
                V => (Start, b'3'),
                X => (T, b'X'),
                Y => (T, b'Y'),
                Z => (T, b'Z'),
                LLLO => (A, b'O'),
                LLLL => (Start, b'0'),
                OOLL => (Start, b'2'),
            },

            Morse::Space => match self {
                Start => (Start, 0),
                E => (Start, b'E'),
                T => (Start, b'T'),
                N => (Start, b'N'),
                I => (Start, b'I'),
                A => (Start, b'A'),
                M => (Start, b'M'),
                D => (Start, b'D'),
                G => (Start, b'G'),
                K => (Start, b'K'),
                O => (Start, b'O'),
                R => (Start, b'R'),
                S => (Start, b'S'),
                U => (Start, b'U'),
                W => (Start, b'W'),
                B => (Start, b'B'),
                C => (Start, b'C'),
                F => (Start, b'F'),
                H => (Start, b'H'),
                J => (Start, b'J'),
                L => (Start, b'L'),
                P => (Start, b'P'),
                Q => (Start, b'Q'),
                V => (Start, b'V'),
                X => (Start, b'X'),
                Y => (Start, b'Y'),
                Z => (Start, b'Z'),
                LLLO => (E, b'O'),
                LLLL => (T, b'O'),
                OOLL => (T, b'U'),
            },
        }
    }
}
