use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

static mut CONFIG_DIR: Option<PathBuf> = None;

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

pub type Palette<'a> = HashMap<Cow<'a, str>, Cow<'a, str>>;
pub type Palettes<'a> = HashMap<Cow<'a, str>, Palette<'a>>;
pub type PalettesBase<'a> = HashMap<Cow<'a, str>, PaletteBase<'a>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaletteBase<'a> {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inherits: Vec<Cow<'a, str>>,

    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub colors: Palette<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PaletteOrFile<'a> {
    File(PathBuf),
    Palette(PaletteBase<'a>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AutoLoad {
    All(bool),
    Specific(Vec<PathBuf>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigBase<'a> {
    pub prefix: Cow<'a, str>,
    pub suffix: Cow<'a, str>,
    pub palette: Cow<'a, str>,
    pub autoload: AutoLoad,
    pub palettes: HashMap<Cow<'a, str>, PaletteOrFile<'a>>,
}

#[derive(Debug)]
pub struct Config<'a> {
    pub prefix: Cow<'a, str>,
    pub suffix: Cow<'a, str>,
    pub palette: Cow<'a, str>,
    pub palettes: Palettes<'a>,
}

impl AutoLoad {
    pub fn parse<'a>(self) -> Result<PalettesBase<'a>> {
        match self {
            Self::All(false) => Ok(HashMap::new()),
            Self::All(true) => {
                let cur = get_config_dir()?;
                let paths = WalkDir::new(cur)
                    .max_depth(1)
                    .into_iter()
                    .filter_map(Result::ok)
                    .map(|e| e.into_path())
                    .collect::<Vec<PathBuf>>();

                Self::Specific(paths).parse()
            }
            Self::Specific(paths) => paths
                .into_iter()
                .filter(|p| p.is_file())
                .filter(|p| p.extension() == Some(OsStr::new("toml")))
                .filter(|p| p.file_name() != Some(OsStr::new("config.toml")))
                .filter(|p| p.file_name().is_some())
                .map(|p| {
                    (
                        p.with_extension("")
                            .file_name()
                            .unwrap()
                            .to_string_lossy()
                            .to_string()
                            .into(),
                        p,
                    )
                })
                .map(|(name, p)| Ok((name, load_config::<PaletteBase>(p)?)))
                .collect::<Result<PalettesBase<'a>>>(),
        }
    }
}

impl<'a> PaletteBase<'a> {
    pub fn all_inherits(
        &self,
        name: Cow<'a, str>,
        palettes: &PalettesBase<'a>,
    ) -> Result<Vec<Cow<'a, str>>> {
        let mut inherits = self.inherits.clone();

        for inherit in &self.inherits {
            let palette = palettes
                .get(inherit)
                .ok_or_else(|| Error::NoInherit(inherit.to_string(), name.to_string()))?;

            let sub_inherits = palette.all_inherits(inherit.clone(), palettes)?;

            inherits.extend(sub_inherits);
        }

        inherits.dedup();

        Ok(inherits)
    }

    pub fn parse(mut self, name: Cow<'a, str>, palettes: &PalettesBase<'a>) -> Result<Palette<'a>> {
        let inherits = self.all_inherits(name, palettes)?;

        for inherit in inherits {
            let palette = palettes
                .get(&inherit)
                .ok_or_else(|| Error::NoPalette(inherit.to_string()))?;

            for (k, v) in palette.colors.clone() {
                self.colors.entry(k).or_insert(v);
            }
        }

        Ok(self.colors)
    }
}

impl<'a> ConfigBase<'a> {
    pub fn parse(self) -> Result<Config<'a>> {
        let Self {
            prefix,
            suffix,
            palette,
            autoload,
            palettes,
        } = self;

        let autoload = autoload.parse()?;

        let mut palettes_base = palettes
            .clone()
            .into_iter()
            .map(|(k, v)| Ok((k, v.parse()?)))
            .collect::<Result<PalettesBase>>()?;

        for (k, v) in autoload {
            palettes_base.entry(k).or_insert(v);
        }

        let palettes = palettes_base
            .clone()
            .into_iter()
            .map(|(k, v)| Ok((k.clone(), v.parse(k, &palettes_base)?)))
            .collect::<Result<Palettes<'a>>>()?;

        Ok(Config {
            prefix,
            suffix,
            palette,
            palettes,
        })
    }
}

impl<'a> PaletteOrFile<'a> {
    pub fn parse(self) -> Result<PaletteBase<'a>> {
        match self {
            Self::File(path) => load_config::<PaletteBase>(path),
            Self::Palette(colors) => Ok(colors),
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
        return Err(Error::NotFile(path));
    }

    let data = std::fs::read_to_string(&path)?;

    let deserializer = toml::Deserializer::new(&data);
    let value = T::deserialize(deserializer)?;

    Ok(value)
}

pub fn override_config_dir(path: impl AsRef<Path>) {
    unsafe {
        #[allow(static_mut_refs)]
        if CONFIG_DIR.is_some() {
            unreachable!(
                "Config directory is already set, should only bet set once at start of application"
            )
        }

        CONFIG_DIR = Some(path.as_ref().to_path_buf());
    }
}

pub fn get_config_dir() -> Result<PathBuf> {
    #[allow(static_mut_refs)]
    unsafe {
        if let Some(path) = &CONFIG_DIR {
            return Ok(path.clone());
        }
    }

    #[cfg(not(debug_assertions))]
    let dirs = directories::ProjectDirs::from("rs", "", APP_NAME) //
        .ok_or_else(|| Error::NoConfigPath)?;

    #[cfg(debug_assertions)]
    let dirs = std::env::current_dir()?;

    Ok(dirs)
}
