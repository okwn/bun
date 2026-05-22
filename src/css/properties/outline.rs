use super::border::{GenericBorder, LineStyle};

/// A value for the [outline](https://drafts.csswg.org/css-ui/#outline) shorthand property.
pub type Outline = GenericBorder<OutlineStyle, 11>;

#[derive(Clone, PartialEq, Eq, crate::Parse, crate::ToCss)]
pub enum OutlineStyle {
    /// The `auto` keyword.
    Auto,
    /// A value equivalent to the `border-style` property.
    LineStyle(LineStyle),
}

impl Default for OutlineStyle {
    fn default() -> Self {
        OutlineStyle::LineStyle(LineStyle::None)
    }
}

impl OutlineStyle {
    pub fn eql(lhs: &Self, rhs: &Self) -> bool {
        lhs == rhs
    }

    pub fn deep_clone(&self, _bump: &bun_alloc::Arena) -> Self {
        // PERF(port): was arena-aware implementDeepClone — variants are POD so Clone suffices
        self.clone()
    }
}

// ported from: src/css/properties/outline.zig
