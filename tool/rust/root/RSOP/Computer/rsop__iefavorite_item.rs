// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_IEFavoriteItem struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_IEFavoriteItem {
    #[serde(flatten)]
    pub base: RSOP_IEFavoriteOrLinkItem,

/// 
    #[serde(rename = "folderItem")]
    pub folder_item: Option<bool>,

/// 
    #[serde(rename = "parentPath")]
    pub parent_path: Option<String>,

/// 
    #[serde(rename = "rsopID")]
    pub rsop_id: Option<String>,

/// 
    #[serde(rename = "rsopPrecedence")]
    pub rsop_precedence: Option<u32>,

/// 
    #[serde(rename = "shortName")]
    pub short_name: Option<String>,
}

impl RSOP_IEFavoriteItem {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: RSOP_IEFavoriteOrLinkItem::new(),
            folder_item: None,
            parent_path: None,
            rsop_id: None,
            rsop_precedence: None,
            short_name: None,
        }
    }


    /// Sets the value of folderItem
    pub fn set_folder_item(&mut self, value: bool) {
        self.folder_item = Some(value);
    }

    /// Gets the value of folderItem
    pub fn get_folder_item(&self) -> Option<&bool> {
        self.folder_item.as_ref()
    }

    /// Sets the value of parentPath
    pub fn set_parent_path(&mut self, value: String) {
        self.parent_path = Some(value);
    }

    /// Gets the value of parentPath
    pub fn get_parent_path(&self) -> Option<&String> {
        self.parent_path.as_ref()
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

    /// Sets the value of shortName
    pub fn set_short_name(&mut self, value: String) {
        self.short_name = Some(value);
    }

    /// Gets the value of shortName
    pub fn get_short_name(&self) -> Option<&String> {
        self.short_name.as_ref()
    }
}

