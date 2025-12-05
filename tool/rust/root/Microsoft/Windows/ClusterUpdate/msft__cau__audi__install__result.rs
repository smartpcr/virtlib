// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ClusterUpdate
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_CAU_Audi_Install_Result struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_CAU_Audi_Install_Result {

/// 
    #[serde(rename = "AudiInstallHResult")]
    pub audi_install_hresult: Option<u32>,

/// 
    #[serde(rename = "RebootRequired")]
    pub reboot_required: Option<bool>,

/// 
    #[serde(rename = "UpdateId")]
    pub update_id: Option<String>,
}

impl MSFT_CAU_Audi_Install_Result {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            audi_install_hresult: None,
            reboot_required: None,
            update_id: None,
        }
    }


    /// Sets the value of AudiInstallHResult
    pub fn set_audi_install_hresult(&mut self, value: u32) {
        self.audi_install_hresult = Some(value);
    }

    /// Gets the value of AudiInstallHResult
    pub fn get_audi_install_hresult(&self) -> Option<&u32> {
        self.audi_install_hresult.as_ref()
    }

    /// Sets the value of RebootRequired
    pub fn set_reboot_required(&mut self, value: bool) {
        self.reboot_required = Some(value);
    }

    /// Gets the value of RebootRequired
    pub fn get_reboot_required(&self) -> Option<&bool> {
        self.reboot_required.as_ref()
    }

    /// Sets the value of UpdateId
    pub fn set_update_id(&mut self, value: String) {
        self.update_id = Some(value);
    }

    /// Gets the value of UpdateId
    pub fn get_update_id(&self) -> Option<&String> {
        self.update_id.as_ref()
    }
}

