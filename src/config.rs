use crate::{Error, Result};
use derive_more::Deref;
use serde::{Deserialize, Deserializer, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

// copy and pasted from palette::named::COLORS since it's private for whatever reason
#[rustfmt::skip]
pub const DEFAULT_COLORS: [(Cow<'static, str>, Cow<'static, str>); 148] = [
    (Cow::Borrowed("aliceblue"), Cow::Borrowed("aliceblue")),
    (Cow::Borrowed("antiquewhite"), Cow::Borrowed("antiquewhite")),
    (Cow::Borrowed("aqua"), Cow::Borrowed("aqua")),
    (Cow::Borrowed("aquamarine"), Cow::Borrowed("aquamarine")),
    (Cow::Borrowed("azure"), Cow::Borrowed("azure")),
    (Cow::Borrowed("beige"), Cow::Borrowed("beige")),
    (Cow::Borrowed("bisque"), Cow::Borrowed("bisque")),
    (Cow::Borrowed("black"), Cow::Borrowed("black")),
    (Cow::Borrowed("blanchedalmond"), Cow::Borrowed("blanchedalmond")),
    (Cow::Borrowed("blue"), Cow::Borrowed("blue")),
    (Cow::Borrowed("blueviolet"), Cow::Borrowed("blueviolet")),
    (Cow::Borrowed("brown"), Cow::Borrowed("brown")),
    (Cow::Borrowed("burlywood"), Cow::Borrowed("burlywood")),
    (Cow::Borrowed("cadetblue"), Cow::Borrowed("cadetblue")),
    (Cow::Borrowed("chartreuse"), Cow::Borrowed("chartreuse")),
    (Cow::Borrowed("chocolate"), Cow::Borrowed("chocolate")),
    (Cow::Borrowed("coral"), Cow::Borrowed("coral")),
    (Cow::Borrowed("cornflowerblue"), Cow::Borrowed("cornflowerblue")),
    (Cow::Borrowed("cornsilk"), Cow::Borrowed("cornsilk")),
    (Cow::Borrowed("crimson"), Cow::Borrowed("crimson")),
    (Cow::Borrowed("cyan"), Cow::Borrowed("cyan")),
    (Cow::Borrowed("darkblue"), Cow::Borrowed("darkblue")),
    (Cow::Borrowed("darkcyan"), Cow::Borrowed("darkcyan")),
    (Cow::Borrowed("darkgoldenrod"), Cow::Borrowed("darkgoldenrod")),
    (Cow::Borrowed("darkgray"), Cow::Borrowed("darkgray")),
    (Cow::Borrowed("darkgreen"), Cow::Borrowed("darkgreen")),
    (Cow::Borrowed("darkgrey"), Cow::Borrowed("darkgrey")),
    (Cow::Borrowed("darkkhaki"), Cow::Borrowed("darkkhaki")),
    (Cow::Borrowed("darkmagenta"), Cow::Borrowed("darkmagenta")),
    (Cow::Borrowed("darkolivegreen"), Cow::Borrowed("darkolivegreen")),
    (Cow::Borrowed("darkorange"), Cow::Borrowed("darkorange")),
    (Cow::Borrowed("darkorchid"), Cow::Borrowed("darkorchid")),
    (Cow::Borrowed("darkred"), Cow::Borrowed("darkred")),
    (Cow::Borrowed("darksalmon"), Cow::Borrowed("darksalmon")),
    (Cow::Borrowed("darkseagreen"), Cow::Borrowed("darkseagreen")),
    (Cow::Borrowed("darkslateblue"), Cow::Borrowed("darkslateblue")),
    (Cow::Borrowed("darkslategray"), Cow::Borrowed("darkslategray")),
    (Cow::Borrowed("darkslategrey"), Cow::Borrowed("darkslategrey")),
    (Cow::Borrowed("darkturquoise"), Cow::Borrowed("darkturquoise")),
    (Cow::Borrowed("darkviolet"), Cow::Borrowed("darkviolet")),
    (Cow::Borrowed("deeppink"), Cow::Borrowed("deeppink")),
    (Cow::Borrowed("deepskyblue"), Cow::Borrowed("deepskyblue")),
    (Cow::Borrowed("dimgray"), Cow::Borrowed("dimgray")),
    (Cow::Borrowed("dimgrey"), Cow::Borrowed("dimgrey")),
    (Cow::Borrowed("dodgerblue"), Cow::Borrowed("dodgerblue")),
    (Cow::Borrowed("firebrick"), Cow::Borrowed("firebrick")),
    (Cow::Borrowed("floralwhite"), Cow::Borrowed("floralwhite")),
    (Cow::Borrowed("forestgreen"), Cow::Borrowed("forestgreen")),
    (Cow::Borrowed("fuchsia"), Cow::Borrowed("fuchsia")),
    (Cow::Borrowed("gainsboro"), Cow::Borrowed("gainsboro")),
    (Cow::Borrowed("ghostwhite"), Cow::Borrowed("ghostwhite")),
    (Cow::Borrowed("gold"), Cow::Borrowed("gold")),
    (Cow::Borrowed("goldenrod"), Cow::Borrowed("goldenrod")),
    (Cow::Borrowed("gray"), Cow::Borrowed("gray")),
    (Cow::Borrowed("grey"), Cow::Borrowed("grey")),
    (Cow::Borrowed("green"), Cow::Borrowed("green")),
    (Cow::Borrowed("greenyellow"), Cow::Borrowed("greenyellow")),
    (Cow::Borrowed("honeydew"), Cow::Borrowed("honeydew")),
    (Cow::Borrowed("hotpink"), Cow::Borrowed("hotpink")),
    (Cow::Borrowed("indianred"), Cow::Borrowed("indianred")),
    (Cow::Borrowed("indigo"), Cow::Borrowed("indigo")),
    (Cow::Borrowed("ivory"), Cow::Borrowed("ivory")),
    (Cow::Borrowed("khaki"), Cow::Borrowed("khaki")),
    (Cow::Borrowed("lavender"), Cow::Borrowed("lavender")),
    (Cow::Borrowed("lavenderblush"), Cow::Borrowed("lavenderblush")),
    (Cow::Borrowed("lawngreen"), Cow::Borrowed("lawngreen")),
    (Cow::Borrowed("lemonchiffon"), Cow::Borrowed("lemonchiffon")),
    (Cow::Borrowed("lightblue"), Cow::Borrowed("lightblue")),
    (Cow::Borrowed("lightcoral"), Cow::Borrowed("lightcoral")),
    (Cow::Borrowed("lightcyan"), Cow::Borrowed("lightcyan")),
    (Cow::Borrowed("lightgoldenrodyellow"), Cow::Borrowed("lightgoldenrodyellow")),
    (Cow::Borrowed("lightgray"), Cow::Borrowed("lightgray")),
    (Cow::Borrowed("lightgreen"), Cow::Borrowed("lightgreen")),
    (Cow::Borrowed("lightgrey"), Cow::Borrowed("lightgrey")),
    (Cow::Borrowed("lightpink"), Cow::Borrowed("lightpink")),
    (Cow::Borrowed("lightsalmon"), Cow::Borrowed("lightsalmon")),
    (Cow::Borrowed("lightseagreen"), Cow::Borrowed("lightseagreen")),
    (Cow::Borrowed("lightskyblue"), Cow::Borrowed("lightskyblue")),
    (Cow::Borrowed("lightslategray"), Cow::Borrowed("lightslategray")),
    (Cow::Borrowed("lightslategrey"), Cow::Borrowed("lightslategrey")),
    (Cow::Borrowed("lightsteelblue"), Cow::Borrowed("lightsteelblue")),
    (Cow::Borrowed("lightyellow"), Cow::Borrowed("lightyellow")),
    (Cow::Borrowed("lime"), Cow::Borrowed("lime")),
    (Cow::Borrowed("limegreen"), Cow::Borrowed("limegreen")),
    (Cow::Borrowed("linen"), Cow::Borrowed("linen")),
    (Cow::Borrowed("magenta"), Cow::Borrowed("magenta")),
    (Cow::Borrowed("maroon"), Cow::Borrowed("maroon")),
    (Cow::Borrowed("mediumaquamarine"), Cow::Borrowed("mediumaquamarine")),
    (Cow::Borrowed("mediumblue"), Cow::Borrowed("mediumblue")),
    (Cow::Borrowed("mediumorchid"), Cow::Borrowed("mediumorchid")),
    (Cow::Borrowed("mediumpurple"), Cow::Borrowed("mediumpurple")),
    (Cow::Borrowed("mediumseagreen"), Cow::Borrowed("mediumseagreen")),
    (Cow::Borrowed("mediumslateblue"), Cow::Borrowed("mediumslateblue")),
    (Cow::Borrowed("mediumspringgreen"), Cow::Borrowed("mediumspringgreen")),
    (Cow::Borrowed("mediumturquoise"), Cow::Borrowed("mediumturquoise")),
    (Cow::Borrowed("mediumvioletred"), Cow::Borrowed("mediumvioletred")),
    (Cow::Borrowed("midnightblue"), Cow::Borrowed("midnightblue")),
    (Cow::Borrowed("mintcream"), Cow::Borrowed("mintcream")),
    (Cow::Borrowed("mistyrose"), Cow::Borrowed("mistyrose")),
    (Cow::Borrowed("moccasin"), Cow::Borrowed("moccasin")),
    (Cow::Borrowed("navajowhite"), Cow::Borrowed("navajowhite")),
    (Cow::Borrowed("navy"), Cow::Borrowed("navy")),
    (Cow::Borrowed("oldlace"), Cow::Borrowed("oldlace")),
    (Cow::Borrowed("olive"), Cow::Borrowed("olive")),
    (Cow::Borrowed("olivedrab"), Cow::Borrowed("olivedrab")),
    (Cow::Borrowed("orange"), Cow::Borrowed("orange")),
    (Cow::Borrowed("orangered"), Cow::Borrowed("orangered")),
    (Cow::Borrowed("orchid"), Cow::Borrowed("orchid")),
    (Cow::Borrowed("palegoldenrod"), Cow::Borrowed("palegoldenrod")),
    (Cow::Borrowed("palegreen"), Cow::Borrowed("palegreen")),
    (Cow::Borrowed("paleturquoise"), Cow::Borrowed("paleturquoise")),
    (Cow::Borrowed("palevioletred"), Cow::Borrowed("palevioletred")),
    (Cow::Borrowed("papayawhip"), Cow::Borrowed("papayawhip")),
    (Cow::Borrowed("peachpuff"), Cow::Borrowed("peachpuff")),
    (Cow::Borrowed("peru"), Cow::Borrowed("peru")),
    (Cow::Borrowed("pink"), Cow::Borrowed("pink")),
    (Cow::Borrowed("plum"), Cow::Borrowed("plum")),
    (Cow::Borrowed("powderblue"), Cow::Borrowed("powderblue")),
    (Cow::Borrowed("purple"), Cow::Borrowed("purple")),
    (Cow::Borrowed("rebeccapurple"), Cow::Borrowed("rebeccapurple")),
    (Cow::Borrowed("red"), Cow::Borrowed("red")),
    (Cow::Borrowed("rosybrown"), Cow::Borrowed("rosybrown")),
    (Cow::Borrowed("royalblue"), Cow::Borrowed("royalblue")),
    (Cow::Borrowed("saddlebrown"), Cow::Borrowed("saddlebrown")),
    (Cow::Borrowed("salmon"), Cow::Borrowed("salmon")),
    (Cow::Borrowed("sandybrown"), Cow::Borrowed("sandybrown")),
    (Cow::Borrowed("seagreen"), Cow::Borrowed("seagreen")),
    (Cow::Borrowed("seashell"), Cow::Borrowed("seashell")),
    (Cow::Borrowed("sienna"), Cow::Borrowed("sienna")),
    (Cow::Borrowed("silver"), Cow::Borrowed("silver")),
    (Cow::Borrowed("skyblue"), Cow::Borrowed("skyblue")),
    (Cow::Borrowed("slateblue"), Cow::Borrowed("slateblue")),
    (Cow::Borrowed("slategray"), Cow::Borrowed("slategray")),
    (Cow::Borrowed("slategrey"), Cow::Borrowed("slategrey")),
    (Cow::Borrowed("snow"), Cow::Borrowed("snow")),
    (Cow::Borrowed("springgreen"), Cow::Borrowed("springgreen")),
    (Cow::Borrowed("steelblue"), Cow::Borrowed("steelblue")),
    (Cow::Borrowed("tan"), Cow::Borrowed("tan")),
    (Cow::Borrowed("teal"), Cow::Borrowed("teal")),
    (Cow::Borrowed("thistle"), Cow::Borrowed("thistle")),
    (Cow::Borrowed("tomato"), Cow::Borrowed("tomato")),
    (Cow::Borrowed("turquoise"), Cow::Borrowed("turquoise")),
    (Cow::Borrowed("violet"), Cow::Borrowed("violet")),
    (Cow::Borrowed("wheat"), Cow::Borrowed("wheat")),
    (Cow::Borrowed("white"), Cow::Borrowed("white")),
    (Cow::Borrowed("whitesmoke"), Cow::Borrowed("whitesmoke")),
    (Cow::Borrowed("yellow"), Cow::Borrowed("yellow")),
    (Cow::Borrowed("yellowgreen"), Cow::Borrowed("yellowgreen")),
];

#[derive(Debug, Serialize, Deserialize, Deref)]
pub struct Palette {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[deref(ignore)]
    pub inherit: Vec<PathBuf>,

    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub colors: HashMap<Cow<'static, str>, Cow<'static, str>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PaletteMaybeFile {
    PaletteFile(PathBuf),
    Palette(Palette),
}

impl PaletteMaybeFile {
    pub fn deserialize_with<'de, D>(deserializer: D) -> Result<Palette, D::Error>
    where
        D: Deserializer<'de>,
    {
        let deserialized = Self::deserialize(deserializer)?;

        match deserialized {
            Self::PaletteFile(path) => load_config(path).map_err(serde::de::Error::custom),
            Self::Palette(colors) => Ok(colors),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Deref)]
pub struct ParsedColorConfig(
    #[serde(deserialize_with = "PaletteMaybeFile::deserialize_with")] pub Palette,
);

impl Default for Palette {
    fn default() -> Self {
        Self {
            inherit: vec![],
            colors: HashMap::from_iter(DEFAULT_COLORS),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config<'a> {
    pub prefix: Cow<'a, str>,
    pub suffix: Cow<'a, str>,
    pub palette: Cow<'a, str>,
    pub palettes: HashMap<Cow<'a, str>, ParsedColorConfig>,
}

impl Default for Config<'_> {
    fn default() -> Self {
        Self {
            prefix: "${".into(),
            suffix: "}".into(),
            palette: "default".into(),
            palettes: HashMap::from_iter([("default".into(), ParsedColorConfig::default())]),
        }
    }
}

pub fn load_config<'de, T: Deserialize<'de>>(name: impl AsRef<Path>) -> Result<T> {
    let dir = get_config_dir()?;
    let path = dir.join(name);

    let ext = path.extension().and_then(OsStr::to_str);
    let path = match ext {
        Some("toml") => path,
        _ => path.with_extension("toml"),
    };

    if !path.is_file() {
        return Err(Error::ConfigNotFile(path));
    }

    let data = std::fs::read_to_string(&path)?;

    let deserializer = toml::Deserializer::new(&data);
    let value = T::deserialize(deserializer)?;

    Ok(value)
}

pub fn get_config_dir() -> Result<PathBuf> {
    #[cfg(not(debug_assertions))]
    let dirs = directories::ProjectDirs::from("rs", "", APP_NAME) //
        .ok_or_else(|| Error::NoConfigPath)?;

    #[cfg(debug_assertions)]
    let dirs = std::env::current_dir()?;

    Ok(dirs)
}
