// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_Group struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_Group {
    #[serde(flatten)]
    pub base: Win32_Account,
}

impl Win32_Group {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_Account::new(),
        }
    }


/// 

    /// * `name` -  (String)

    /// * `return_value` -  (u32)
    pub fn rename(&self, name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        self.invoke_method("Rename", &args)

    }

}

