use gpui2::rgba;

use crate::{PlayerTheme, SyntaxTheme, Theme, ThemeMetadata};

pub fn rose_pine() -> Theme {
    Theme {
        metadata: ThemeMetadata {
            name: "Rosé Pine".into(),
            is_light: false,
        },
        transparent: rgba(0x00000000).into(),
        mac_os_traffic_light_red: rgba(0xec695eff).into(),
        mac_os_traffic_light_yellow: rgba(0xf4bf4eff).into(),
        mac_os_traffic_light_green: rgba(0x61c553ff).into(),
        border: rgba(0x423f55ff).into(),
        border_variant: rgba(0x423f55ff).into(),
        border_focused: rgba(0x435255ff).into(),
        border_transparent: rgba(0x00000000).into(),
        elevated_surface: rgba(0x292738ff).into(),
        surface: rgba(0x1c1b2aff).into(),
        background: rgba(0x292738ff).into(),
        filled_element: rgba(0x292738ff).into(),
        filled_element_hover: rgba(0xffffff1e).into(),
        filled_element_active: rgba(0xffffff28).into(),
        filled_element_selected: rgba(0x2f3639ff).into(),
        filled_element_disabled: rgba(0x00000000).into(),
        ghost_element: rgba(0x00000000).into(),
        ghost_element_hover: rgba(0xffffff14).into(),
        ghost_element_active: rgba(0xffffff1e).into(),
        ghost_element_selected: rgba(0x2f3639ff).into(),
        ghost_element_disabled: rgba(0x00000000).into(),
        text: rgba(0xe0def4ff).into(),
        text_muted: rgba(0x74708dff).into(),
        text_placeholder: rgba(0xea6e92ff).into(),
        text_disabled: rgba(0x2f2b43ff).into(),
        text_accent: rgba(0x9bced6ff).into(),
        icon_muted: rgba(0x74708dff).into(),
        syntax: SyntaxTheme {
            comment: rgba(0x6e6a86ff).into(),
            string: rgba(0xf5c177ff).into(),
            function: rgba(0xebbcbaff).into(),
            keyword: rgba(0x30738fff).into(),
            highlights: Vec::new(),
        },
        status_bar: rgba(0x292738ff).into(),
        title_bar: rgba(0x292738ff).into(),
        toolbar: rgba(0x191724ff).into(),
        tab_bar: rgba(0x1c1b2aff).into(),
        editor: rgba(0x191724ff).into(),
        editor_subheader: rgba(0x1c1b2aff).into(),
        editor_active_line: rgba(0x1c1b2aff).into(),
        terminal: rgba(0x191724ff).into(),
        image_fallback_background: rgba(0x292738ff).into(),
        git_created: rgba(0x5cc1a3ff).into(),
        git_modified: rgba(0x9bced6ff).into(),
        git_deleted: rgba(0xea6e92ff).into(),
        git_conflict: rgba(0xf5c177ff).into(),
        git_ignored: rgba(0x2f2b43ff).into(),
        git_renamed: rgba(0xf5c177ff).into(),
        players: [
            PlayerTheme {
                cursor: rgba(0x9bced6ff).into(),
                selection: rgba(0x9bced63d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x5cc1a3ff).into(),
                selection: rgba(0x5cc1a33d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x9d7591ff).into(),
                selection: rgba(0x9d75913d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xc4a7e6ff).into(),
                selection: rgba(0xc4a7e63d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xc4a7e6ff).into(),
                selection: rgba(0xc4a7e63d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x31738fff).into(),
                selection: rgba(0x31738f3d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xea6e92ff).into(),
                selection: rgba(0xea6e923d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xf5c177ff).into(),
                selection: rgba(0xf5c1773d).into(),
            },
        ],
    }
}

