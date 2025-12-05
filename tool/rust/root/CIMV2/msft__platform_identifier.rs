// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_PlatformIdentifier struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_PlatformIdentifier {
}

impl MSFT_PlatformIdentifier {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `name` -  (String)

    /// * `identifier` -  (String)
    /// * `return_value` -  (u32)
    pub fn get_platform_identifier(&self, name: &String, identifier: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });

        let result = self.invoke_method("GetPlatformIdentifier", &args)?;
        let identifier = result.get_value("Identifier")?;
        Ok(result.return_value)

    }

}

