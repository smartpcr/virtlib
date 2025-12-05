// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_IEFavoriteItemLink struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_IEFavoriteItemLink {

/// 
    #[serde(rename = "favoriteItem")]
    pub favorite_item: Option<RSOP_IEFavoriteItem>,

/// 
    #[serde(rename = "policySetting")]
    pub policy_setting: Option<RSOP_IEAKPolicySetting>,
}

impl RSOP_IEFavoriteItemLink {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            favorite_item: None,
            policy_setting: None,
        }
    }


    /// Sets the value of favoriteItem
    pub fn set_favorite_item(&mut self, value: RSOP_IEFavoriteItem) {
        self.favorite_item = Some(value);
    }

    /// Gets the value of favoriteItem
    pub fn get_favorite_item(&self) -> Option<&RSOP_IEFavoriteItem> {
        self.favorite_item.as_ref()
    }

    /// Sets the value of policySetting
    pub fn set_policy_setting(&mut self, value: RSOP_IEAKPolicySetting) {
        self.policy_setting = Some(value);
    }

    /// Gets the value of policySetting
    pub fn get_policy_setting(&self) -> Option<&RSOP_IEAKPolicySetting> {
        self.policy_setting.as_ref()
    }
}

