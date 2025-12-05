// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_IEFavoriteOrLinkItem struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_IEFavoriteOrLinkItem {

/// 
    #[serde(rename = "iconPath")]
    pub icon_path: Option<String>,

/// 
    #[serde(rename = "makeAvailableOffline")]
    pub make_available_offline: Option<bool>,

/// 
    #[serde(rename = "name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "order")]
    pub order: Option<i32>,

/// 
    #[serde(rename = "url")]
    pub url: Option<String>,
}

impl RSOP_IEFavoriteOrLinkItem {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            icon_path: None,
            make_available_offline: None,
            name: None,
            order: None,
            url: None,
        }
    }


    /// Sets the value of iconPath
    pub fn set_icon_path(&mut self, value: String) {
        self.icon_path = Some(value);
    }

    /// Gets the value of iconPath
    pub fn get_icon_path(&self) -> Option<&String> {
        self.icon_path.as_ref()
    }

    /// Sets the value of makeAvailableOffline
    pub fn set_make_available_offline(&mut self, value: bool) {
        self.make_available_offline = Some(value);
    }

    /// Gets the value of makeAvailableOffline
    pub fn get_make_available_offline(&self) -> Option<&bool> {
        self.make_available_offline.as_ref()
    }

    /// Sets the value of name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of order
    pub fn set_order(&mut self, value: i32) {
        self.order = Some(value);
    }

    /// Gets the value of order
    pub fn get_order(&self) -> Option<&i32> {
        self.order.as_ref()
    }

    /// Sets the value of url
    pub fn set_url(&mut self, value: String) {
        self.url = Some(value);
    }

    /// Gets the value of url
    pub fn get_url(&self) -> Option<&String> {
        self.url.as_ref()
    }
}

