use crate::values::number::CSSNumberFns;
use crate::values::percentage::NumberOrPercentage;
use crate::{Parser, PrintErr, Printer, Result};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AlphaValue {
    pub v: f32,
}

impl AlphaValue {
    pub fn parse(input: &mut Parser) -> Result<AlphaValue> {
        // For some reason NumberOrPercentage.parse makes zls crash, using this instead.
        // PORT NOTE: the Zig used `@call(.auto, @field(...))` as a zls workaround; direct call in Rust.
        let val: NumberOrPercentage = NumberOrPercentage::parse(input)?;
        let final_ = match val {
            NumberOrPercentage::Percentage(percent) => AlphaValue { v: percent.v },
            NumberOrPercentage::Number(num) => AlphaValue { v: num },
        };
        Result::Ok(final_)
    }

    pub fn to_css(self, dest: &mut Printer) -> core::result::Result<(), PrintErr> {
        CSSNumberFns::to_css(self.v, dest)
    }

    pub fn eql(lhs: Self, rhs: Self) -> bool {
        // PORT NOTE: Zig used css.implementEql (comptime field reflection); single f32 field → direct compare.
        lhs.v == rhs.v
    }

    // TODO(port): css.implementHash (comptime field reflection) — wires once
    // generics::CssHash blanket impl covers f32-payload structs.

    pub fn deep_clone(self) -> Self {
        // PORT NOTE: Zig used css.implementDeepClone; struct is Copy so this is a trivial copy.
        self
    }
}

// ported from: src/css/values/alpha.zig
