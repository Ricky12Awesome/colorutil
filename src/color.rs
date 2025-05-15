use crate::config::Palette;
use crate::{Error, Result};
use itertools::Itertools;
use palette::rgb::{Rgb, Rgba};
use palette::{Hsl, Hsla, Hsv, Hsva, IntoColor, Srgb, Srgba, WithAlpha};
use std::str::FromStr;

macro_rules! impl_color {
    ($format:ty) => {
        impl Into<$format> for Color {
            fn into(self) -> $format {
                match self {
                    Self::Srgb(color) => color.into_color(),
                    Self::Rgba(color) => color.into_color(),
                    Self::Hsla(color) => color.into_color(),
                    Self::Hsva(color) => color.into_color(),
                }
            }
        }
    };
}

macro_rules! impl_from_color {
    ($($name:ident($t:ty) -> $color:ident:$format:tt $(a=$alpha:literal)?),+ $(,)?) => {
        $(pub fn $name(value: $t) -> Self {
            let value = $format::from(value);
            $(let value = value.with_alpha($alpha);)?
            let value = value.into();
            Self::$color(value)
        })+
    };
}

macro_rules! impl_match_color {
    ($src:ident, $($name:literal: $from:ident<$t:tt, $n:literal>),+ $(,)?) => {
        $(if $src.starts_with(concat!($name, "(")) {
            let params = parse_params::<$t, $n>($src)?;
            let color = Color::$from(params);
            return Ok(color);
        })+
    };
}

macro_rules! impl_from_color_map {
    ($($name:ident($t:ty) -> $from:ident($($n:literal),+)),+ $(,)?) => {
        $(pub fn $name(value: $t) -> Self {
            Self::$from([$(value[$n]),+])
        })+
    };
}

macro_rules! impl_to_color_map {
    ($($name:literal: $fname:ident -> $to:tt<$($t:tt),+>(|$($n:ident),+| $($fmt:tt)*)),+ $(,)?) => {
        $(pub fn $fname(self) -> String {
            let __into_color: $to = self.into();
            let ($($n),+) = __into_color.into_format::<$($t),+>().into_components();

            format!($($fmt)*)
        })+

        pub fn to_format(self, format: &str) -> Result<String> {
            match format {
                $($name => Ok(self.$fname())),+,
                _ => Err(Error::FailedToParseFormat(format.to_owned()))
            }
        }
    };
}

#[derive(Debug, Clone)]
pub enum Color {
    Srgb(Srgb),
    Rgba(Rgba),
    Hsla(Hsla),
    Hsva(Hsva),
}

impl Color {
    impl_from_color!(
        from_rgba([u8; 4]) -> Rgba : Rgba,
        from_rgb([u8; 3]) -> Rgba : Rgb a=255,
        from_frgba([f32; 4]) -> Rgba : Srgba,
        from_frgb([f32; 3]) -> Rgba : Srgb a=1.0,
        from_hsla([f32; 4]) -> Hsla : Hsla,
        from_hsl([f32; 3]) -> Hsla : Hsl a=1.0,
        from_hsva([f32; 4]) -> Hsva : Hsva,
        from_hsv([f32; 3]) -> Hsva : Hsv a=1.0,
    );

    impl_from_color_map!(
        from_argb([u8; 4]) -> from_rgba(3, 0, 1, 2),
        from_fargb([f32; 4]) -> from_frgba(3, 0, 1, 2),
        from_ahsl([f32; 4]) -> from_hsla(3, 0, 1, 2),
        from_ahsv([f32; 4]) -> from_hsva(3, 0, 1, 2),
    );

