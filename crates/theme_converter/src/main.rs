use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use gpui2::{hsla, rgb, serde_json, AssetSource, Hsla, SharedString};
use log::LevelFilter;
use rust_embed::RustEmbed;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer};
use simplelog::SimpleLogger;
use theme2::{PlayerTheme, SyntaxTheme, ThemeMetadata};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The name of the theme to convert.
    theme: String,
}

fn main() -> Result<()> {
    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

    let args = Args::parse();

    let legacy_theme = load_theme(args.theme)?;

    let theme = convert_theme(legacy_theme)?;

    println!("{:?}", ThemePrinter(theme));

    Ok(())
}

#[derive(RustEmbed)]
#[folder = "../../assets"]
#[include = "fonts/**/*"]
#[include = "icons/**/*"]
#[include = "themes/**/*"]
#[include = "sounds/**/*"]
#[include = "*.md"]
#[exclude = "*.DS_Store"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Cow<[u8]>> {
        Self::get(path)
            .map(|f| f.data)
            .ok_or_else(|| anyhow!("could not find asset at path \"{}\"", path))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter(|p| p.starts_with(path))
            .map(SharedString::from)
            .collect())
    }
}

#[derive(Clone, Copy)]
pub struct PlayerThemeColors {
    pub cursor: Hsla,
    pub selection: Hsla,
}

impl PlayerThemeColors {
    pub fn new(theme: &LegacyTheme, ix: usize) -> Self {
        if ix < theme.players.len() {
            Self {
                cursor: theme.players[ix].cursor,
                selection: theme.players[ix].selection,
            }
        } else {
            Self {
                cursor: rgb::<Hsla>(0xff00ff),
                selection: rgb::<Hsla>(0xff00ff),
            }
        }
    }
}

impl From<PlayerThemeColors> for PlayerTheme {
    fn from(value: PlayerThemeColors) -> Self {
        Self {
            cursor: value.cursor,
            selection: value.selection,
        }
    }
}

#[derive(Clone, Copy)]
pub struct SyntaxColor {
    pub comment: Hsla,
    pub string: Hsla,
    pub function: Hsla,
    pub keyword: Hsla,
}

impl std::fmt::Debug for SyntaxColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SyntaxColor")
            .field("comment", &self.comment.to_rgb().to_hex())
            .field("string", &self.string.to_rgb().to_hex())
            .field("function", &self.function.to_rgb().to_hex())
            .field("keyword", &self.keyword.to_rgb().to_hex())
            .finish()
    }
}

impl SyntaxColor {
    pub fn new(theme: &LegacyTheme) -> Self {
        Self {
            comment: theme
                .syntax
                .get("comment")
                .cloned()
                .unwrap_or_else(|| rgb::<Hsla>(0xff00ff)),
            string: theme
                .syntax
                .get("string")
                .cloned()
                .unwrap_or_else(|| rgb::<Hsla>(0xff00ff)),
            function: theme
                .syntax
                .get("function")
                .cloned()
                .unwrap_or_else(|| rgb::<Hsla>(0xff00ff)),
            keyword: theme
                .syntax
                .get("keyword")
                .cloned()
                .unwrap_or_else(|| rgb::<Hsla>(0xff00ff)),
        }
    }
}

impl From<SyntaxColor> for SyntaxTheme {
    fn from(value: SyntaxColor) -> Self {
        Self {
            comment: value.comment,
            string: value.string,
            keyword: value.keyword,
            function: value.function,
            highlights: Vec::new(),
        }
    }
}

