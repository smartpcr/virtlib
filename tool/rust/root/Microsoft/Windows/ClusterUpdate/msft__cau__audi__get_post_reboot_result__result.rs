// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ClusterUpdate
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_CAU_Audi_GetPostRebootResult_Result struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_CAU_Audi_GetPostRebootResult_Result {

/// 
    #[serde(rename = "HResult")]
    pub hresult: Option<u32>,

/// 
    #[serde(rename = "PostRebootHResult")]
    pub post_reboot_hresult: Option<u32>,
}

impl MSFT_CAU_Audi_GetPostRebootResult_Result {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            hresult: None,
            post_reboot_hresult: None,
        }
    }


    /// Sets the value of HResult
    pub fn set_hresult(&mut self, value: u32) {
        self.hresult = Some(value);
    }

    /// Gets the value of HResult
    pub fn get_hresult(&self) -> Option<&u32> {
        self.hresult.as_ref()
    }

    /// Sets the value of PostRebootHResult
    pub fn set_post_reboot_hresult(&mut self, value: u32) {
        self.post_reboot_hresult = Some(value);
    }

    /// Gets the value of PostRebootHResult
    pub fn get_post_reboot_hresult(&self) -> Option<&u32> {
        self.post_reboot_hresult.as_ref()
    }
}

