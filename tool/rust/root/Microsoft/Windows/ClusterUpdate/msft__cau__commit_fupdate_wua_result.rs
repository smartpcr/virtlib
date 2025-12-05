// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ClusterUpdate
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_CAU_CommitFUpdateWuaResult struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_CAU_CommitFUpdateWuaResult {

/// 
    #[serde(rename = "WuaCommitHResult")]
    pub wua_commit_hresult: Option<u32>,
}

impl MSFT_CAU_CommitFUpdateWuaResult {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            wua_commit_hresult: None,
        }
    }


    /// Sets the value of WuaCommitHResult
    pub fn set_wua_commit_hresult(&mut self, value: u32) {
        self.wua_commit_hresult = Some(value);
    }

    /// Gets the value of WuaCommitHResult
    pub fn get_wua_commit_hresult(&self) -> Option<&u32> {
        self.wua_commit_hresult.as_ref()
    }
}

