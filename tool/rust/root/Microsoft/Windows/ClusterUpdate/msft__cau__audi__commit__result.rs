// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ClusterUpdate
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_CAU_Audi_Commit_Result struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_CAU_Audi_Commit_Result {

/// 
    #[serde(rename = "AudiCommitHResult")]
    pub audi_commit_hresult: Option<u32>,
}

impl MSFT_CAU_Audi_Commit_Result {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            audi_commit_hresult: None,
        }
    }


    /// Sets the value of AudiCommitHResult
    pub fn set_audi_commit_hresult(&mut self, value: u32) {
        self.audi_commit_hresult = Some(value);
    }

    /// Gets the value of AudiCommitHResult
    pub fn get_audi_commit_hresult(&self) -> Option<&u32> {
        self.audi_commit_hresult.as_ref()
    }
}

