// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.msdtc
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DtcAdvancedSettingTask struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DtcAdvancedSettingTask {
}

impl MSFT_DtcAdvancedSettingTask {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `dtc_name` -  (String)
    /// * `name` -  (String)
    /// * `subkey` -  (String)

    /// * `cmdlet_output` -  (String)
    /// * `return_value` -  (u32)
    pub fn get(&self, dtc_name: &String, subkey: &String, name: &String, cmdlet_output: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DtcName".to_string(), value: dtc_name.into() });
        args.push(MethodParameter { name: "Subkey".to_string(), value: subkey.into() });
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });

        let result = self.invoke_method("Get", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `dtc_name` -  (String)
    /// * `name` -  (String)
    /// * `subkey` -  (String)
    /// * `type` -  (String)
    /// * `value` -  (String)

    /// * `return_value` -  (u32)
    pub fn set(&self, dtc_name: &String, subkey: &String, name: &String, value: &String, type: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DtcName".to_string(), value: dtc_name.into() });
        args.push(MethodParameter { name: "Subkey".to_string(), value: subkey.into() });
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "Value".to_string(), value: value.into() });
        args.push(MethodParameter { name: "Type".to_string(), value: type.into() });
        self.invoke_method("Set", &args)

    }

}

