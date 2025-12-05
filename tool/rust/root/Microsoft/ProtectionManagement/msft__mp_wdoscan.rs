// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.ProtectionManagement
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MpWDOScan struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MpWDOScan {
}

impl MSFT_MpWDOScan {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `return_value` -  (u32)
    pub fn start(&self) -> Result<(), WmiError> {
        self.invoke_method("Start", &[])

    }

}

