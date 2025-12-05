// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.msdtc
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DtcClusterTMMappingTask struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DtcClusterTMMappingTask {
}

impl MSFT_DtcClusterTMMappingTask {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `name` -  (String)

    /// * `cmdlet_output` -  (DtcClusterTMMapping[])
    /// * `return_value` -  (u32)
    pub fn get(&self, name: &String, cmdlet_output: &mut Vec<DtcClusterTMMapping>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });

        let result = self.invoke_method("Get", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `cluster_resource_name` -  (String)
    /// * `com_plus_app_id` -  (String)
    /// * `local` -  (bool)
    /// * `name` -  (String)

    /// * `return_value` -  (u32)
    pub fn add_by_com_plus_set(&self, name: &String, cluster_resource_name: &String, local: bool, com_plus_app_id: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "ClusterResourceName".to_string(), value: cluster_resource_name.into() });
        args.push(MethodParameter { name: "Local".to_string(), value: local.into() });
        args.push(MethodParameter { name: "ComPlusAppId".to_string(), value: com_plus_app_id.into() });
        self.invoke_method("AddByComPlusSet", &args)

    }


/// 

    /// * `cluster_resource_name` -  (String)
    /// * `executable_path` -  (String)
    /// * `local` -  (bool)
    /// * `name` -  (String)

    /// * `return_value` -  (u32)
    pub fn add_by_exe_set(&self, name: &String, cluster_resource_name: &String, local: bool, executable_path: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "ClusterResourceName".to_string(), value: cluster_resource_name.into() });
        args.push(MethodParameter { name: "Local".to_string(), value: local.into() });
        args.push(MethodParameter { name: "ExecutablePath".to_string(), value: executable_path.into() });
        self.invoke_method("AddByExeSet", &args)

    }


/// 

    /// * `cluster_resource_name` -  (String)
    /// * `local` -  (bool)
    /// * `name` -  (String)
    /// * `service` -  (String)

    /// * `return_value` -  (u32)
    pub fn add_by_service_set(&self, name: &String, cluster_resource_name: &String, local: bool, service: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "ClusterResourceName".to_string(), value: cluster_resource_name.into() });
        args.push(MethodParameter { name: "Local".to_string(), value: local.into() });
        args.push(MethodParameter { name: "Service".to_string(), value: service.into() });
        self.invoke_method("AddByServiceSet", &args)

    }


/// 

    /// * `all` -  (bool)

    /// * `return_value` -  (u32)
    pub fn remove_by_all_parameter_set(&self, all: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "All".to_string(), value: all.into() });
        self.invoke_method("RemoveByAllParameterSet", &args)

    }


/// 

    /// * `name` -  (String)

    /// * `return_value` -  (u32)
    pub fn remove_by_mapping_name_parameter_set(&self, name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        self.invoke_method("RemoveByMappingNameParameterSet", &args)

    }


/// 

    /// * `cluster_resource_name` -  (String)
    /// * `com_plus_app_id` -  (String)
    /// * `local` -  (bool)
    /// * `name` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_by_com_plus_set(&self, name: &String, com_plus_app_id: &String, cluster_resource_name: &String, local: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "ComPlusAppId".to_string(), value: com_plus_app_id.into() });
        args.push(MethodParameter { name: "ClusterResourceName".to_string(), value: cluster_resource_name.into() });
        args.push(MethodParameter { name: "Local".to_string(), value: local.into() });
        self.invoke_method("SetByComPlusSet", &args)

    }


/// 

    /// * `cluster_resource_name` -  (String)
    /// * `executable_path` -  (String)
    /// * `local` -  (bool)
    /// * `name` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_by_exe_set(&self, name: &String, executable_path: &String, cluster_resource_name: &String, local: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "ExecutablePath".to_string(), value: executable_path.into() });
        args.push(MethodParameter { name: "ClusterResourceName".to_string(), value: cluster_resource_name.into() });
        args.push(MethodParameter { name: "Local".to_string(), value: local.into() });
        self.invoke_method("SetByExeSet", &args)

    }


/// 

    /// * `cluster_resource_name` -  (String)
    /// * `local` -  (bool)
    /// * `name` -  (String)
    /// * `service` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_by_service_set(&self, name: &String, service: &String, cluster_resource_name: &String, local: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "Service".to_string(), value: service.into() });
        args.push(MethodParameter { name: "ClusterResourceName".to_string(), value: cluster_resource_name.into() });
        args.push(MethodParameter { name: "Local".to_string(), value: local.into() });
        self.invoke_method("SetByServiceSet", &args)

    }

}

