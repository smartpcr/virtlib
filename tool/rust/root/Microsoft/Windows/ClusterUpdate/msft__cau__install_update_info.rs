// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ClusterUpdate
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_CAU_InstallUpdateInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_CAU_InstallUpdateInfo {
    #[serde(flatten)]
    pub base: MSFT_CAU_DownloadUpdateInfo,

/// 
    #[serde(rename = "CommitRequired")]
    pub commit_required: Option<bool>,

/// 
    #[serde(rename = "LongRebootHint")]
    pub long_reboot_hint: Option<bool>,

/// 
    #[serde(rename = "RebootRequired")]
    pub reboot_required: Option<bool>,
}

impl MSFT_CAU_InstallUpdateInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_CAU_DownloadUpdateInfo::new(),
            commit_required: None,
            long_reboot_hint: None,
            reboot_required: None,
        }
    }


    /// Sets the value of CommitRequired
    pub fn set_commit_required(&mut self, value: bool) {
        self.commit_required = Some(value);
    }

    /// Gets the value of CommitRequired
    pub fn get_commit_required(&self) -> Option<&bool> {
        self.commit_required.as_ref()
    }

    /// Sets the value of LongRebootHint
    pub fn set_long_reboot_hint(&mut self, value: bool) {
        self.long_reboot_hint = Some(value);
    }

    /// Gets the value of LongRebootHint
    pub fn get_long_reboot_hint(&self) -> Option<&bool> {
        self.long_reboot_hint.as_ref()
    }

    /// Sets the value of RebootRequired
    pub fn set_reboot_required(&mut self, value: bool) {
        self.reboot_required = Some(value);
    }

    /// Gets the value of RebootRequired
    pub fn get_reboot_required(&self) -> Option<&bool> {
        self.reboot_required.as_ref()
    }
}

