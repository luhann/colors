use rgbtohex::*;

#[test]
fn test_parse_rgb_valid_inputs() {
    let result = parse_rgb("255,0,128").unwrap();
    assert_eq!(result.0, [255, 0, 128]);
    assert_eq!(result.1, None);

    let result = parse_rgb("255, 0, 128, 0.50").unwrap();
    assert_eq!(result.0, [255, 0, 128]);
    assert_eq!(result.1, Some(0.50));
}

#[test]
fn test_parse_rgb_errors() {
    assert_eq!(parse_rgb("255,0"), Err("Invalid RGB format"));
    assert_eq!(parse_rgb("255,0,0,0.5,1"), Err("Invalid RGB format"));
    assert_eq!(parse_rgb("255,blue,0"), Err("Invalid RGB component value"));
    assert_eq!(parse_rgb("300,0,0"), Err("Invalid RGB component value"));
    assert_eq!(
        parse_rgb("255,0,0,2.0"),
        Err("Alpha must be between 0.0 and 1.0")
    );
    assert_eq!(parse_rgb("255,0,0,nope"), Err("Invalid alpha value"));
}

#[test]
fn test_convert_rgb_to_format_hex() {
    assert_eq!(convert_rgb_to_format("255,0,0", "hex").unwrap(), "#FF0000");
    assert_eq!(
        convert_rgb_to_format("255, 0, 0, 0.5", "hex").unwrap(),
        "#FF00007F"
    );
}

#[test]
fn test_convert_rgb_to_format_other_formats() {
    assert_eq!(
        convert_rgb_to_format("255,0,0", "standard").unwrap(),
        "RGB(255, 0, 0)"
    );
    assert_eq!(
        convert_rgb_to_format("255,0,0,0.67", "css").unwrap(),
        "rgba(255, 0, 0, 0.67)"
    );
    assert_eq!(
        convert_rgb_to_format("255,0,0", "json").unwrap(),
        r#"{"r": 255, "g": 0, "b": 0}"#
    );
    assert_eq!(
        convert_rgb_to_format("255,0,0,0.67", "compact").unwrap(),
        "255,0,0,0.67"
    );
}

#[test]
fn test_convert_rgb_to_format_errors() {
    assert_eq!(
        convert_rgb_to_format("255,0,0", "invalid").unwrap_err(),
        "Unknown format: invalid"
    );
    assert_eq!(
        convert_rgb_to_format("255,0", "hex").unwrap_err(),
        "Invalid RGB format"
    );
}

#[test]
fn test_rgbtohex_function() {
    assert_eq!(rgbtohex("255,0,0"), "#FF0000");
    assert_eq!(rgbtohex("255,0,0,0.5"), "#FF00007F");
    assert_eq!(rgbtohex("255,0"), "Invalid RGB format");
}
