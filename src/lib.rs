pub mod prelude;

use std::fmt;
use std::fmt::Display;

use ansi_term::{Color, Style};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Colored<T> {
    inner: T,
    color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Styled<T> {
    inner: T,
    style: Style,
}

pub trait Colorize<T> {
    fn colored(&self, color: Color) -> Colored<&Self> {
        Colored { inner: self, color }
    }
    fn black(&self) -> Colored<&Self> {
        self.colored(Color::Black)
    }
    fn red(&self) -> Colored<&Self> {
        self.colored(Color::Red)
    }
    fn green(&self) -> Colored<&Self> {
        self.colored(Color::Green)
    }
    fn yellow(&self) -> Colored<&Self> {
        self.colored(Color::Yellow)
    }
    fn blue(&self) -> Colored<&Self> {
        self.colored(Color::Blue)
    }
    fn purple(&self) -> Colored<&Self> {
        self.colored(Color::Purple)
    }
    fn cyan(&self) -> Colored<&Self> {
        self.colored(Color::Cyan)
    }
    fn white(&self) -> Colored<&Self> {
        self.colored(Color::White)
    }
    fn fixed(&self, n: u8) -> Colored<&Self> {
        self.colored(Color::Fixed(n))
    }
    fn rgb(&self, r: u8, g: u8, b: u8) -> Colored<&Self> {
        self.colored(Color::RGB(r, g, b))
    }
}

pub trait Stylize<T> {
    fn styled(&self, style: Style) -> Styled<&Self> {
        Styled { inner: self, style }
    }
    fn bold(&self) -> Styled<&Self> {
        self.styled(Style::new().bold())
    }
    fn dimmed(&self) -> Styled<&Self> {
        self.styled(Style::new().dimmed())
    }
    fn italic(&self) -> Styled<&Self> {
        self.styled(Style::new().italic())
    }
    fn underline(&self) -> Styled<&Self> {
        self.styled(Style::new().underline())
    }
    fn blink(&self) -> Styled<&Self> {
        self.styled(Style::new().blink())
    }
    fn reverse(&self) -> Styled<&Self> {
        self.styled(Style::new().reverse())
    }
    fn hidden(&self) -> Styled<&Self> {
        self.styled(Style::new().hidden())
    }
    fn strikethrough(&self) -> Styled<&Self> {
        self.styled(Style::new().strikethrough())
    }
}

impl<T: Display> Display for Colored<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.color.paint(self.inner.to_string()), f)
    }
}

impl<T: Display> Display for Styled<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.style.paint(self.inner.to_string()), f)
    }
}

/// Blanket implementation for everything that implements [`Display`].
///
/// Note: this is includes [`Colored`] and [`Styled`].
impl<T> Colorize<T> for T where T: Display {}

/// Blanket implementation for everything that implements [`Display`].
///
/// Note: this is includes [`Colored`] and [`Styled`].
impl<T> Stylize<T> for T where T: Display {}
