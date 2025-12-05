// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ShortcutAction struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ShortcutAction {
    #[serde(flatten)]
    pub base: CIM_Action,

/// 
    #[serde(rename = "Arguments")]
    pub arguments: Option<String>,

/// 
    #[serde(rename = "HotKey")]
    pub hot_key: Option<u16>,

/// 
    #[serde(rename = "IconIndex")]
    pub icon_index: Option<String>,

/// 
    #[serde(rename = "Shortcut")]
    pub shortcut: Option<String>,

/// 
    #[serde(rename = "ShowCmd")]
    pub show_cmd: Option<u16>,

/// 
    #[serde(rename = "Target")]
    pub target: Option<String>,

/// 
    #[serde(rename = "WkDir")]
    pub wk_dir: Option<String>,
}

impl Win32_ShortcutAction {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Action::new(),
            arguments: None,
            hot_key: None,
            icon_index: None,
            shortcut: None,
            show_cmd: None,
            target: None,
            wk_dir: None,
        }
    }


    /// Sets the value of Arguments
    pub fn set_arguments(&mut self, value: String) {
        self.arguments = Some(value);
    }

    /// Gets the value of Arguments
    pub fn get_arguments(&self) -> Option<&String> {
        self.arguments.as_ref()
    }

    /// Sets the value of HotKey
    pub fn set_hot_key(&mut self, value: u16) {
        self.hot_key = Some(value);
    }

    /// Gets the value of HotKey
    pub fn get_hot_key(&self) -> Option<&u16> {
        self.hot_key.as_ref()
    }

    /// Sets the value of IconIndex
    pub fn set_icon_index(&mut self, value: String) {
        self.icon_index = Some(value);
    }

    /// Gets the value of IconIndex
    pub fn get_icon_index(&self) -> Option<&String> {
        self.icon_index.as_ref()
    }

    /// Sets the value of Shortcut
    pub fn set_shortcut(&mut self, value: String) {
        self.shortcut = Some(value);
    }

    /// Gets the value of Shortcut
    pub fn get_shortcut(&self) -> Option<&String> {
        self.shortcut.as_ref()
    }

    /// Sets the value of ShowCmd
    pub fn set_show_cmd(&mut self, value: u16) {
        self.show_cmd = Some(value);
    }

    /// Gets the value of ShowCmd
    pub fn get_show_cmd(&self) -> Option<&u16> {
        self.show_cmd.as_ref()
    }

    /// Sets the value of Target
    pub fn set_target(&mut self, value: String) {
        self.target = Some(value);
    }

    /// Gets the value of Target
    pub fn get_target(&self) -> Option<&String> {
        self.target.as_ref()
    }

    /// Sets the value of WkDir
    pub fn set_wk_dir(&mut self, value: String) {
        self.wk_dir = Some(value);
    }

    /// Gets the value of WkDir
    pub fn get_wk_dir(&self) -> Option<&String> {
        self.wk_dir.as_ref()
    }
}

