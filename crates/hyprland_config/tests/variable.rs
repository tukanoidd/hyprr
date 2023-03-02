use hyprland_config::{
    util::Parse,
    variable::{
        Bool, Color, ControlModifier, Gradient, Modifier, SuperModifier, Variable, VariableValue,
        Vec2,
    },
};

#[test]
fn test_parse_variable() {
    let input = "$test = 10";
    let (rest, var) = Variable::parse(input).unwrap();

    assert_eq!(rest, "");
    assert_eq!(var.name, "test");
    assert_eq!(var.value, VariableValue::Int(10));
}

#[test]
fn test_variable_value_parser() {
    assert_eq!(VariableValue::parse("1"), Ok(("", VariableValue::Int(1))));
    assert_eq!(
        VariableValue::parse("true"),
        Ok(("", VariableValue::Bool(Bool::True)))
    );
    assert_eq!(
        VariableValue::parse("yes"),
        Ok(("", VariableValue::Bool(Bool::Yes)))
    );
    assert_eq!(
        VariableValue::parse("on"),
        Ok(("", VariableValue::Bool(Bool::On)))
    );
    assert_eq!(
        VariableValue::parse("1.0"),
        Ok(("", VariableValue::Float(1.0)))
    );
    assert_eq!(
        VariableValue::parse("0.0"),
        Ok(("", VariableValue::Float(0.0)))
    );
    assert_eq!(
        VariableValue::parse("0.1"),
        Ok(("", VariableValue::Float(0.1)))
    );
    assert_eq!(
        VariableValue::parse("rgba(8f4f9484)"),
        Ok(("", VariableValue::Color(Color::RGBA(0x8f4f9484))))
    );
    assert_eq!(
        VariableValue::parse("rgb(8f4f94)"),
        Ok(("", VariableValue::Color(Color::RGB(0x8f4f94))))
    );
    assert_eq!(
        VariableValue::parse("0x8f4f94f7"),
        Ok(("", VariableValue::Color(Color::Legacy(0x8f4f94f7))))
    );
    assert_eq!(
        VariableValue::parse("0 0"),
        Ok(("", VariableValue::Vec2(Vec2(0.0, 0.0))))
    );
    assert_eq!(
        VariableValue::parse("-10.9 99.1"),
        Ok(("", VariableValue::Vec2(Vec2(-10.9, 99.1))))
    );
    assert_eq!(
        VariableValue::parse("SHIFT"),
        Ok(("", VariableValue::Mod(Modifier::SHIFT))),
    );
    assert_eq!(
        VariableValue::parse("CTRL"),
        Ok((
            "",
            VariableValue::Mod(Modifier::CTRL(ControlModifier::CTRL))
        )),
    );
    assert_eq!(
        VariableValue::parse("\"Hello \""),
        Ok(("", VariableValue::String("Hello ".to_string())))
    );
    assert_eq!(
        VariableValue::parse("rgb(8f4f94) rgba(8f4f9484) 0x8f4f94f7 20deg"),
        Ok((
            "",
            VariableValue::Gradient(Gradient {
                colors: vec![
                    Color::RGB(0x8f4f94),
                    Color::RGBA(0x8f4f9484),
                    Color::Legacy(0x8f4f94f7),
                ],
                angle: Some(20)
            })
        ))
    );
    assert_eq!(
        VariableValue::parse("$variable_test_name"),
        Ok((
            "",
            VariableValue::Variable("variable_test_name".to_string())
        ))
    );
}

#[test]
fn test_bool_parser() {
    assert_eq!(Bool::parse("true"), Ok(("", Bool::True)));
    assert_eq!(Bool::parse("false"), Ok(("", Bool::False)));

    assert_eq!(Bool::parse("yes"), Ok(("", Bool::Yes)));
    assert_eq!(Bool::parse("no"), Ok(("", Bool::No)));

    assert_eq!(Bool::parse("on"), Ok(("", Bool::On)));
    assert_eq!(Bool::parse("off"), Ok(("", Bool::Off)));

    assert_eq!(Bool::parse("1"), Ok(("", Bool::One)));
    assert_eq!(Bool::parse("0"), Ok(("", Bool::Zero)));
}