fn convert_theme(theme: LegacyTheme) -> Result<theme2::Theme> {
    let transparent = hsla(0.0, 0.0, 0.0, 0.0);

    let players: [PlayerTheme; 8] = [
        PlayerThemeColors::new(&theme, 0).into(),
        PlayerThemeColors::new(&theme, 1).into(),
        PlayerThemeColors::new(&theme, 2).into(),
        PlayerThemeColors::new(&theme, 3).into(),
        PlayerThemeColors::new(&theme, 4).into(),
        PlayerThemeColors::new(&theme, 5).into(),
        PlayerThemeColors::new(&theme, 6).into(),
        PlayerThemeColors::new(&theme, 7).into(),
    ];

    let theme = theme2::Theme {
        metadata: theme2::ThemeMetadata {
            name: theme.name.clone().into(),
            is_light: theme.is_light,
        },
        transparent,
        mac_os_traffic_light_red: rgb::<Hsla>(0xEC695E),
        mac_os_traffic_light_yellow: rgb::<Hsla>(0xF4BF4F),
        mac_os_traffic_light_green: rgb::<Hsla>(0x62C554),
        border: theme.lowest.base.default.border,
        border_variant: theme.lowest.variant.default.border,
        border_focused: theme.lowest.accent.default.border,
        border_transparent: transparent,
        elevated_surface: theme.lowest.base.default.background,
        surface: theme.middle.base.default.background,
        background: theme.lowest.base.default.background,
        filled_element: theme.lowest.base.default.background,
        filled_element_hover: hsla(0.0, 0.0, 100.0, 0.12),
        filled_element_active: hsla(0.0, 0.0, 100.0, 0.16),
        filled_element_selected: theme.lowest.accent.default.background,
        filled_element_disabled: transparent,
        ghost_element: transparent,
        ghost_element_hover: hsla(0.0, 0.0, 100.0, 0.08),
        ghost_element_active: hsla(0.0, 0.0, 100.0, 0.12),
        ghost_element_selected: theme.lowest.accent.default.background,
        ghost_element_disabled: transparent,
        text: theme.lowest.base.default.foreground,
        text_muted: theme.lowest.variant.default.foreground,
        /// TODO: map this to a real value
        text_placeholder: theme.lowest.negative.default.foreground,
        text_disabled: theme.lowest.base.disabled.foreground,
        text_accent: theme.lowest.accent.default.foreground,
        icon_muted: theme.lowest.variant.default.foreground,
        syntax: SyntaxColor::new(&theme).into(),

        status_bar: theme.lowest.base.default.background,
        title_bar: theme.lowest.base.default.background,
        toolbar: theme.highest.base.default.background,
        tab_bar: theme.middle.base.default.background,
        editor: theme.highest.base.default.background,
        editor_subheader: theme.middle.base.default.background,
        terminal: theme.highest.base.default.background,
        editor_active_line: theme.highest.on.default.background,
        image_fallback_background: theme.lowest.base.default.background,

        git_created: theme.lowest.positive.default.foreground,
        git_modified: theme.lowest.accent.default.foreground,
        git_deleted: theme.lowest.negative.default.foreground,
        git_conflict: theme.lowest.warning.default.foreground,
        git_ignored: theme.lowest.base.disabled.foreground,
        git_renamed: theme.lowest.warning.default.foreground,

        players,
    };

    Ok(theme)
}

#[derive(Deserialize)]
struct JsonTheme {
    pub base_theme: serde_json::Value,
}

/// Loads the [`Theme`] with the given name.
pub fn load_theme(name: String) -> Result<LegacyTheme> {
    let theme_contents = Assets::get(&format!("themes/{name}.json"))
        .with_context(|| format!("theme file not found: '{name}'"))?;

    let json_theme: JsonTheme = serde_json::from_str(std::str::from_utf8(&theme_contents.data)?)
        .context("failed to parse legacy theme")?;

    let legacy_theme: LegacyTheme = serde_json::from_value(json_theme.base_theme.clone())
        .context("failed to parse `base_theme`")?;

    Ok(legacy_theme)
}

#[derive(Deserialize, Clone, Default, Debug)]
pub struct LegacyTheme {
    pub name: String,
    pub is_light: bool,
    pub lowest: Layer,
    pub middle: Layer,
    pub highest: Layer,
    pub popover_shadow: Shadow,
    pub modal_shadow: Shadow,
    #[serde(deserialize_with = "deserialize_player_colors")]
    pub players: Vec<PlayerColors>,
    #[serde(deserialize_with = "deserialize_syntax_colors")]
    pub syntax: HashMap<String, Hsla>,
}

