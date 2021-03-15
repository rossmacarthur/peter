use std::fmt::{Display, Formatter, Result};

use termion::style::*;

pub trait Style: Display {
    type Reset: Display;

    fn reset(&self) -> Self::Reset;

    fn write_styled<D: Display>(&self, f: &mut Formatter<'_>, display: D) -> Result {
        write!(f, "{}{}{}", self, display, self.reset())
    }
}

macro_rules! impl_style {
    ($style:ident, $reset:ident) => {
        impl Style for $style {
            type Reset = $reset;

            fn reset(&self) -> Self::Reset {
                $reset
            }
        }
    };
}

impl_style! { Bold, NoBold }
impl_style! { Underline, NoUnderline }
