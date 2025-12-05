// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Wdac
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_OdbcPerfCounterTask struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_OdbcPerfCounterTask {
}

impl MSFT_OdbcPerfCounterTask {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `input_object` -  (MSFT_OdbcPerfCounter[])
    /// * `pass_thru` -  (bool)

    /// * `cmdlet_output` -  (MSFT_OdbcPerfCounter[])
    /// * `return_value` -  (u32)
    pub fn enable_by_input_object(&self, pass_thru: bool, input_object: &Vec<MSFT_OdbcPerfCounter>, cmdlet_output: &mut Vec<MSFT_OdbcPerfCounter>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });

        let result = self.invoke_method("EnableByInputObject", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `pass_thru` -  (bool)
    /// * `platform` -  (String)

    /// * `cmdlet_output` -  (MSFT_OdbcPerfCounter[])
    /// * `return_value` -  (u32)
    pub fn enable_by_platform(&self, pass_thru: bool, platform: &String, cmdlet_output: &mut Vec<MSFT_OdbcPerfCounter>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "Platform".to_string(), value: platform.into() });

        let result = self.invoke_method("EnableByPlatform", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `input_object` -  (MSFT_OdbcPerfCounter[])
    /// * `pass_thru` -  (bool)

    /// * `cmdlet_output` -  (MSFT_OdbcPerfCounter[])
    /// * `return_value` -  (u32)
    pub fn disable_by_input_object(&self, pass_thru: bool, input_object: &Vec<MSFT_OdbcPerfCounter>, cmdlet_output: &mut Vec<MSFT_OdbcPerfCounter>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });

        let result = self.invoke_method("DisableByInputObject", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `pass_thru` -  (bool)
    /// * `platform` -  (String)

    /// * `cmdlet_output` -  (MSFT_OdbcPerfCounter[])
    /// * `return_value` -  (u32)
    pub fn disable_by_platform(&self, pass_thru: bool, platform: &String, cmdlet_output: &mut Vec<MSFT_OdbcPerfCounter>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "Platform".to_string(), value: platform.into() });

        let result = self.invoke_method("DisableByPlatform", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `platform` -  (String)

    /// * `cmdlet_output` -  (MSFT_OdbcPerfCounter[])
    /// * `return_value` -  (u32)
    pub fn get(&self, platform: &String, cmdlet_output: &mut Vec<MSFT_OdbcPerfCounter>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Platform".to_string(), value: platform.into() });

        let result = self.invoke_method("Get", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

