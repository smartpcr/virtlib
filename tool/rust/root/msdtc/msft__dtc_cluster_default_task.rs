// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.msdtc
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DtcClusterDefaultTask struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DtcClusterDefaultTask {
}

impl MSFT_DtcClusterDefaultTask {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `cmdlet_output` -  (String)
    /// * `return_value` -  (u32)
    pub fn get(&self, cmdlet_output: &mut String) -> Result<(), WmiError> {

        let result = self.invoke_method("Get", &[])?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `dtc_resource_name` -  (String)

    /// * `cmdlet_output` -  (String)
    /// * `return_value` -  (u32)
    pub fn set(&self, dtc_resource_name: &String, cmdlet_output: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DtcResourceName".to_string(), value: dtc_resource_name.into() });

        let result = self.invoke_method("Set", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

