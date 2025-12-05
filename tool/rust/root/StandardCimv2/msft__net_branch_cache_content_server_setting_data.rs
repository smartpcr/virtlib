// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetBranchCacheContentServerSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetBranchCacheContentServerSettingData {
    #[serde(flatten)]
    pub base: MSFT_NetBranchCacheSettingData,

/// 
    #[serde(rename = "ContentServerIsEnabled")]
    pub content_server_is_enabled: Option<bool>,
}

impl MSFT_NetBranchCacheContentServerSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetBranchCacheSettingData::new(),
            content_server_is_enabled: None,
        }
    }


    /// Sets the value of ContentServerIsEnabled
    pub fn set_content_server_is_enabled(&mut self, value: bool) {
        self.content_server_is_enabled = Some(value);
    }

    /// Gets the value of ContentServerIsEnabled
    pub fn get_content_server_is_enabled(&self) -> Option<&bool> {
        self.content_server_is_enabled.as_ref()
    }
}

