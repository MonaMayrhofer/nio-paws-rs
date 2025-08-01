use rmk::action::KeyAction;
use rmk::{a, layer};

pub(crate) const LEFT_COL: usize = 8;
pub(crate) const LEFT_ROW: usize = 5;
pub(crate) const LEFT_COL_OFFSET: usize = 0;
pub(crate) const LEFT_ROW_OFFSET: usize = 0;

pub(crate) const RIGHT_COL: usize = 8;
pub(crate) const RIGHT_ROW: usize = 5;
pub(crate) const RIGHT_COL_OFFSET: usize = LEFT_COL;
pub(crate) const RIGHT_ROW_OFFSET: usize = 0;

pub(crate) const NUM_LAYER: usize = 4;

pub(crate) const TOTAL_COL: usize = LEFT_COL + RIGHT_COL;
pub(crate) const TOTAL_ROW: usize = 5;

/// Create a normal key. For example, `k!(A)` represents `KeyAction::Single(Action::Key(KeyCode::A))`
macro_rules! k {
    ($k: expr) => {
        rmk::action::KeyAction::Single(rmk::action::Action::Key($k))
    };
}

macro_rules! wm {
    ($x: expr, $m: expr) => {
        rmk::action::KeyAction::Single(rmk::action::Action::KeyWithModifier($x, $m))
    };
}

macro_rules! shifted {
    ($x: expr) => {
        wm!(
            $x,
            rmk::keycode::ModifierCombination::new_from(false, false, false, true, false)
        )
    };
}

macro_rules! algr {
    ($x: expr) => {
        wm!(
            $x,
            rmk::keycode::ModifierCombination::new_from(true, false, true, false, false)
        )
    };
}

macro_rules! nokey {
    () => {
        rmk::action::KeyAction::Single(rmk::action::Action::Key(
            rmk::keycode::KeyCode::ErrorUndefined,
        ))
    };
}

macro_rules! mo_prog {
    () => {
        rmk::mo!(3)
    };
}

macro_rules! mo_control {
    () => {
        rmk::mo!(1)
    };
}

macro_rules! mo_spcl {
    () => {
        rmk::mo!(2)
    };
}

mod german {
    use rmk::{action::KeyAction, keycode::KeyCode};

    pub const Circumflex: KeyCode = KeyCode::Grave;
    pub const Kc1: KeyCode = KeyCode::Kc1;
    pub const Kc2: KeyCode = KeyCode::Kc2;
    pub const Kc3: KeyCode = KeyCode::Kc3;
    pub const Kc4: KeyCode = KeyCode::Kc4;
    pub const Kc5: KeyCode = KeyCode::Kc5;
    pub const Kc6: KeyCode = KeyCode::Kc6;
    pub const Kc7: KeyCode = KeyCode::Kc7;
    pub const Kc8: KeyCode = KeyCode::Kc8;
    pub const Kc9: KeyCode = KeyCode::Kc9;
    pub const Kc0: KeyCode = KeyCode::Kc0;
    pub const SharpS: KeyCode = KeyCode::Minus;
    pub const Acute: KeyCode = KeyCode::Equal;
    // #define DE_Q    KC_Q    // Q
    // #define DE_W    KC_W    // W
    // #define DE_E    KC_E    // E
    // #define DE_R    KC_R    // R
    // #define DE_T    KC_T    // T
    // #define DE_Z    KC_Y    // Z
    // #define DE_U    KC_U    // U
    // #define DE_I    KC_I    // I
    // #define DE_O    KC_O    // O
    // #define DE_P    KC_P    // P
    pub const Udia: KeyCode = KeyCode::LeftBracket;
    pub const Plus: KeyCode = KeyCode::RightBracket;
    // #define DE_A    KC_A    // A
    // #define DE_S    KC_S    // S
    // #define DE_D    KC_D    // D
    // #define DE_F    KC_F    // F
    // #define DE_G    KC_G    // G
    // #define DE_H    KC_H    // H
    // #define DE_J    KC_J    // J
    // #define DE_K    KC_K    // K
    // #define DE_L    KC_L    // L
    pub const Odia: KeyCode = KeyCode::Semicolon;
    pub const Adia: KeyCode = KeyCode::Quote;
    pub const Hash: KeyCode = KeyCode::NonusHash;

