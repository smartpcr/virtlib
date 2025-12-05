// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.AppBackgroundTask
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PS_BackgroundTask struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PS_BackgroundTask {
}

impl PS_BackgroundTask {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `task_id` -  (String[])

    /// * `return_value` -  (u32)
    pub fn start(&self, task_id: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TaskID".to_string(), value: task_id.into() });
        self.invoke_method("Start", &args)

    }


/// 

    /// * `task_id` -  (String[])

    /// * `return_value` -  (u32)
    pub fn unregister(&self, task_id: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TaskID".to_string(), value: task_id.into() });
        self.invoke_method("Unregister", &args)

    }


/// 

    /// * `include_resource_usage` -  (bool)
    /// * `package_family_name` -  (String)

    /// * `cmdlet_output` -  (MSFT_BackgroundTask[])
    /// * `return_value` -  (u32)
    pub fn get(&self, package_family_name: &String, include_resource_usage: bool, cmdlet_output: &mut Vec<MSFT_BackgroundTask>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PackageFamilyName".to_string(), value: package_family_name.into() });
        args.push(MethodParameter { name: "IncludeResourceUsage".to_string(), value: include_resource_usage.into() });

        let result = self.invoke_method("Get", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn enable(&self) -> Result<(), WmiError> {
        self.invoke_method("Enable", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn disable(&self) -> Result<(), WmiError> {
        self.invoke_method("Disable", &[])

    }


/// 

    /// * `mode` -  (String)

    /// * `return_value` -  (u32)
    pub fn set(&self, mode: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "mode".to_string(), value: mode.into() });
        self.invoke_method("Set", &args)

    }

}

