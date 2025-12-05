// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.ProtectionManagement
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MpRollback struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MpRollback {
}

impl MSFT_MpRollback {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `engine` -  (bool)
    /// * `platform` -  (bool)

    /// * `return_value` -  (u32)
    pub fn start(&self, engine: bool, platform: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Engine".to_string(), value: engine.into() });
        args.push(MethodParameter { name: "Platform".to_string(), value: platform.into() });
        self.invoke_method("Start", &args)

    }

}

