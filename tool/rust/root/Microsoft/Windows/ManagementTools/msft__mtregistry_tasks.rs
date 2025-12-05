// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ManagementTools
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MTRegistryTasks struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MTRegistryTasks {
}

impl MSFT_MTRegistryTasks {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `key_name` -  (String)
    /// * `options` -  (u8)
    /// * `value` -  (String)

    /// * `results` -  (MSFT_MTRegistryObject[])
    /// * `return_value` -  (u32)
    pub fn search(&self, value: &String, key_name: &String, options: u8, results: &mut Vec<MSFT_MTRegistryObject>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Value".to_string(), value: value.into() });
        args.push(MethodParameter { name: "KeyName".to_string(), value: key_name.into() });
        args.push(MethodParameter { name: "Options".to_string(), value: options.into() });

        let result = self.invoke_method("Search", &args)?;
        let results = result.get_value("Results")?;
        Ok(result.return_value)

    }

}