#[derive(Deserialize, Clone, Default, Debug)]
pub struct Layer {
    pub base: StyleSet,
    pub variant: StyleSet,
    pub on: StyleSet,
    pub accent: StyleSet,
    pub positive: StyleSet,
    pub warning: StyleSet,
    pub negative: StyleSet,
}

#[derive(Deserialize, Clone, Default, Debug)]
pub struct StyleSet {
    #[serde(rename = "default")]
    pub default: ContainerColors,
    pub hovered: ContainerColors,
    pub pressed: ContainerColors,
    pub active: ContainerColors,
    pub disabled: ContainerColors,
    pub inverted: ContainerColors,
}

#[derive(Deserialize, Clone, Default, Debug)]
pub struct ContainerColors {
    pub background: Hsla,
    pub foreground: Hsla,
    pub border: Hsla,
}

#[derive(Deserialize, Clone, Default, Debug)]
pub struct PlayerColors {
    pub selection: Hsla,
    pub cursor: Hsla,
}

#[derive(Deserialize, Clone, Default, Debug)]
pub struct Shadow {
    pub blur: u8,
    pub color: Hsla,
    pub offset: Vec<u8>,
}

fn deserialize_player_colors<'de, D>(deserializer: D) -> Result<Vec<PlayerColors>, D::Error>
where
    D: Deserializer<'de>,
{
    struct PlayerArrayVisitor;

    impl<'de> Visitor<'de> for PlayerArrayVisitor {
        type Value = Vec<PlayerColors>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an object with integer keys")
        }

        fn visit_map<A: serde::de::MapAccess<'de>>(
            self,
            mut map: A,
        ) -> Result<Self::Value, A::Error> {
            let mut players = Vec::with_capacity(8);
            while let Some((key, value)) = map.next_entry::<usize, PlayerColors>()? {
                if key < 8 {
                    players.push(value);
                } else {
                    return Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Unsigned(key as u64),
                        &"a key in range 0..7",
                    ));
                }
            }
            Ok(players)
        }
    }

    deserializer.deserialize_map(PlayerArrayVisitor)
}

fn deserialize_syntax_colors<'de, D>(deserializer: D) -> Result<HashMap<String, Hsla>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct ColorWrapper {
        color: Hsla,
    }

    struct SyntaxVisitor;

    impl<'de> Visitor<'de> for SyntaxVisitor {
        type Value = HashMap<String, Hsla>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map with keys and objects with a single color field as values")
        }

        fn visit_map<M>(self, mut map: M) -> Result<HashMap<String, Hsla>, M::Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            let mut result = HashMap::new();
            while let Some(key) = map.next_key()? {
                let wrapper: ColorWrapper = map.next_value()?; // Deserialize values as Hsla
                result.insert(key, wrapper.color);
            }
            Ok(result)
        }
    }
    deserializer.deserialize_map(SyntaxVisitor)
}

pub struct ThemePrinter(theme2::Theme);