    pub const LeftAngleBracket: KeyCode = KeyCode::NonusBackslash;
    pub const Y: KeyCode = KeyCode::Z;
    pub const Z: KeyCode = KeyCode::Y;
    // #define DE_X    KC_X    // X
    // #define DE_C    KC_C    // C
    // #define DE_V    KC_V    // V
    // #define DE_B    KC_B    // B
    // #define DE_N    KC_N    // N
    // #define DE_M    KC_M    // M
    pub const Comma: KeyCode = KeyCode::Comma;
    pub const Dot: KeyCode = KeyCode::Dot;
    pub const Minus: KeyCode = KeyCode::Slash;
    // #define DE_DEG  S(DE_CIRC) // °
    const Degree: KeyAction = shifted!(Circumflex);
    // #define DE_EXLM S(DE_1)    // !
    pub const Exclamation: KeyAction = shifted!(Kc1);
    // #define DE_DQUO S(DE_2)    // "
    pub const DoubleQuote: KeyAction = shifted!(Kc2);
    // #define DE_SECT S(DE_3)    // §
    const Section: KeyAction = shifted!(Kc3);
    // #define DE_DLR  S(DE_4)    // $
    pub const Dollar: KeyAction = shifted!(Kc4);
    // #define DE_PERC S(DE_5)    // %
    const Percent: KeyAction = shifted!(Kc5);
    // #define DE_AMPR S(DE_6)    // &
    pub const Ampersand: KeyAction = shifted!(Kc6);
    // #define DE_SLSH S(DE_7)    // /
    pub const Slash: KeyAction = shifted!(Kc7);
    // #define DE_LPRN S(DE_8)    // (
    pub const LeftParenthesis: KeyAction = shifted!(Kc8);
    // #define DE_RPRN S(DE_9)    // )
    pub const RightParenthesis: KeyAction = shifted!(Kc9);
    // #define DE_EQL  S(DE_0)    // =
    pub const Equal: KeyAction = shifted!(Kc0);
    // #define DE_QUES S(DE_SS)   // ?
    const QuestionMark: KeyAction = shifted!(SharpS);

