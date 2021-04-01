use peter::Stylize;

/// Check that width parameters does not include the ansi codes.
#[test]
fn format_width() {
    let result = format!("{:>8}", "test".bold().blue());
    assert_eq!(result, "\u{1b}[1;34m    test\u{1b}[0m");
}
