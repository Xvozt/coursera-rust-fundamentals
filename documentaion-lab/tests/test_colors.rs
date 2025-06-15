use documentaion_lab::colors::{Color, ColorString};

#[test]
fn test_red_painting() {
    let mut color_string = ColorString {
        color: Color::Red,
        string: String::from("Red"),
        colorized: String::from(""),
    };

    color_string.paint();
    assert_eq!(color_string.colorized, "\x1b[31mRed\x1b[0m");
}

#[test]
fn test_empty_string_painting() {
    let mut color_string = ColorString {
        color: Color::Red,
        string: String::from(""),
        colorized: String::from(""),
    };

    color_string.paint();
    assert_eq!(color_string.colorized, "\x1b[31m\x1b[0m");
}
