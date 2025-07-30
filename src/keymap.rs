use rmk::action::KeyAction;
use rmk::{a, k, layer};

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
    use rmk::keycode::KeyCode;
}

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; TOTAL_COL]; TOTAL_ROW]; NUM_LAYER] {
    [
        //BASE
        layer!([
            [k!(Backspace), k!(Delete), k!(W),     k!(E),    k!(R),    k!(T),      a!(No),        nokey!(), /**/ nokey!(),  k!(Kc0),        k!(Z),      k!(U),    k!(I),     k!(O),    a!(No),    a!(No)],
            [k!(Escape),    k!(Q),      k!(S),     k!(D),    k!(F),    k!(G),      mo_control!(), nokey!(), /**/ nokey!(),  a!(No),        k!(H),      k!(J),    k!(K),     k!(L),    k!(P),     a!(No)],
            [k!(LShift),    k!(A),      k!(X),     k!(C),    k!(V),    k!(B),      nokey!(),      nokey!(), /**/ nokey!(),  nokey!(),      k!(N),      k!(M),    k!(Comma), k!(Dot),  k!(Enter), k!(Tab)],
            [nokey!(),      k!(Y),      nokey!(),  nokey!(), k!(LGui), mo_prog!(), k!(Space),     k!(LAlt), /**/ a!(No),    mo_control!(), mo_spcl!(), a!(No),   nokey!(),  nokey!(), k!(Minus), nokey!()],
            [nokey!(),      nokey!(),   nokey!(),  nokey!(), nokey!(), nokey!(),   k!(LCtrl),     k!(RAlt), /**/ a!(No),    a!(No),        nokey!(),   nokey!(), nokey!(),  nokey!(), nokey!(),  nokey!()]
        ]),
        //CONTROL
        layer!([
            [a!(No),    a!(No),   a!(No),   a!(No),   a!(No),   a!(No),     a!(No),   nokey!(), /**/ nokey!(),  k!(Kc1),        a!(No),     a!(No),   a!(No),   a!(No),   a!(No),   a!(No)],
            [a!(No),    a!(No),   a!(No),   a!(No),   a!(No),   a!(No),     a!(No),   nokey!(), /**/ nokey!(),  a!(No),        a!(No),     a!(No),   a!(No),   a!(No),   a!(No),   a!(No)],
            [a!(No),    a!(No),   a!(No),   a!(No),   a!(No),   a!(No),     nokey!(), nokey!(), /**/ nokey!(),  nokey!(),      a!(No),     a!(No),   a!(No),   a!(No),   a!(No),   a!(No)],
            [nokey!(),  a!(No),   nokey!(), nokey!(), a!(No),   a!(No),     a!(No),   a!(No),   /**/ a!(No),    a!(No),        a!(No),     a!(No),   nokey!(), nokey!(), a!(No),   nokey!()],
            [nokey!(),  nokey!(), nokey!(), nokey!(), nokey!(), nokey!(),   a!(No),   a!(No),   /**/ a!(No),    a!(No),        nokey!(),   nokey!(), nokey!(), nokey!(), nokey!(), nokey!()]
        ]),
        //SPCL
        layer!([
            [a!(No),    a!(No),   a!(No),   a!(No),   a!(No),   a!(No),     a!(No),   nokey!(), /**/ nokey!(),  k!(Kc2),        a!(No),     a!(No),   a!(No),   a!(No),   a!(No),   a!(No)],
            [a!(No),    a!(No),   a!(No),   a!(No),   a!(No),   a!(No),     a!(No),   nokey!(), /**/ nokey!(),  a!(No),        a!(No),     a!(No),   a!(No),   a!(No),   a!(No),   a!(No)],
            [a!(No),    a!(No),   a!(No),   a!(No),   a!(No),   a!(No),     nokey!(), nokey!(), /**/ nokey!(),  nokey!(),      a!(No),     a!(No),   a!(No),   a!(No),   a!(No),   a!(No)],
            [nokey!(),  a!(No),   nokey!(), nokey!(), a!(No),   a!(No),     a!(No),   a!(No),   /**/ a!(No),    a!(No),        a!(No),     a!(No),   nokey!(), nokey!(), a!(No),   nokey!()],
            [nokey!(),  nokey!(), nokey!(), nokey!(), nokey!(), nokey!(),   a!(No),   a!(No),   /**/ a!(No),    a!(No),        nokey!(),   nokey!(), nokey!(), nokey!(), nokey!(), nokey!()]
        ]),
        //PROG
        layer!([
            [a!(No),    a!(No),   a!(No),   a!(No),   a!(No),   a!(No),     a!(No),   nokey!(), /**/ nokey!(),  k!(Kc3),        a!(No),     a!(No),   a!(No),   a!(No),   a!(No),   a!(No)],
            [a!(No),    a!(No),   a!(No),   a!(No),   a!(No),   a!(No),     a!(No),   nokey!(), /**/ nokey!(),  a!(No),        a!(No),     a!(No),   a!(No),   a!(No),   a!(No),   a!(No)],
            [a!(No),    a!(No),   a!(No),   a!(No),   a!(No),   a!(No),     nokey!(), nokey!(), /**/ nokey!(),  nokey!(),      a!(No),     a!(No),   a!(No),   a!(No),   a!(No),   a!(No)],
            [nokey!(),  a!(No),   nokey!(), nokey!(), a!(No),   a!(No),     a!(No),   a!(No),   /**/ a!(No),    a!(No),        a!(No),     a!(No),   nokey!(), nokey!(), a!(No),   nokey!()],
            [nokey!(),  nokey!(), nokey!(), nokey!(), nokey!(), nokey!(),   a!(No),   a!(No),   /**/ a!(No),    a!(No),        nokey!(),   nokey!(), nokey!(), nokey!(), nokey!(), nokey!()]
        ]),
    ]
}