pub fn rose_pine_dawn() -> Theme {
    Theme {
        metadata: ThemeMetadata {
            name: "Rosé Pine Dawn".into(),
            is_light: true,
        },
        transparent: rgba(0x00000000).into(),
        mac_os_traffic_light_red: rgba(0xec695eff).into(),
        mac_os_traffic_light_yellow: rgba(0xf4bf4eff).into(),
        mac_os_traffic_light_green: rgba(0x61c553ff).into(),
        border: rgba(0xdcd6d5ff).into(),
        border_variant: rgba(0xdcd6d5ff).into(),
        border_focused: rgba(0xc3d7dbff).into(),
        border_transparent: rgba(0x00000000).into(),
        elevated_surface: rgba(0xdcd8d8ff).into(),
        surface: rgba(0xfef9f2ff).into(),
        background: rgba(0xdcd8d8ff).into(),
        filled_element: rgba(0xdcd8d8ff).into(),
        filled_element_hover: rgba(0xffffff1e).into(),
        filled_element_active: rgba(0xffffff28).into(),
        filled_element_selected: rgba(0xdde9ebff).into(),
        filled_element_disabled: rgba(0x00000000).into(),
        ghost_element: rgba(0x00000000).into(),
        ghost_element_hover: rgba(0xffffff14).into(),
        ghost_element_active: rgba(0xffffff1e).into(),
        ghost_element_selected: rgba(0xdde9ebff).into(),
        ghost_element_disabled: rgba(0x00000000).into(),
        text: rgba(0x575279ff).into(),
        text_muted: rgba(0x706c8cff).into(),
        text_placeholder: rgba(0xb4647aff).into(),
        text_disabled: rgba(0x938fa3ff).into(),
        text_accent: rgba(0x57949fff).into(),
        icon_muted: rgba(0x706c8cff).into(),
        syntax: SyntaxTheme {
            comment: rgba(0x9893a5ff).into(),
            string: rgba(0xea9d34ff).into(),
            function: rgba(0xd7827dff).into(),
            keyword: rgba(0x276983ff).into(),
            highlights: Vec::new(),
        },
        status_bar: rgba(0xdcd8d8ff).into(),
        title_bar: rgba(0xdcd8d8ff).into(),
        toolbar: rgba(0xfaf4edff).into(),
        tab_bar: rgba(0xfef9f2ff).into(),
        editor: rgba(0xfaf4edff).into(),
        editor_subheader: rgba(0xfef9f2ff).into(),
        editor_active_line: rgba(0xfef9f2ff).into(),
        terminal: rgba(0xfaf4edff).into(),
        image_fallback_background: rgba(0xdcd8d8ff).into(),
        git_created: rgba(0x3daa8eff).into(),
        git_modified: rgba(0x57949fff).into(),
        git_deleted: rgba(0xb4647aff).into(),
        git_conflict: rgba(0xe99d35ff).into(),
        git_ignored: rgba(0x938fa3ff).into(),
        git_renamed: rgba(0xe99d35ff).into(),
        players: [
            PlayerTheme {
                cursor: rgba(0x57949fff).into(),
                selection: rgba(0x57949f3d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x3daa8eff).into(),
                selection: rgba(0x3daa8e3d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x7c697fff).into(),
                selection: rgba(0x7c697f3d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x9079a9ff).into(),
                selection: rgba(0x9079a93d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x9079a9ff).into(),
                selection: rgba(0x9079a93d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x296983ff).into(),
                selection: rgba(0x2969833d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xb4647aff).into(),
                selection: rgba(0xb4647a3d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xe99d35ff).into(),
                selection: rgba(0xe99d353d).into(),
            },
        ],
    }
}

pub fn rose_pine_moon() -> Theme {
    Theme {
        metadata: ThemeMetadata {
            name: "Rosé Pine Moon".into(),
            is_light: false,
        },
        transparent: rgba(0x00000000).into(),
        mac_os_traffic_light_red: rgba(0xec695eff).into(),
        mac_os_traffic_light_yellow: rgba(0xf4bf4eff).into(),
        mac_os_traffic_light_green: rgba(0x61c553ff).into(),
        border: rgba(0x504c68ff).into(),
        border_variant: rgba(0x504c68ff).into(),
        border_focused: rgba(0x435255ff).into(),
        border_transparent: rgba(0x00000000).into(),
        elevated_surface: rgba(0x38354eff).into(),
        surface: rgba(0x28253cff).into(),
        background: rgba(0x38354eff).into(),
        filled_element: rgba(0x38354eff).into(),
        filled_element_hover: rgba(0xffffff1e).into(),
        filled_element_active: rgba(0xffffff28).into(),
        filled_element_selected: rgba(0x2f3639ff).into(),
        filled_element_disabled: rgba(0x00000000).into(),
        ghost_element: rgba(0x00000000).into(),
        ghost_element_hover: rgba(0xffffff14).into(),
        ghost_element_active: rgba(0xffffff1e).into(),
        ghost_element_selected: rgba(0x2f3639ff).into(),
        ghost_element_disabled: rgba(0x00000000).into(),
        text: rgba(0xe0def4ff).into(),
        text_muted: rgba(0x85819eff).into(),
        text_placeholder: rgba(0xea6e92ff).into(),
        text_disabled: rgba(0x605d7aff).into(),
        text_accent: rgba(0x9bced6ff).into(),
        icon_muted: rgba(0x85819eff).into(),
        syntax: SyntaxTheme {
            comment: rgba(0x6e6a86ff).into(),
            string: rgba(0xf5c177ff).into(),
            function: rgba(0xea9a97ff).into(),
            keyword: rgba(0x3d8fb0ff).into(),
            highlights: Vec::new(),
        },
        status_bar: rgba(0x38354eff).into(),
        title_bar: rgba(0x38354eff).into(),
        toolbar: rgba(0x232136ff).into(),
        tab_bar: rgba(0x28253cff).into(),
        editor: rgba(0x232136ff).into(),
        editor_subheader: rgba(0x28253cff).into(),
        editor_active_line: rgba(0x28253cff).into(),
        terminal: rgba(0x232136ff).into(),
        image_fallback_background: rgba(0x38354eff).into(),
        git_created: rgba(0x5cc1a3ff).into(),
        git_modified: rgba(0x9bced6ff).into(),
        git_deleted: rgba(0xea6e92ff).into(),
        git_conflict: rgba(0xf5c177ff).into(),
        git_ignored: rgba(0x605d7aff).into(),
        git_renamed: rgba(0xf5c177ff).into(),
        players: [
            PlayerTheme {
                cursor: rgba(0x9bced6ff).into(),
                selection: rgba(0x9bced63d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x5cc1a3ff).into(),
                selection: rgba(0x5cc1a33d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xa683a0ff).into(),
                selection: rgba(0xa683a03d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xc4a7e6ff).into(),
                selection: rgba(0xc4a7e63d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xc4a7e6ff).into(),
                selection: rgba(0xc4a7e63d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x3e8fb0ff).into(),
                selection: rgba(0x3e8fb03d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xea6e92ff).into(),
                selection: rgba(0xea6e923d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xf5c177ff).into(),
                selection: rgba(0xf5c1773d).into(),
            },
        ],
    }
}