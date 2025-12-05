// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source UserStateConfigurationControls_OfflineFiles
//////////////////////////////////////////////

/// UserStateConfigurationControls_OfflineFiles enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum UserStateConfigurationControls_OfflineFiles {
    /// GroupPolicy
    #[serde(rename = "GroupPolicy")]
    GroupPolicy = 0,
    /// WMI
    #[serde(rename = "WMI")]
    WMI = 1,
}

impl Default for UserStateConfigurationControls_OfflineFiles {
    fn default() -> Self {
        Self::GroupPolicy
    }
}

