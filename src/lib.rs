//! Peter builds on the [`termion`] crate to allow styling of anything
//! implementing [`Display`] and makes colorizing text less verbose to use by
//! providing the [`Stylize`] trait.
//!
//! # Examples
//!
//! ```rust
//! use peter::{color, style, Stylize};
//!
//! println!("This is in red: {}", "a red string".fg(color::Red));
//!
//! println!(
//!     "How about some {}?",
//!     "bold and underline"
//!         .style(style::Bold)
//!         .style(style::Underline)
//! );
//! ```

mod impl_style;

use std::fmt;
use std::fmt::Display;
use std::ops::{Deref, DerefMut};

use termion::color::Color;
pub use termion::{color, style};

use crate::impl_style::Style;

pub trait Stylize: Display {
    fn fg<C: Color>(&self, color: C) -> Fg<&Self, C> {
        Fg::new(self, color)
    }
    fn bg<C: Color>(&self, color: C) -> Bg<&Self, C> {
        Bg::new(self, color)
    }
    fn style<S: Style>(&self, style: S) -> Styled<&Self, S> {
        Styled::new(self, style)
    }
}

/// Wraps something in a foreground color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Fg<D: Display, C: Color> {
    inner: D,
    color: C,
}

/// Wraps something in a background color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Bg<D: Display, C: Color> {
    inner: D,
    color: C,
}

/// Wraps something in a style.
pub struct Styled<D: Display, S: Style> {
    inner: D,
    style: S,
}

impl<D: Display, C: Color> Fg<D, C> {
    /// Construct a new `Fg`.
    fn new(inner: D, color: C) -> Self {
        Self { inner, color }
    }

    pub fn fg(mut self, color: C) -> Self {
        self.color = color;
        self
    }
}

impl<D: Display, C: Color> Bg<D, C> {
    /// Construct a new `Bg`.
    fn new(inner: D, color: C) -> Self {
        Self { inner, color }
    }

    pub fn bg(mut self, color: C) -> Self {
        self.color = color;
        self
    }
}

impl<D: Display, S: Style> Styled<D, S> {
    fn new(inner: D, style: S) -> Self {
        Self { inner, style }
    }
}

impl<D: Display, C: Color> Deref for Fg<D, C> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<D: Display, C: Color> DerefMut for Fg<D, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<D: Display, C: Color> Deref for Bg<D, C> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<D: Display, C: Color> DerefMut for Bg<D, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<D: Display, S: Style> Deref for Styled<D, S> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<D: Display, S: Style> DerefMut for Styled<D, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<D: Display, C: Color + Copy> Display for Fg<D, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            color::Fg(self.color),
            self.inner,
            color::Fg(color::Reset)
        )
    }
}

impl<D: Display, C: Color + Copy> Display for Bg<D, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            color::Bg(self.color),
            self.inner,
            color::Bg(color::Reset)
        )
    }
}

impl<D: Display, S: Style + Copy> Display for Styled<D, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.style.write_styled(f, &self.inner)
    }
}

impl<T> Stylize for T where T: Display {}
