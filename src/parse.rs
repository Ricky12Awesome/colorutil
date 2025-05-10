use crate::color::parse_format;
use crate::config::Palette;
use crate::{Error, Result};

pub fn replace_colors(
    src: impl AsRef<str>,
    prefix: impl AsRef<str>,
    suffix: impl AsRef<str>,
    colors: &Palette,
) -> Result<String> {
    let prefix = prefix.as_ref();
    let suffix = suffix.as_ref();

    let src = src.as_ref();
    let mut dst = String::with_capacity(src.len() * 2);

    let mut offset = 0;

    while let Some(start) = src[offset..].find(prefix) {
        let start = offset + start;
        let end = src[start..]
            .find(suffix)
            .ok_or_else(|| Error::FailedToFindSuffix(start))?;

        let value = &src[start + prefix.len()..start + end];
        let (name, format) = value
            .split_once(":")
            .ok_or_else(|| Error::FailedToParseValue(value.to_string()))?;

        let color = colors
            .get(name)
            .ok_or_else(|| Error::FailedToGetColor(value.to_owned()))?;

        let color = parse_format(color, format)?;

        dst.push_str(&src[offset..start]);
        dst.push_str(&color);
        offset = start + end + suffix.len();
    }

    dst.push_str(&src[offset..]);

    Ok(dst)
}

#[test]
fn test_replace_colors() {
    use std::collections::HashMap;

    let prefix = "${";
    let suffix = "}";
    let colors = Palette {
        colors: HashMap::from_iter(
            [
                ("white", "white"),
                ("black", "black"),
                ("transparent", "#00000000"),
                ("mid", "rgb(128, 128, 128)"),
                ("mid2", "frgb(0.5, 0.5, 0.5)"),
                ("rgb", "rgb(1, 2, 3)"),
                ("rgba", "rgba(1, 2, 3, 255)"),
                ("argb", "argb(1, 2, 3, 255)"),
                ("hsl", "hsl(360, 1.0, 0.5)"),
                ("hsla", "hsla(360, 1.0, 0.5, 1.0)"),
                ("hsv", "hsv(360, 1.0, 0.5)"),
                ("hsva", "hsva(360, 1.0, 0.5, 1.0)"),
            ]
            .map(|(a, b)| (a.into(), b.into())),
        ),
        ..Palette::default()
    };

    let src = r#"
white: ${white:hex}
black: ${black:hex}
transparent: ${transparent:hex}
mid: ${mid:frgb}
mid2: ${mid2:fargb}
rgb: ${rgb:rgb}
rgba: ${rgba:rgba}
argb: ${argb:argb}
hsl: ${hsl:hsl}
hsla: ${hsla:hsla}
hsv: ${hsv:hsv}
hsva: ${hsva:hsva}
"#;
    let result = replace_colors(src, prefix, suffix, &colors).unwrap();

    println!("{}", result);
}
