// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.ProtectionManagement
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MpSignature struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MpSignature {
}

impl MSFT_MpSignature {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `update_source` -  (u8)

    /// * `return_value` -  (u32)
    pub fn update(&self, update_source: u8) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "UpdateSource".to_string(), value: update_source.into() });
        self.invoke_method("Update", &args)

    }

}