    impl_to_color_map!(
        "ahex": to_ahex -> Rgba<u8, u8>(|r, g, b, a| "#{a:02X}{r:02X}{g:02X}{b:02X}"),
        "hexa": to_hexa -> Rgba<u8, u8>(|r, g, b, a| "#{r:02X}{g:02X}{b:02X}{a:02X}"),
        "hex": to_hex -> Rgb<u8>(|r, g, b| "#{r:02X}{g:02X}{b:02X}"),
        "argb": to_argb -> Rgba<u8, u8>(|r, g, b, a| "{a}, {r}, {g}, {b}"),
        "rgba": to_rgba -> Rgba<u8, u8>(|r, g, b, a| "{r}, {g}, {b}, {a}"),
        "rgb": to_rgb -> Rgb<u8>(|r, g, b| "{r}, {g}, {b}"),
        "fargb": to_fargb -> Srgba<f32, f32>(|r, g, b, a| "{a}, {r}, {g}, {b}"),
        "frgba": to_frgba -> Srgba<f32, f32>(|r, g, b, a| "{r}, {g}, {b}, {a}"),
        "frgb": to_frgb -> Srgb<u8>(|r, g, b| "{r}, {g}, {b}"),
        "ahsl": to_ahsl -> Hsla<f32, f32>(|h, s, l, a| "{a}, {}, {s}, {l},", h.into_inner()),
        "hsla": to_hsla -> Hsla<f32, f32>(|h, s, l, a| "{}, {s}, {l}, {a}", h.into_inner()),
        "hsl": to_hsl -> Hsl<f32>(|h, s, l| "{}, {s}, {l}", h.into_inner()),
        "ahsv": to_ahsv -> Hsva<f32, f32>(|h, s, v, a| "{a}, {}, {s}, {v}", h.into_inner()),
        "hsva": to_hsva -> Hsva<f32, f32>(|h, s, v, a| "{}, {s}, {v}, {a}", h.into_inner()),
        "hsv": to_hsv -> Hsv<f32>(|h, s, v| "{}, {s}, {v}", h.into_inner()),
    );
}

impl_color!(Srgba);
impl_color!(Rgb);
impl_color!(Hsl);
impl_color!(Hsla);
impl_color!(Hsv);
impl_color!(Hsva);

impl Color {
    fn from_str(s: &str, palette: &Palette) -> Result<Self, Error> {
        impl_match_color!(s,
            "argb": from_argb<u8, 4>,
            "rgba": from_rgba<u8, 4>,
            "rgb": from_rgb<u8, 3>,
            "fargb": from_fargb<f32, 4>,
            "frgba": from_frgba<f32, 4>,
            "frgb": from_frgb<f32, 3>,
            "ahsl": from_ahsl<f32, 4>,
            "hsla": from_hsla<f32, 4>,
            "hsl": from_hsl<f32, 3>,
            "ahsv": from_ahsv<f32, 4>,
            "hsva": from_hsva<f32, 4>,
            "hsv": from_hsv<f32, 3>,
        );

        if s.starts_with('#') {
            let rgba = Rgba::from_str(s)
                .or_else(|_| Rgb::from_str(s).map(Into::into))
                .map_err(|_| Error::FailedToParseColor(s.to_owned()))?;

            return Ok(Self::Rgba(rgba.into()));
        }

        if s.starts_with('$') {
            let s = palette
                .get(s.trim_start_matches('$'))
                .ok_or_else(|| Error::FailedToParseColor(s.to_owned()))?;

            return Self::from_str(s, palette);
        }

        palette::named::from_str(s)
            .map(Srgb::into)
            .map(Self::Srgb)
            .ok_or_else(|| Error::FailedToParseColor(s.to_owned()))
    }
}

pub fn parse_params<T: FromStr, const N: usize>(text: &str) -> Result<[T; N]> {
    let start = text
        .find("(")
        .ok_or_else(|| Error::FailedToParseColorParams(text.to_owned()))?;
    let end = text
        .find(")")
        .ok_or_else(|| Error::FailedToParseColorParams(text.to_owned()))?;

    let result = text[start + 1..end]
        .split(",")
        .map(str::trim)
        .map(str::parse)
        .flatten()
        .collect_array::<N>()
        .ok_or_else(|| Error::FailedToParseColorParams(text.to_owned()))?;

    Ok(result)
}

pub fn parse_format<'a>(src_color: &'a str, format: &'a str, palette: &Palette) -> Result<String> {
    let color = Color::from_str(src_color, palette)?;

    color.to_format(format)
}

pub fn parse_text(
    src: impl AsRef<str>,
    prefix: impl AsRef<str>,
    suffix: impl AsRef<str>,
    palette: &Palette,
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

        let color = palette
            .get(name)
            .ok_or_else(|| Error::FailedToGetColor(value.to_owned()))?;

        let color = parse_format(color, format, palette)?;

        dst.push_str(&src[offset..start]);
        dst.push_str(&color);
        offset = start + end + suffix.len();
    }

    dst.push_str(&src[offset..]);

    Ok(dst)
}
