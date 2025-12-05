// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Wdac
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_OdbcDriverTask struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_OdbcDriverTask {
}

impl MSFT_OdbcDriverTask {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `name` -  (String)
    /// * `platform` -  (String)

    /// * `cmdlet_output` -  (MSFT_OdbcDriver[])
    /// * `return_value` -  (u32)
    pub fn get(&self, name: &String, platform: &String, cmdlet_output: &mut Vec<MSFT_OdbcDriver>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "Platform".to_string(), value: platform.into() });

        let result = self.invoke_method("Get", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `input_object` -  (MSFT_OdbcDriver[])
    /// * `pass_thru` -  (bool)
    /// * `remove_property_value` -  (String[])
    /// * `set_property_value` -  (String[])

    /// * `cmdlet_output` -  (MSFT_OdbcDriver[])
    /// * `return_value` -  (u32)
    pub fn set_by_input_object(&self, pass_thru: bool, set_property_value: &Vec<String>, remove_property_value: &Vec<String>, input_object: &Vec<MSFT_OdbcDriver>, cmdlet_output: &mut Vec<MSFT_OdbcDriver>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "SetPropertyValue".to_string(), value: set_property_value.into() });
        args.push(MethodParameter { name: "RemovePropertyValue".to_string(), value: remove_property_value.into() });
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });

        let result = self.invoke_method("SetByInputObject", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `name` -  (String)
    /// * `pass_thru` -  (bool)
    /// * `platform` -  (String)
    /// * `remove_property_value` -  (String[])
    /// * `set_property_value` -  (String[])

    /// * `cmdlet_output` -  (MSFT_OdbcDriver[])
    /// * `return_value` -  (u32)
    pub fn set_by_name(&self, pass_thru: bool, set_property_value: &Vec<String>, remove_property_value: &Vec<String>, name: &String, platform: &String, cmdlet_output: &mut Vec<MSFT_OdbcDriver>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "SetPropertyValue".to_string(), value: set_property_value.into() });
        args.push(MethodParameter { name: "RemovePropertyValue".to_string(), value: remove_property_value.into() });
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "Platform".to_string(), value: platform.into() });

        let result = self.invoke_method("SetByName", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

