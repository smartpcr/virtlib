// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.ProtectionManagement
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MpScan struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MpScan {
}

impl MSFT_MpScan {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `scan_path` -  (String)
    /// * `scan_type` -  (u8)

    /// * `return_value` -  (u32)
    pub fn start(&self, scan_type: u8, scan_path: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ScanType".to_string(), value: scan_type.into() });
        args.push(MethodParameter { name: "ScanPath".to_string(), value: scan_path.into() });
        self.invoke_method("Start", &args)

    }

}

