use std::fmt;
use std::fmt::Display;
use std::ops::{Deref, DerefMut};

use ansi_term::{Color, Style};

/// Wraps something in a [`Style`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Styled<T> {
    inner: T,
    style: Style,
}

impl<T> Styled<T> {
    /// Construct a new `Styled`.
    fn new(inner: T) -> Self {
        Self {
            inner,
            style: Style::new(),
        }
    }
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
        fn $meth(self) -> Styled<Self::Output> {
            self.on(Color::$color)
        }
    };
    ($meth:ident, $color:ident($($arg:ident),+)) => {
        fn $meth(self, $($arg: u8),+) -> Styled<Self::Output> {
            self.fg(Color::$color($($arg),+))
        }
    };
}

macro_rules! meth_style {
    ($meth:ident) => {
        fn $meth(self) -> Styled<Self::Output> {
            let Styled { inner, style } = self.styled();
            Styled {
                inner,
                style: style.$meth(),
            }
        }
    };
    ($meth:ident, $color:ident) => {
        fn $meth(self, $color: Color) -> Styled<Self::Output> {
            let Styled { inner, style } = self.styled();
            Styled {
                inner,
                style: style.$meth($color),
            }
        }
    };
}

/// Allows anything implementing `Display` to be styled.
pub trait Stylize<T>: Sized {
    type Output;

    fn styled(self) -> Styled<Self::Output>;

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

/// Special implementation for [`Styled`] to prevent unnecessary copying.
impl<T> Stylize<T> for Styled<T>
where
    T: Display,
{
    type Output = T;

    fn styled(self) -> Styled<Self::Output> {
        self
    }
}

/// Blanket implementation for everything that implements [`Display`].
impl<T> Stylize<T> for T
where
    T: Display,
{
    type Output = Self;

    fn styled(self) -> Styled<Self::Output> {
        Styled::new(self)
    }
}
