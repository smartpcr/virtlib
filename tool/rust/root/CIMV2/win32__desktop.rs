// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_Desktop struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_Desktop {
    #[serde(flatten)]
    pub base: CIM_Setting,

/// 
    #[serde(rename = "BorderWidth")]
    pub border_width: Option<u32>,

/// 
    #[serde(rename = "CoolSwitch")]
    pub cool_switch: Option<bool>,

/// 
    #[serde(rename = "CursorBlinkRate")]
    pub cursor_blink_rate: Option<u32>,

/// 
    #[serde(rename = "DragFullWindows")]
    pub drag_full_windows: Option<bool>,

/// 
    #[serde(rename = "GridGranularity")]
    pub grid_granularity: Option<u32>,

/// 
    #[serde(rename = "IconSpacing")]
    pub icon_spacing: Option<u32>,

/// 
    #[serde(rename = "IconTitleFaceName")]
    pub icon_title_face_name: Option<String>,

/// 
    #[serde(rename = "IconTitleSize")]
    pub icon_title_size: Option<u32>,

/// 
    #[serde(rename = "IconTitleWrap")]
    pub icon_title_wrap: Option<bool>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Pattern")]
    pub pattern: Option<String>,

/// 
    #[serde(rename = "ScreenSaverActive")]
    pub screen_saver_active: Option<bool>,

/// 
    #[serde(rename = "ScreenSaverExecutable")]
    pub screen_saver_executable: Option<String>,

/// 
    #[serde(rename = "ScreenSaverSecure")]
    pub screen_saver_secure: Option<bool>,

/// 
    #[serde(rename = "ScreenSaverTimeout")]
    pub screen_saver_timeout: Option<u32>,

/// 
    #[serde(rename = "Wallpaper")]
    pub wallpaper: Option<String>,

/// 
    #[serde(rename = "WallpaperStretched")]
    pub wallpaper_stretched: Option<bool>,

/// 
    #[serde(rename = "WallpaperTiled")]
    pub wallpaper_tiled: Option<bool>,
}