impl std::fmt::Debug for ThemePrinter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Theme")
            .field("metadata", &ThemeMetadataPrinter(self.0.metadata.clone()))
            .field("transparent", &self.0.transparent.to_rgb().to_hex())
            .field(
                "mac_os_traffic_light_red",
                &self.0.mac_os_traffic_light_red.to_rgb().to_hex(),
            )
            .field(
                "mac_os_traffic_light_yellow",
                &self.0.mac_os_traffic_light_yellow.to_rgb().to_hex(),
            )
            .field(
                "mac_os_traffic_light_green",
                &self.0.mac_os_traffic_light_green.to_rgb().to_hex(),
            )
            .field("border", &self.0.border.to_rgb().to_hex())
            .field("border_variant", &self.0.border_variant.to_rgb().to_hex())
            .field("border_focused", &self.0.border_focused.to_rgb().to_hex())
            .field(
                "border_transparent",
                &self.0.border_transparent.to_rgb().to_hex(),
            )
            .field(
                "elevated_surface",
                &self.0.elevated_surface.to_rgb().to_hex(),
            )
            .field("surface", &self.0.surface.to_rgb().to_hex())
            .field("background", &self.0.background.to_rgb().to_hex())
            .field("filled_element", &self.0.filled_element.to_rgb().to_hex())
            .field(
                "filled_element_hover",
                &self.0.filled_element_hover.to_rgb().to_hex(),
            )
            .field(
                "filled_element_active",
                &self.0.filled_element_active.to_rgb().to_hex(),
            )
            .field(
                "filled_element_selected",
                &self.0.filled_element_selected.to_rgb().to_hex(),
            )
            .field(
                "filled_element_disabled",
                &self.0.filled_element_disabled.to_rgb().to_hex(),
            )
            .field("ghost_element", &self.0.ghost_element.to_rgb().to_hex())
            .field(
                "ghost_element_hover",
                &self.0.ghost_element_hover.to_rgb().to_hex(),
            )
            .field(
                "ghost_element_active",
                &self.0.ghost_element_active.to_rgb().to_hex(),
            )
            .field(
                "ghost_element_selected",
                &self.0.ghost_element_selected.to_rgb().to_hex(),
            )
            .field(
                "ghost_element_disabled",
                &self.0.ghost_element_disabled.to_rgb().to_hex(),
            )
            .field("text", &self.0.text.to_rgb().to_hex())
            .field("text_muted", &self.0.text_muted.to_rgb().to_hex())
            .field(
                "text_placeholder",
                &self.0.text_placeholder.to_rgb().to_hex(),
            )
            .field("text_disabled", &self.0.text_disabled.to_rgb().to_hex())
            .field("text_accent", &self.0.text_accent.to_rgb().to_hex())
            .field("icon_muted", &self.0.icon_muted.to_rgb().to_hex())
            .field("syntax", &SyntaxThemePrinter(self.0.syntax.clone()))
            .field("status_bar", &self.0.status_bar.to_rgb().to_hex())
            .field("title_bar", &self.0.title_bar.to_rgb().to_hex())
            .field("toolbar", &self.0.toolbar.to_rgb().to_hex())
            .field("tab_bar", &self.0.tab_bar.to_rgb().to_hex())
            .field("editor", &self.0.editor.to_rgb().to_hex())
            .field(
                "editor_subheader",
                &self.0.editor_subheader.to_rgb().to_hex(),
            )
            .field(
                "editor_active_line",
                &self.0.editor_active_line.to_rgb().to_hex(),
            )
            .field("terminal", &self.0.terminal.to_rgb().to_hex())
            .field(
                "image_fallback_background",
                &self.0.image_fallback_background.to_rgb().to_hex(),
            )
            .field("git_created", &self.0.git_created.to_rgb().to_hex())
            .field("git_modified", &self.0.git_modified.to_rgb().to_hex())
            .field("git_deleted", &self.0.git_deleted.to_rgb().to_hex())
            .field("git_conflict", &self.0.git_conflict.to_rgb().to_hex())
            .field("git_ignored", &self.0.git_ignored.to_rgb().to_hex())
            .field("git_renamed", &self.0.git_renamed.to_rgb().to_hex())
            .field("players", &self.0.players.map(PlayerThemePrinter))
            .finish()
    }
}

pub struct ThemeMetadataPrinter(ThemeMetadata);

impl std::fmt::Debug for ThemeMetadataPrinter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ThemeMetadata")
            .field("name", &self.0.name)
            .field("is_light", &self.0.is_light)
            .finish()
    }
}

pub struct SyntaxThemePrinter(SyntaxTheme);

impl std::fmt::Debug for SyntaxThemePrinter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SyntaxTheme")
            .field("comment", &self.0.comment.to_rgb().to_hex())
            .field("string", &self.0.string.to_rgb().to_hex())
            .field("function", &self.0.function.to_rgb().to_hex())
            .field("keyword", &self.0.keyword.to_rgb().to_hex())
            .field("highlights", &self.0.highlights)
            .finish()
    }
}

pub struct PlayerThemePrinter(PlayerTheme);

impl std::fmt::Debug for PlayerThemePrinter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PlayerTheme")
            .field("cursor", &self.0.cursor.to_rgb().to_hex())
            .field("selection", &self.0.selection.to_rgb().to_hex())
            .finish()
    }
}