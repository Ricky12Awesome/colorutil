use crate::color::parse_format;
use crate::{Error, Result};
use derive_more::Deref;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Deref)]
pub struct Colors<'a> {
    #[serde(flatten, borrow)]
    colors: HashMap<&'a str, &'a str>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config<'a> {
    prefix: &'a str,
    suffix: &'a str,
    colors: HashMap<&'a str, Colors<'a>>,
}

pub fn replace_colors<'a>(
    src: impl AsRef<str>,
    prefix: impl AsRef<str>,
    suffix: impl AsRef<str>,
    colors: &'a Colors<'a>,
) -> Result<String> {
    let prefix = prefix.as_ref();
    let suffix = suffix.as_ref();

    let src = src.as_ref();
    let mut dst = String::with_capacity(src.len() * 2);

    let mut offset = 0;

    while let Some(start) = src[offset..].find(prefix) {
        let start = offset + start;
        let end = src[start..].find(suffix).ok_or(Error::FailedToParse)?;
        let value = &src[start + prefix.len()..start + end];
        let (name, format) = value.split_once(":").ok_or(Error::FailedToParse)?;
        let color = colors.get(name).ok_or(Error::FailedToParse)?;
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
    let prefix = "${";
    let suffix = "}";
    let colors = Colors {
        colors: HashMap::from_iter([
            ("white", "#ffffff"),
            ("black", "#000000"),
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
        ]),
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
