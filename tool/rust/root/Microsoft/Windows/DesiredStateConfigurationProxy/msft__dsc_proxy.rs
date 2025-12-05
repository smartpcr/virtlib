// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DesiredStateConfigurationProxy
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DscProxy struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DscProxy {
}

impl MSFT_DscProxy {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// Get resource state.

    /// * `configuration_data` -  (u8[])

    /// * `return_value` -  (u32)
    /// * `state` -  (bool)
    pub fn get_resource_state(&self, configuration_data: &Vec<u8>, state: &mut bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ConfigurationData".to_string(), value: configuration_data.into() });

        let result = self.invoke_method("GetResourceState", &args)?;
        let state = result.get_value("state")?;
        Ok(result.return_value)

    }

}

