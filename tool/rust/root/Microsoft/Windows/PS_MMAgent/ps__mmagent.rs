// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.PS_MMAgent
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PS_MMAgent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PS_MMAgent {
}

impl PS_MMAgent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `application_launch_prefetching` -  (bool)
    /// * `application_pre_launch` -  (bool)
    /// * `memory_compression` -  (bool)
    /// * `operation_api` -  (bool)
    /// * `page_combining` -  (bool)

    /// * `return_value` -  (u32)
    pub fn enable(&self, application_launch_prefetching: bool, operation_api: bool, page_combining: bool, application_pre_launch: bool, memory_compression: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ApplicationLaunchPrefetching".to_string(), value: application_launch_prefetching.into() });
        args.push(MethodParameter { name: "OperationAPI".to_string(), value: operation_api.into() });
        args.push(MethodParameter { name: "PageCombining".to_string(), value: page_combining.into() });
        args.push(MethodParameter { name: "ApplicationPreLaunch".to_string(), value: application_pre_launch.into() });
        args.push(MethodParameter { name: "MemoryCompression".to_string(), value: memory_compression.into() });
        self.invoke_method("Enable", &args)

    }


/// 

    /// * `application_launch_prefetching` -  (bool)
    /// * `application_pre_launch` -  (bool)
    /// * `memory_compression` -  (bool)
    /// * `operation_api` -  (bool)
    /// * `page_combining` -  (bool)

    /// * `return_value` -  (u32)
    pub fn disable(&self, application_launch_prefetching: bool, operation_api: bool, page_combining: bool, application_pre_launch: bool, memory_compression: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ApplicationLaunchPrefetching".to_string(), value: application_launch_prefetching.into() });
        args.push(MethodParameter { name: "OperationAPI".to_string(), value: operation_api.into() });
        args.push(MethodParameter { name: "PageCombining".to_string(), value: page_combining.into() });
        args.push(MethodParameter { name: "ApplicationPreLaunch".to_string(), value: application_pre_launch.into() });
        args.push(MethodParameter { name: "MemoryCompression".to_string(), value: memory_compression.into() });
        self.invoke_method("Disable", &args)

    }


/// 

    /// * `max_operation_apifiles` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set(&self, max_operation_apifiles: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "MaxOperationAPIFiles".to_string(), value: max_operation_apifiles.into() });
        self.invoke_method("Set", &args)

    }


/// 

    /// * `cmdlet_output` -  (MMAgentComponents)
    /// * `return_value` -  (u32)
    pub fn get(&self, cmdlet_output: &mut MMAgentComponents) -> Result<(), WmiError> {

        let result = self.invoke_method("Get", &[])?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `disable_debug_mode` -  (bool)
    /// * `package_full_name` -  (String)
    /// * `package_relative_app_id` -  (String)

    /// * `return_value` -  (u32)
    pub fn debug(&self, package_full_name: &String, disable_debug_mode: bool, package_relative_app_id: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PackageFullName".to_string(), value: package_full_name.into() });
        args.push(MethodParameter { name: "DisableDebugMode".to_string(), value: disable_debug_mode.into() });
        args.push(MethodParameter { name: "PackageRelativeAppId".to_string(), value: package_relative_app_id.into() });
        self.invoke_method("Debug", &args)

    }

}

