// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_IEToolbarButton struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_IEToolbarButton {

/// 
    #[serde(rename = "actionPath")]
    pub action_path: Option<String>,

/// 
    #[serde(rename = "buttonOrder")]
    pub button_order: Option<i32>,

/// 
    #[serde(rename = "caption")]
    pub caption: Option<String>,

/// 
    #[serde(rename = "hotIconPath")]
    pub hot_icon_path: Option<String>,

/// 
    #[serde(rename = "iconPath")]
    pub icon_path: Option<String>,

/// 
    #[serde(rename = "rsopID")]
    pub rsop_id: Option<String>,

/// 
    #[serde(rename = "rsopPrecedence")]
    pub rsop_precedence: Option<u32>,

/// 
    #[serde(rename = "showOnToolbarByDefault")]
    pub show_on_toolbar_by_default: Option<bool>,
}

impl RSOP_IEToolbarButton {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            action_path: None,
            button_order: None,
            caption: None,
            hot_icon_path: None,
            icon_path: None,
            rsop_id: None,
            rsop_precedence: None,
            show_on_toolbar_by_default: None,
        }
    }


    /// Sets the value of actionPath
    pub fn set_action_path(&mut self, value: String) {
        self.action_path = Some(value);
    }

    /// Gets the value of actionPath
    pub fn get_action_path(&self) -> Option<&String> {
        self.action_path.as_ref()
    }

    /// Sets the value of buttonOrder
    pub fn set_button_order(&mut self, value: i32) {
        self.button_order = Some(value);
    }

    /// Gets the value of buttonOrder
    pub fn get_button_order(&self) -> Option<&i32> {
        self.button_order.as_ref()
    }

    /// Sets the value of caption
    pub fn set_caption(&mut self, value: String) {
        self.caption = Some(value);
    }

    /// Gets the value of caption
    pub fn get_caption(&self) -> Option<&String> {
        self.caption.as_ref()
    }

    /// Sets the value of hotIconPath
    pub fn set_hot_icon_path(&mut self, value: String) {
        self.hot_icon_path = Some(value);
    }

    /// Gets the value of hotIconPath
    pub fn get_hot_icon_path(&self) -> Option<&String> {
        self.hot_icon_path.as_ref()
    }

    /// Sets the value of iconPath
    pub fn set_icon_path(&mut self, value: String) {
        self.icon_path = Some(value);
    }

    /// Gets the value of iconPath
    pub fn get_icon_path(&self) -> Option<&String> {
        self.icon_path.as_ref()
    }

    /// Sets the value of rsopID
    pub fn set_rsop_id(&mut self, value: String) {
        self.rsop_id = Some(value);
    }

    /// Gets the value of rsopID
    pub fn get_rsop_id(&self) -> Option<&String> {
        self.rsop_id.as_ref()
    }

    /// Sets the value of rsopPrecedence
    pub fn set_rsop_precedence(&mut self, value: u32) {
        self.rsop_precedence = Some(value);
    }

    /// Gets the value of rsopPrecedence
    pub fn get_rsop_precedence(&self) -> Option<&u32> {
        self.rsop_precedence.as_ref()
    }

    /// Sets the value of showOnToolbarByDefault
    pub fn set_show_on_toolbar_by_default(&mut self, value: bool) {
        self.show_on_toolbar_by_default = Some(value);
    }

    /// Gets the value of showOnToolbarByDefault
    pub fn get_show_on_toolbar_by_default(&self) -> Option<&bool> {
        self.show_on_toolbar_by_default.as_ref()
    }
}

