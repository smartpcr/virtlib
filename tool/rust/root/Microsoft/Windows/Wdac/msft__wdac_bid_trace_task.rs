// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Wdac
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_WdacBidTraceTask struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_WdacBidTraceTask {
}

impl MSFT_WdacBidTraceTask {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `include_all_applications` -  (bool)
    /// * `pass_thru` -  (bool)
    /// * `platform` -  (String)

    /// * `cmdlet_output` -  (MSFT_WdacBidTrace[])
    /// * `return_value` -  (u32)
    pub fn enable_by_all_app(&self, pass_thru: bool, include_all_applications: bool, platform: &String, cmdlet_output: &mut Vec<MSFT_WdacBidTrace>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "IncludeAllApplications".to_string(), value: include_all_applications.into() });
        args.push(MethodParameter { name: "Platform".to_string(), value: platform.into() });

        let result = self.invoke_method("EnableByAllApp", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `folder` -  (String)
    /// * `pass_thru` -  (bool)
    /// * `platform` -  (String)

    /// * `cmdlet_output` -  (MSFT_WdacBidTrace[])
    /// * `return_value` -  (u32)
    pub fn enable_by_folder(&self, pass_thru: bool, platform: &String, folder: &String, cmdlet_output: &mut Vec<MSFT_WdacBidTrace>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "Platform".to_string(), value: platform.into() });
        args.push(MethodParameter { name: "Folder".to_string(), value: folder.into() });

        let result = self.invoke_method("EnableByFolder", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `input_object` -  (MSFT_WdacBidTrace[])
    /// * `pass_thru` -  (bool)

    /// * `cmdlet_output` -  (MSFT_WdacBidTrace[])
    /// * `return_value` -  (u32)
    pub fn enable_by_input_object(&self, pass_thru: bool, input_object: &Vec<MSFT_WdacBidTrace>, cmdlet_output: &mut Vec<MSFT_WdacBidTrace>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });

        let result = self.invoke_method("EnableByInputObject", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `pass_thru` -  (bool)
    /// * `path` -  (String)
    /// * `platform` -  (String)
    /// * `process_id` -  (u32)

    /// * `cmdlet_output` -  (MSFT_WdacBidTrace[])
    /// * `return_value` -  (u32)
    pub fn enable_by_path(&self, pass_thru: bool, path: &String, platform: &String, process_id: u32, cmdlet_output: &mut Vec<MSFT_WdacBidTrace>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        args.push(MethodParameter { name: "Platform".to_string(), value: platform.into() });
        args.push(MethodParameter { name: "ProcessId".to_string(), value: process_id.into() });

        let result = self.invoke_method("EnableByPath", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `include_all_applications` -  (bool)
    /// * `pass_thru` -  (bool)
    /// * `platform` -  (String)

    /// * `cmdlet_output` -  (MSFT_WdacBidTrace[])
    /// * `return_value` -  (u32)
    pub fn disable_by_all_app(&self, pass_thru: bool, include_all_applications: bool, platform: &String, cmdlet_output: &mut Vec<MSFT_WdacBidTrace>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "IncludeAllApplications".to_string(), value: include_all_applications.into() });
        args.push(MethodParameter { name: "Platform".to_string(), value: platform.into() });

        let result = self.invoke_method("DisableByAllApp", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `folder` -  (String)
    /// * `pass_thru` -  (bool)
    /// * `platform` -  (String)

    /// * `cmdlet_output` -  (MSFT_WdacBidTrace[])
    /// * `return_value` -  (u32)
    pub fn disable_by_folder(&self, pass_thru: bool, folder: &String, platform: &String, cmdlet_output: &mut Vec<MSFT_WdacBidTrace>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "Folder".to_string(), value: folder.into() });
        args.push(MethodParameter { name: "Platform".to_string(), value: platform.into() });

        let result = self.invoke_method("DisableByFolder", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `input_object` -  (MSFT_WdacBidTrace[])
    /// * `pass_thru` -  (bool)

    /// * `cmdlet_output` -  (MSFT_WdacBidTrace[])
    /// * `return_value` -  (u32)
    pub fn disable_by_input_object(&self, pass_thru: bool, input_object: &Vec<MSFT_WdacBidTrace>, cmdlet_output: &mut Vec<MSFT_WdacBidTrace>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });

        let result = self.invoke_method("DisableByInputObject", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `pass_thru` -  (bool)
    /// * `path` -  (String)
    /// * `platform` -  (String)
    /// * `process_id` -  (u32)

    /// * `cmdlet_output` -  (MSFT_WdacBidTrace[])
    /// * `return_value` -  (u32)
    pub fn disable_by_path(&self, pass_thru: bool, path: &String, process_id: u32, platform: &String, cmdlet_output: &mut Vec<MSFT_WdacBidTrace>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        args.push(MethodParameter { name: "ProcessId".to_string(), value: process_id.into() });
        args.push(MethodParameter { name: "Platform".to_string(), value: platform.into() });

        let result = self.invoke_method("DisableByPath", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `include_all_applications` -  (bool)
    /// * `platform` -  (String)

    /// * `cmdlet_output` -  (MSFT_WdacBidTrace[])
    /// * `return_value` -  (u32)
    pub fn get_by_all_app(&self, platform: &String, include_all_applications: bool, cmdlet_output: &mut Vec<MSFT_WdacBidTrace>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Platform".to_string(), value: platform.into() });
        args.push(MethodParameter { name: "IncludeAllApplications".to_string(), value: include_all_applications.into() });

        let result = self.invoke_method("GetByAllApp", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `folder` -  (String)
    /// * `platform` -  (String)

    /// * `cmdlet_output` -  (MSFT_WdacBidTrace[])
    /// * `return_value` -  (u32)
    pub fn get_by_folder(&self, platform: &String, folder: &String, cmdlet_output: &mut Vec<MSFT_WdacBidTrace>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Platform".to_string(), value: platform.into() });
        args.push(MethodParameter { name: "Folder".to_string(), value: folder.into() });

        let result = self.invoke_method("GetByFolder", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `path` -  (String)
    /// * `platform` -  (String)
    /// * `process_id` -  (u32)

    /// * `cmdlet_output` -  (MSFT_WdacBidTrace[])
    /// * `return_value` -  (u32)
    pub fn get_by_path(&self, platform: &String, path: &String, process_id: u32, cmdlet_output: &mut Vec<MSFT_WdacBidTrace>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Platform".to_string(), value: platform.into() });
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        args.push(MethodParameter { name: "ProcessId".to_string(), value: process_id.into() });

        let result = self.invoke_method("GetByPath", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

