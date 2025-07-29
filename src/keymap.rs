use rmk::action::KeyAction;
use rmk::{a, k, layer, mo};

pub(crate) const LEFT_COL: usize = 8;
pub(crate) const LEFT_ROW: usize = 5;
pub(crate) const LEFT_COL_OFFSET: usize = 0;
pub(crate) const LEFT_ROW_OFFSET: usize = 0;

pub(crate) const RIGHT_COL: usize = 8;
pub(crate) const RIGHT_ROW: usize = 5;
pub(crate) const RIGHT_COL_OFFSET: usize = LEFT_COL;
pub(crate) const RIGHT_ROW_OFFSET: usize = 0;

pub(crate) const NUM_LAYER: usize = 1;

pub(crate) const TOTAL_COL: usize = LEFT_COL + RIGHT_COL;
pub(crate) const TOTAL_ROW: usize = 5;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; TOTAL_COL]; TOTAL_ROW]; NUM_LAYER] {
    [
        layer!([
            [k!(Kp0), k!(Kp1), k!(Kp2), k!(Kp3), k!(Kp4), k!(Kp5), k!(Kp6), k!(No) , /**/ k!(No),  k!(Kp1), k!(Kp2), k!(Kp3), k!(Kp4), k!(Kp5), k!(Kp6), k!(Kp7)],
            [k!(Kp0), k!(Kp1), k!(Kp2), k!(Kp3), k!(Kp4), k!(Kp5), k!(Kp6), k!(No) , /**/ k!(No),  k!(Kp1), k!(Kp2), k!(Kp3), k!(Kp4), k!(Kp5), k!(Kp6), k!(Kp7)],
            [k!(Kp0), k!(Kp1), k!(Kp2), k!(Kp3), k!(Kp4), k!(Kp5), k!(No),  k!(No) , /**/ k!(No),  k!(No),  k!(Kp1), k!(Kp2), k!(Kp3), k!(Kp4), k!(Kp5), k!(Kp6)],
            [k!(Kp0), k!(Kp1), k!(No),  k!(No),  k!(Kp4), k!(Kp5), k!(Kp5), k!(Kp6), /**/ k!(Kp1), k!(Kp2), k!(Kp3), k!(Kp4), k!(No),  k!(No),  k!(Kp5), k!(Kp6)],
            [k!(No),  k!(No),  k!(No),  k!(No),  k!(No),  k!(No),  k!(Kp5), k!(Kp6), /**/ k!(Kp1), k!(Kp2), k!(No),  k!(No),  k!(No),  k!(No),  k!(No),  k!(No)]
        ]),
    ]
}