    // #define DE_GRV  S(DE_ACUT) // ` (dead)
    const GraveAccent: KeyAction = shifted!(Acute);
    // #define DE_ASTR S(DE_PLUS) // *
    const Asterisk: KeyAction = shifted!(Plus);
    // #define DE_QUOT S(DE_HASH) // '
    const SingleQuote: KeyAction = shifted!(Hash);
    // #define DE_RABK S(DE_LABK) // >
    pub const RightAngleBracket: KeyAction = shifted!(LeftAngleBracket);
    // #define DE_SCLN S(DE_COMM) // ;
    pub const Semicolon: KeyAction = shifted!(Comma);
    // #define DE_COLN S(DE_DOT)  // :
    pub const Colon: KeyAction = shifted!(Dot);
    // #define DE_UNDS S(DE_MINS) // _
    pub const Underscore: KeyAction = shifted!(Minus);
    // #define DE_SUP2 ALGR(DE_2)    // ²
    // #define DE_SUP3 ALGR(DE_3)    // ³
    // #define DE_LCBR ALGR(DE_7)    // {
    pub const LeftCurlyBracket: KeyAction = algr!(Kc7);
    // #define DE_LBRC ALGR(DE_8)    // [
    pub const LeftBracket: KeyAction = algr!(Kc8);
    // #define DE_RBRC ALGR(DE_9)    // ]
    pub const RightBracket: KeyAction = algr!(Kc9);
    // #define DE_RCBR ALGR(DE_0)    // }
    pub const RightCurlyBracket: KeyAction = algr!(Kc0);
    // #define DE_BSLS ALGR(DE_SS)   // (backslash)
    pub const Backslash: KeyAction = algr!(SharpS);
    // #define DE_AT   ALGR(DE_Q)    // @
    // #define DE_EURO ALGR(DE_E)    // €
    // #define DE_TILD ALGR(DE_PLUS) // ~
    pub const Tilde: KeyAction = algr!(Plus);
    // #define DE_PIPE ALGR(DE_LABK) // |
    pub const Pipe: KeyAction = algr!(LeftAngleBracket);
    // #define DE_MICR ALGR(DE_M)    // µ
    const Micro: KeyAction = algr!(KeyCode::M);
}

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; TOTAL_COL]; TOTAL_ROW]; NUM_LAYER] {
    use rmk::keycode::KeyCode::*;
    use german as g;

    [
        //BASE
        layer!([
            [k!(Backspace), k!(Delete), k!(W),     k!(E),    k!(R),    k!(T),      a!(No),        nokey!(), /**/ nokey!(),  k!(Kc0),       k!(g::Z),   k!(U),    k!(I),        k!(O),       a!(No),       a!(No)],
            [k!(Escape),    k!(Q),      k!(S),     k!(D),    k!(F),    k!(G),      mo_control!(), nokey!(), /**/ nokey!(),  a!(No),        k!(H),      k!(J),    k!(K),        k!(L),       k!(P),        a!(No)],
            [k!(LShift),    k!(A),      k!(X),     k!(C),    k!(V),    k!(B),      nokey!(),      nokey!(), /**/ nokey!(),  nokey!(),      k!(N),      k!(M),    k!(g::Comma), k!(g::Dot),  k!(Enter),    k!(Tab)],
            [nokey!(),      k!(g::Y),   nokey!(),  nokey!(), k!(LGui), mo_prog!(), k!(Space),     k!(LAlt), /**/ a!(No),    mo_control!(), mo_spcl!(), a!(No),   nokey!(),     nokey!(),    k!(g::Minus), nokey!()],
            [nokey!(),      nokey!(),   nokey!(),  nokey!(), nokey!(), nokey!(),   k!(LCtrl),     k!(RAlt), /**/ a!(No),    a!(No),        nokey!(),   nokey!(), nokey!(),     nokey!(),    nokey!(),     nokey!()]
        ]),
        //CONTROL
        layer!([
            [a!(Transparent), a!(Transparent), k!(F2),           k!(F3),          k!(F4),          k!(F5),          a!(Transparent),   nokey!(),        /**/ nokey!(),        k!(Kc1),         a!(Transparent), a!(Transparent),   a!(Transparent),   a!(Transparent),   a!(Transparent),   a!(Transparent)],
            [a!(Transparent), k!(F1),          k!(PrintScreen),  a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent),   nokey!(),        /**/ nokey!(),        a!(Transparent), a!(Transparent), a!(Transparent),   a!(Transparent),   a!(Transparent),   a!(Transparent),   a!(Transparent)],
            [a!(Transparent), a!(Transparent), a!(Transparent),  a!(Transparent), a!(Transparent), a!(Transparent), nokey!(),          nokey!(),        /**/ nokey!(),        nokey!(),        a!(Transparent), a!(Transparent),   a!(Transparent),   a!(Transparent),   a!(Transparent),   a!(Transparent)],
            [nokey!(),        a!(Transparent), nokey!(),         nokey!(),        a!(Transparent), k!(Insert),      a!(Transparent),   a!(Transparent), /**/ a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent),   nokey!(),          nokey!(),          a!(Transparent),   nokey!()],
            [nokey!(),        nokey!(),        nokey!(),         nokey!(),        nokey!(),        nokey!(),        a!(Transparent),   nokey!(),        /**/ a!(Transparent), a!(Transparent), nokey!(),        nokey!(),          nokey!(),          nokey!(),          nokey!(),          nokey!()]
        ]),
        //SPCL
        layer!([
            [a!(Transparent),   a!(Transparent),   k!(g::Kc2),      k!(g::Kc3),      k!(g::Kc4),      k!(g::Kc5),      a!(Transparent), nokey!(),          /**/ nokey!(),        a!(Transparent), k!(g::Kc6),      k!(g::Kc7),      k!(g::Kc8),      k!(g::Kc9),      a!(Transparent), a!(Transparent)],
            [a!(Transparent),   k!(g::Kc1),        k!(Backspace),   k!(g::Udia),     k!(g::Odia),     k!(Delete),      a!(Transparent), nokey!(),          /**/ nokey!(),        a!(Transparent), k!(Left),        k!(Down),        k!(Up),          k!(Right),       k!(g::Kc0),      k!(g::Acute)],
            [a!(Transparent),   k!(g::Adia),       a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), nokey!(),        nokey!(),          /**/ nokey!(),        nokey!(),        a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
            [nokey!(),          k!(g::Circumflex), nokey!(),        nokey!(),        a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent),   /**/ a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), nokey!(),        nokey!(),        a!(Transparent), nokey!()],
            [nokey!(),          nokey!(),          nokey!(),        nokey!(),        nokey!(),        nokey!(),        a!(Transparent), a!(Transparent),   /**/ a!(Transparent), a!(Transparent), nokey!(),        nokey!(),        nokey!(),        nokey!(),        nokey!(),        nokey!()]
        ]),
        //PROG
        layer!([
            [a!(Transparent),   a!(Transparent), g::DoubleQuote,          a!(Transparent),      g::Dollar,       g::Tilde,        a!(Transparent),   nokey!(),        /**/ nokey!(),        a!(Transparent), g::Ampersand,    g::LeftCurlyBracket,  g::LeftBracket,     g::RightBracket,     a!(Transparent),      a!(Transparent)],
            [k!(g::Circumflex), g::Exclamation,  k!(g::LeftAngleBracket), g::RightAngleBracket, k!(g::Plus),     k!(g::Hash),     a!(Transparent),   nokey!(),        /**/ nokey!(),        a!(Transparent), g::Slash,        a!(Transparent),      g::LeftParenthesis, g::RightParenthesis, g::RightCurlyBracket, a!(Transparent)],
            [a!(Transparent),   a!(Transparent), a!(Transparent),         a!(Transparent),      a!(Transparent), a!(Transparent), nokey!(),          nokey!(),        /**/ nokey!(),        nokey!(),        a!(Transparent), a!(Transparent),      a!(Transparent),    a!(Transparent),     g::Equal,             a!(Transparent)],
            [nokey!(),          a!(Transparent), nokey!(),                nokey!(),             a!(Transparent), a!(Transparent), a!(Transparent),   a!(Transparent), /**/ a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent),      nokey!(),           nokey!(),            a!(Transparent),      nokey!()],
            [nokey!(),          nokey!(),        nokey!(),                nokey!(),             nokey!(),        nokey!(),        a!(Transparent),   a!(Transparent), /**/ a!(Transparent), a!(Transparent), nokey!(),        nokey!(),             nokey!(),           nokey!(),            nokey!(),             nokey!()]
        ]),
    ]
}
