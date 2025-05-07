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

macro_rules! impl_to_color {
    ($($name:ident -> $format:ty),+) => {
        $(pub fn $name(self) -> $format {
            self.into()
        })+
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

    impl_to_color!(
        to_srgb -> Srgb,
        to_srgba -> Srgba,
        to_rgb -> Rgb,
        to_rgba -> Rgba,
        to_hsl -> Hsl,
        to_hsla -> Hsla,
        to_hsv -> Hsv,
        to_hsva -> Hsva
    );
}

impl_color!(Srgba);
impl_color!(Rgb);
impl_color!(Hsl);
impl_color!(Hsla);
impl_color!(Hsv);
impl_color!(Hsva);

impl FromStr for Color {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        impl_match_color!(s,
            "argb": from_argb<u8, 4>,
            "rgba": from_rgba<u8, 4>,
            "rgb": from_rgb<u8, 3>,
            "fargb": from_fargb<f32, 4>,
            "frgba": from_frgba<f32, 4>,
            "frgb": from_frgb<f32, 3>,
            "ahsv": from_ahsv<f32, 4>,
            "hsla": from_hsla<f32, 4>,
            "hsl": from_hsl<f32, 3>,
            "ahsl": from_ahsl<f32, 4>,
            "hsva": from_hsva<f32, 4>,
            "hsv": from_hsv<f32, 3>,
        );

        if s.starts_with('#') {
            let rgba = Rgba::from_str(s)
                .or_else(|_| Rgb::from_str(s).map(Into::into))
                .map_err(|_| Error::FailedToParseColor(s.to_owned()))?;

            return Ok(Color::Rgba(rgba.into()));
        }

        palette::named::from_str(s)
            .map(Srgb::into)
            .map(Color::Srgb)
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

pub fn parse_format<'a>(src_color: &'a str, format: &'a str) -> Result<String> {
    let color = src_color.parse::<Color>()?;

    match format {
        "hexa" => {
            let rgba = color.to_rgba().into_format::<u8, u8>();
            let (r, g, b, a) = rgba.into_components();
            let color = format!("#{r:02X}{g:02X}{b:02X}{a:02X}");

            Ok(color)
        }
        "hex" => {
            let rgb = color.to_rgb().into_format::<u8>();
            let (r, g, b) = rgb.into_components();
            let color = format!("#{r:02X}{g:02X}{b:02X}");

            Ok(color)
        }
        "frgb" => {
            let rgb = color.to_rgb();
            let (r, g, b) = rgb.into_components();
            let color = format!("{r}, {g}, {b}");

            Ok(color)
        }
        "frgba" => {
            let rgba = color.to_rgba();
            let (r, g, b, a) = rgba.into_components();
            let color = format!("{r}, {g}, {b}, {a}");

            Ok(color)
        }
        "fargb" => {
            let rgba = color.to_rgba();
            let (r, g, b, a) = rgba.into_components();
            let color = format!("{a}, {r}, {g}, {b}");

            Ok(color)
        }
        "rgb" => {
            let rgb = color.to_rgb().into_format::<u8>();
            let (r, g, b) = rgb.into_components();
            let color = format!("{r}, {g}, {b}");

            Ok(color)
        }
        "rgba" => {
            let rgba = color.to_rgba().into_format::<u8, u8>();
            let (r, g, b, a) = rgba.into_components();
            let color = format!("{r}, {g}, {b}, {a}");

            Ok(color)
        }
        "argb" => {
            let rgba = color.to_rgba().into_format::<u8, u8>();
            let (r, g, b, a) = rgba.into_components();
            let color = format!("{a}, {r}, {g}, {b}");

            Ok(color)
        }
        "hsl" => {
            let hsl = color.to_hsl();
            let (h, s, l) = hsl.into_components();
            let h = h.into_inner();
            let color = format!("{h}, {s}, {l}");

            Ok(color)
        }
        "hsla" => {
            let hsla = color.to_hsla();
            let (h, s, l, a) = hsla.into_components();
            let h = h.into_inner();
            let color = format!("{h}, {s}, {l}, {a}");

            Ok(color)
        }
        "hsv" => {
            let hsv = color.to_hsv();
            let (h, s, v) = hsv.into_components();
            let h = h.into_inner();
            let color = format!("{h}, {s}, {v}");

            Ok(color)
        }
        "hsva" => {
            let hsva = color.to_hsva();
            let (h, s, v, a) = hsva.into_components();
            let h = h.into_inner();
            let color = format!("{h}, {s}, {v}, {a}");

            Ok(color)
        }
        _ => Err(Error::FailedToParseFormat(format.to_owned())),
    }
}