impl Win32_Desktop {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Setting::new(),
            border_width: None,
            cool_switch: None,
            cursor_blink_rate: None,
            drag_full_windows: None,
            grid_granularity: None,
            icon_spacing: None,
            icon_title_face_name: None,
            icon_title_size: None,
            icon_title_wrap: None,
            name: None,
            pattern: None,
            screen_saver_active: None,
            screen_saver_executable: None,
            screen_saver_secure: None,
            screen_saver_timeout: None,
            wallpaper: None,
            wallpaper_stretched: None,
            wallpaper_tiled: None,
        }
    }


    /// Sets the value of BorderWidth
    pub fn set_border_width(&mut self, value: u32) {
        self.border_width = Some(value);
    }

    /// Gets the value of BorderWidth
    pub fn get_border_width(&self) -> Option<&u32> {
        self.border_width.as_ref()
    }

    /// Sets the value of CoolSwitch
    pub fn set_cool_switch(&mut self, value: bool) {
        self.cool_switch = Some(value);
    }

    /// Gets the value of CoolSwitch
    pub fn get_cool_switch(&self) -> Option<&bool> {
        self.cool_switch.as_ref()
    }

    /// Sets the value of CursorBlinkRate
    pub fn set_cursor_blink_rate(&mut self, value: u32) {
        self.cursor_blink_rate = Some(value);
    }

    /// Gets the value of CursorBlinkRate
    pub fn get_cursor_blink_rate(&self) -> Option<&u32> {
        self.cursor_blink_rate.as_ref()
    }

    /// Sets the value of DragFullWindows
    pub fn set_drag_full_windows(&mut self, value: bool) {
        self.drag_full_windows = Some(value);
    }

    /// Gets the value of DragFullWindows
    pub fn get_drag_full_windows(&self) -> Option<&bool> {
        self.drag_full_windows.as_ref()
    }

    /// Sets the value of GridGranularity
    pub fn set_grid_granularity(&mut self, value: u32) {
        self.grid_granularity = Some(value);
    }

    /// Gets the value of GridGranularity
    pub fn get_grid_granularity(&self) -> Option<&u32> {
        self.grid_granularity.as_ref()
    }

    /// Sets the value of IconSpacing
    pub fn set_icon_spacing(&mut self, value: u32) {
        self.icon_spacing = Some(value);
    }

    /// Gets the value of IconSpacing
    pub fn get_icon_spacing(&self) -> Option<&u32> {
        self.icon_spacing.as_ref()
    }

    /// Sets the value of IconTitleFaceName
    pub fn set_icon_title_face_name(&mut self, value: String) {
        self.icon_title_face_name = Some(value);
    }

    /// Gets the value of IconTitleFaceName
    pub fn get_icon_title_face_name(&self) -> Option<&String> {
        self.icon_title_face_name.as_ref()
    }

    /// Sets the value of IconTitleSize
    pub fn set_icon_title_size(&mut self, value: u32) {
        self.icon_title_size = Some(value);
    }

    /// Gets the value of IconTitleSize
    pub fn get_icon_title_size(&self) -> Option<&u32> {
        self.icon_title_size.as_ref()
    }

    /// Sets the value of IconTitleWrap
    pub fn set_icon_title_wrap(&mut self, value: bool) {
        self.icon_title_wrap = Some(value);
    }

    /// Gets the value of IconTitleWrap
    pub fn get_icon_title_wrap(&self) -> Option<&bool> {
        self.icon_title_wrap.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Pattern
    pub fn set_pattern(&mut self, value: String) {
        self.pattern = Some(value);
    }

    /// Gets the value of Pattern
    pub fn get_pattern(&self) -> Option<&String> {
        self.pattern.as_ref()
    }

    /// Sets the value of ScreenSaverActive
    pub fn set_screen_saver_active(&mut self, value: bool) {
        self.screen_saver_active = Some(value);
    }

    /// Gets the value of ScreenSaverActive
    pub fn get_screen_saver_active(&self) -> Option<&bool> {
        self.screen_saver_active.as_ref()
    }

    /// Sets the value of ScreenSaverExecutable
    pub fn set_screen_saver_executable(&mut self, value: String) {
        self.screen_saver_executable = Some(value);
    }

    /// Gets the value of ScreenSaverExecutable
    pub fn get_screen_saver_executable(&self) -> Option<&String> {
        self.screen_saver_executable.as_ref()
    }

    /// Sets the value of ScreenSaverSecure
    pub fn set_screen_saver_secure(&mut self, value: bool) {
        self.screen_saver_secure = Some(value);
    }

    /// Gets the value of ScreenSaverSecure
    pub fn get_screen_saver_secure(&self) -> Option<&bool> {
        self.screen_saver_secure.as_ref()
    }

    /// Sets the value of ScreenSaverTimeout
    pub fn set_screen_saver_timeout(&mut self, value: u32) {
        self.screen_saver_timeout = Some(value);
    }

    /// Gets the value of ScreenSaverTimeout
    pub fn get_screen_saver_timeout(&self) -> Option<&u32> {
        self.screen_saver_timeout.as_ref()
    }

    /// Sets the value of Wallpaper
    pub fn set_wallpaper(&mut self, value: String) {
        self.wallpaper = Some(value);
    }

    /// Gets the value of Wallpaper
    pub fn get_wallpaper(&self) -> Option<&String> {
        self.wallpaper.as_ref()
    }

    /// Sets the value of WallpaperStretched
    pub fn set_wallpaper_stretched(&mut self, value: bool) {
        self.wallpaper_stretched = Some(value);
    }

    /// Gets the value of WallpaperStretched
    pub fn get_wallpaper_stretched(&self) -> Option<&bool> {
        self.wallpaper_stretched.as_ref()
    }

    /// Sets the value of WallpaperTiled
    pub fn set_wallpaper_tiled(&mut self, value: bool) {
        self.wallpaper_tiled = Some(value);
    }

    /// Gets the value of WallpaperTiled
    pub fn get_wallpaper_tiled(&self) -> Option<&bool> {
        self.wallpaper_tiled.as_ref()
    }
}

