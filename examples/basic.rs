use peter::{color, style, Stylize};

fn main() {
    println!("This is in red: {}", "a red string".fg(color::Red));

    println!(
        "How about some {}?",
        "bold and underline"
            .style(style::Bold)
            .style(style::Underline)
    );
}
