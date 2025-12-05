// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ClusterUpdate
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_CAU_Legacy_Upgrade_Install_Result struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_CAU_Legacy_Upgrade_Install_Result {

/// 
    #[serde(rename = "LaunchInstallerHResult")]
    pub launch_installer_hresult: Option<u32>,

/// 
    #[serde(rename = "ReturnCode")]
    pub return_code: Option<i32>,
}

impl MSFT_CAU_Legacy_Upgrade_Install_Result {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            launch_installer_hresult: None,
            return_code: None,
        }
    }


    /// Sets the value of LaunchInstallerHResult
    pub fn set_launch_installer_hresult(&mut self, value: u32) {
        self.launch_installer_hresult = Some(value);
    }

    /// Gets the value of LaunchInstallerHResult
    pub fn get_launch_installer_hresult(&self) -> Option<&u32> {
        self.launch_installer_hresult.as_ref()
    }

    /// Sets the value of ReturnCode
    pub fn set_return_code(&mut self, value: i32) {
        self.return_code = Some(value);
    }

    /// Gets the value of ReturnCode
    pub fn get_return_code(&self) -> Option<&i32> {
        self.return_code.as_ref()
    }
}