#[test]
fn test_color_parser() {
    let input = "rgba(b3ff1aee)";
    let rest_color = Color::parse(input);
    assert!(matches!(rest_color, Ok(("", Color::RGBA(0xb3ff1aee)))));

    let input = "rgb(b3ff1a)";
    let rest_color = Color::parse(input);
    assert!(matches!(rest_color, Ok(("", Color::RGB(0xb3ff1a)))));

    let input = "0xeeb3ff1a";
    let rest_color = Color::parse(input);
    assert!(matches!(rest_color, Ok(("", Color::Legacy(0xeeb3ff1a)))));
}

#[test]
fn test_vec2_parser() {
    let input = "1.0 2.0";
    let rest_vec2 = Vec2::parse(input);

    assert_eq!(rest_vec2, Ok(("", Vec2(1.0, 2.0))));

    let input = "3 4";
    let rest_vec2 = Vec2::parse(input);

    assert_eq!(rest_vec2, Ok(("", Vec2(3.0, 4.0))));
}

#[test]
fn modifier_parse() {
    let input = "SHIFT";
    let rest_modifier = Modifier::parse(input);
    assert_eq!(rest_modifier, Ok(("", Modifier::SHIFT)));

    let input = "CAPS";
    let rest_modifier = Modifier::parse(input);
    assert_eq!(rest_modifier, Ok(("", Modifier::CAPS)));

    let input = "CTRL";
    let rest_modifier = Modifier::parse(input);
    assert_eq!(
        rest_modifier,
        Ok(("", Modifier::CTRL(ControlModifier::CTRL)))
    );

    let input = "CONTROL";
    let rest_modifier = Modifier::parse(input);
    assert_eq!(
        rest_modifier,
        Ok(("", Modifier::CTRL(ControlModifier::CONTROL)))
    );

    let input = "ALT";
    let rest_modifier = Modifier::parse(input);
    assert_eq!(rest_modifier, Ok(("", Modifier::ALT)));

    let input = "MOD2";
    let rest_modifier = Modifier::parse(input);
    assert_eq!(rest_modifier, Ok(("", Modifier::MOD2)));

    let input = "MOD3";
    let rest_modifier = Modifier::parse(input);
    assert_eq!(rest_modifier, Ok(("", Modifier::MOD3)));

    let input = "SUPER";
    let rest_modifier = Modifier::parse(input);
    assert_eq!(
        rest_modifier,
        Ok(("", Modifier::SUPER(SuperModifier::SUPER)))
    );

    let input = "WIN";
    let rest_modifier = Modifier::parse(input);
    assert_eq!(rest_modifier, Ok(("", Modifier::SUPER(SuperModifier::WIN))));

    let input = "LOGO";
    let rest_modifier = Modifier::parse(input);
    assert_eq!(
        rest_modifier,
        Ok(("", Modifier::SUPER(SuperModifier::LOGO)))
    );

    let input = "MOD4";
    let rest_modifier = Modifier::parse(input);
    assert_eq!(
        rest_modifier,
        Ok(("", Modifier::SUPER(SuperModifier::MOD4)))
    );

    let input = "MOD5";
    let rest_modifier = Modifier::parse(input);
    assert_eq!(rest_modifier, Ok(("", Modifier::MOD5)));
}

#[test]
fn test_gradient_parser() {
    let input = "rgba(b3ff1aee) rgb(b3ff1a) 0xeeb3ff1a";
    let res = (
        "",
        Gradient {
            colors: vec![
                Color::RGBA(0xb3ff1aee),
                Color::RGB(0xb3ff1a),
                Color::Legacy(0xeeb3ff1a),
            ],
            angle: None,
        },
    );

    let rest_gradient = Gradient::parse(input);
    assert_eq!(rest_gradient, Ok(res));

    let input = "rgba(b3ff1aee) rgb(b3ff1a) 0xeeb3ff1a 20deg";
    let res = (
        "",
        Gradient {
            colors: vec![
                Color::RGBA(0xb3ff1aee),
                Color::RGB(0xb3ff1a),
                Color::Legacy(0xeeb3ff1a),
            ],
            angle: Some(20),
        },
    );

    let rest_gradient = Gradient::parse(input);
    assert_eq!(rest_gradient, Ok(res));
}
