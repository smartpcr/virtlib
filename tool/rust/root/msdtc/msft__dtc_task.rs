// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.msdtc
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DtcTask struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DtcTask {
}

impl MSFT_DtcTask {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `dtc_name` -  (String)

    /// * `cmdlet_output` -  (DtcInstance[])
    /// * `return_value` -  (u32)
    pub fn get(&self, dtc_name: &String, cmdlet_output: &mut Vec<DtcInstance>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DtcName".to_string(), value: dtc_name.into() });

        let result = self.invoke_method("Get", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `log_path` -  (String)
    /// * `start_type` -  (String)

    /// * `return_value` -  (u32)
    pub fn install(&self, log_path: &String, start_type: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "LogPath".to_string(), value: log_path.into() });
        args.push(MethodParameter { name: "StartType".to_string(), value: start_type.into() });
        self.invoke_method("Install", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn uninstall(&self) -> Result<(), WmiError> {
        self.invoke_method("Uninstall", &[])

    }


/// 

    /// * `dtc_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn start(&self, dtc_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DtcName".to_string(), value: dtc_name.into() });
        self.invoke_method("Start", &args)

    }


/// 

    /// * `dtc_name` -  (String)
    /// * `recursive` -  (bool)

    /// * `return_value` -  (u32)
    pub fn stop(&self, dtc_name: &String, recursive: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DtcName".to_string(), value: dtc_name.into() });
        args.push(MethodParameter { name: "Recursive".to_string(), value: recursive.into() });
        self.invoke_method("Stop", &args)

    }

}

