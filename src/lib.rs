//! Peter builds on the [`ansi_term`] crate to allow styling of anything
//! implementing [`Display`] and makes colorizing text less verbose to use by
//! providing the [`Stylize`] trait.
//!
//! # Examples
//!
//! ```rust
//! use peter::Stylize;
//!
//! println!("This is in red: {}", "a red string".red());
//!
//! println!("How about some {}?", "bold and underline".bold().underline());
//! ```

#![no_std]

use core::fmt;
use core::fmt::Display;
use core::ops::{Deref, DerefMut};

use ansi_term::Style;

/// See [`ansi_term::Color`].
///
pub use ansi_term::Color;

/// Wraps something in a [`Style`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Styled<T> {
    inner: T,
    style: Style,
}

macro_rules! meth_color {
    ($meth:ident, $color:ident) => {
        pub fn $meth(self) -> Self {
            self.on(Color::$color)
        }
    };
    ($meth:ident, $color:ident($($arg:ident),+)) => {
        pub fn $meth(self, $($arg: u8),+) -> Self {
            self.fg(Color::$color($($arg),+))
        }
    };
}

macro_rules! meth_style {
    ($meth:ident) => {
        pub fn $meth(self) -> Self {
            let Self { inner, style } = self;
            Self {
                inner,
                style: style.$meth(),
            }
        }
    };
    ($meth:ident, $color:ident) => {
        pub fn $meth(self, $color: Color) -> Self {
            let Self { inner, style } = self;
            Self {
                inner,
                style: style.$meth($color),
            }
        }
    };
}

impl<T> Styled<T> {
    /// Construct a new `Styled`.
    fn new(inner: T) -> Self {
        Self {
            inner,
            style: Style::new(),
        }
    }

    // different styles
    meth_style!(bold);
    meth_style!(dimmed);
    meth_style!(italic);
    meth_style!(underline);
    meth_style!(blink);
    meth_style!(reverse);
    meth_style!(hidden);
    meth_style!(strikethrough);
    meth_style!(fg, color);
    meth_style!(on, color);

    // different colors
    meth_color!(black, Black);
    meth_color!(red, Red);
    meth_color!(green, Green);
    meth_color!(yellow, Yellow);
    meth_color!(blue, Blue);
    meth_color!(magenta, Purple);
    meth_color!(cyan, Cyan);
    meth_color!(white, White);
    meth_color!(fixed, Fixed(n));
    meth_color!(rgb, RGB(r, g, b));
}

impl<T> Deref for Styled<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Styled<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: Display> Display for Styled<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.style.prefix())?;
        write!(f, "{}", self.inner)?;
        write!(f, "{}", self.style.suffix())?;
        Ok(())
    }
}

macro_rules! meth_color {
    ($meth:ident, $color:ident) => {
        fn $meth(self) -> Styled<Self> {
            self.on(Color::$color)
        }
    };
    ($meth:ident, $color:ident($($arg:ident),+)) => {
        fn $meth(self, $($arg: u8),+) -> Styled<Self> {
            self.fg(Color::$color($($arg),+))
        }
    };
}

macro_rules! meth_style {
    ($meth:ident) => {
        fn $meth(self) -> Styled<Self> {
            Styled::new(self).$meth()
        }
    };
    ($meth:ident, $color:ident) => {
        fn $meth(self, $color: Color) -> Styled<Self> {
            Styled::new(self).$meth($color)
        }
    };
}

/// Allows anything implementing [`Display`] to be styled.
pub trait Stylize<T>: Sized {
    // different styles
    meth_style!(bold);
    meth_style!(dimmed);
    meth_style!(italic);
    meth_style!(underline);
    meth_style!(blink);
    meth_style!(reverse);
    meth_style!(hidden);
    meth_style!(strikethrough);
    meth_style!(fg, color);
    meth_style!(on, color);

    // different colors
    meth_color!(black, Black);
    meth_color!(red, Red);
    meth_color!(green, Green);
    meth_color!(yellow, Yellow);
    meth_color!(blue, Blue);
    meth_color!(magenta, Purple);
    meth_color!(cyan, Cyan);
    meth_color!(white, White);
    meth_color!(fixed, Fixed(n));
    meth_color!(rgb, RGB(r, g, b));
}

/// Blanket implementation for everything that implements [`Display`].
impl<T> Stylize<T> for T where T: Display {}
