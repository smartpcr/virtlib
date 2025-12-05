// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DesiredStateConfiguration
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DSCLocalConfigurationManager struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DSCLocalConfigurationManager {
}

impl MSFT_DSCLocalConfigurationManager {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `configuration_data` -  (u8[])
    /// * `force` -  (bool)

    /// * `return_value` -  (u32)
    pub fn send_configuration(&self, configuration_data: &Vec<u8>, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ConfigurationData".to_string(), value: configuration_data.into() });
        args.push(MethodParameter { name: "force".to_string(), value: force.into() });
        self.invoke_method("SendConfiguration", &args)

    }


/// 

    /// * `configuration_data` -  (u8[])
    /// * `force` -  (bool)

    /// * `return_value` -  (u32)
    pub fn send_configuration_apply(&self, configuration_data: &Vec<u8>, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ConfigurationData".to_string(), value: configuration_data.into() });
        args.push(MethodParameter { name: "force".to_string(), value: force.into() });
        self.invoke_method("SendConfigurationApply", &args)

    }


/// 

    /// * `configuration_data` -  (u8[])

    /// * `configurations` -  (OMI_BaseResource[])
    /// * `return_value` -  (u32)
    pub fn get_configuration(&self, configuration_data: &Vec<u8>, configurations: &mut Vec<OMI_BaseResource>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "configurationData".to_string(), value: configuration_data.into() });

        let result = self.invoke_method("GetConfiguration", &args)?;
        let configurations = result.get_value("configurations")?;
        Ok(result.return_value)

    }


/// 

    /// * `configuration_data` -  (u8[])

    /// * `in_desired_state` -  (bool)
    /// * `resources_in_desired_state` -  (MSFT_ResourceInDesiredState[])
    /// * `resources_not_in_desired_state` -  (MSFT_ResourceNotInDesiredState[])
    /// * `return_value` -  (u32)
    pub fn test_configuration(&self, configuration_data: &Vec<u8>, in_desired_state: &mut bool, resources_in_desired_state: &mut Vec<MSFT_ResourceInDesiredState>, resources_not_in_desired_state: &mut Vec<MSFT_ResourceNotInDesiredState>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "configurationData".to_string(), value: configuration_data.into() });

        let result = self.invoke_method("TestConfiguration", &args)?;
        let in_desired_state = result.get_value("InDesiredState")?;
        let resources_in_desired_state = result.get_value("ResourcesInDesiredState")?;
        let resources_not_in_desired_state = result.get_value("ResourcesNotInDesiredState")?;
        Ok(result.return_value)

    }


/// 

    /// * `force` -  (bool)

    /// * `return_value` -  (u32)
    pub fn apply_configuration(&self, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "force".to_string(), value: force.into() });
        self.invoke_method("ApplyConfiguration", &args)

    }


/// 

    /// * `configuration_data` -  (u8[])
    /// * `force` -  (bool)

    /// * `return_value` -  (u32)
    pub fn send_meta_configuration_apply(&self, configuration_data: &Vec<u8>, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ConfigurationData".to_string(), value: configuration_data.into() });
        args.push(MethodParameter { name: "force".to_string(), value: force.into() });
        self.invoke_method("SendMetaConfigurationApply", &args)

    }


/// 

    /// * `meta_configuration` -  (MSFT_DSCMetaConfiguration)
    /// * `return_value` -  (u32)
    pub fn get_meta_configuration(&self, meta_configuration: &mut MSFT_DSCMetaConfiguration) -> Result<(), WmiError> {

        let result = self.invoke_method("GetMetaConfiguration", &[])?;
        let meta_configuration = result.get_value("MetaConfiguration")?;
        Ok(result.return_value)

    }


/// 

    /// * `configuration_number` -  (u8)

    /// * `return_value` -  (u32)
    pub fn roll_back(&self, configuration_number: u8) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "configurationNumber".to_string(), value: configuration_number.into() });
        self.invoke_method("RollBack", &args)

    }


/// 

    /// * `flags` -  (u32)

    /// * `return_value` -  (u32)
    pub fn perform_required_configuration_checks(&self, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("PerformRequiredConfigurationChecks", &args)

    }


/// 

    /// * `force` -  (bool)

    /// * `return_value` -  (u32)
    pub fn stop_configuration(&self, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "force".to_string(), value: force.into() });
        self.invoke_method("StopConfiguration", &args)

    }


/// 

    /// * `all` -  (bool)

    /// * `configuration_status` -  (MSFT_DSCConfigurationStatus[])
    /// * `return_value` -  (u32)
    pub fn get_configuration_status(&self, all: bool, configuration_status: &mut Vec<MSFT_DSCConfigurationStatus>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "All".to_string(), value: all.into() });

        let result = self.invoke_method("GetConfigurationStatus", &args)?;
        let configuration_status = result.get_value("configurationStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `configuration_data` -  (u8[])
    /// * `force` -  (bool)
    /// * `job_id` -  (String)

    /// * `return_value` -  (u32)
    pub fn send_configuration_apply_async(&self, configuration_data: &Vec<u8>, force: bool, job_id: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ConfigurationData".to_string(), value: configuration_data.into() });
        args.push(MethodParameter { name: "force".to_string(), value: force.into() });
        args.push(MethodParameter { name: "jobId".to_string(), value: job_id.into() });
        self.invoke_method("SendConfigurationApplyAsync", &args)

    }


/// 

    /// * `job_id` -  (String)
    /// * `resume_output_bookmark` -  (u8[])

    /// * `output` -  (MSFT_DSCConfigurationOutput[])
    /// * `return_value` -  (u32)
    pub fn get_configuration_result_output(&self, job_id: &String, resume_output_bookmark: &Vec<u8>, output: &mut Vec<MSFT_DSCConfigurationOutput>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "jobId".to_string(), value: job_id.into() });
        args.push(MethodParameter { name: "resumeOutputBookmark".to_string(), value: resume_output_bookmark.into() });

        let result = self.invoke_method("GetConfigurationResultOutput", &args)?;
        let output = result.get_value("output")?;
        Ok(result.return_value)

    }


/// 

    /// * `force` -  (bool)
    /// * `stage` -  (u32)

    /// * `return_value` -  (u32)
    pub fn remove_configuration(&self, stage: u32, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Stage".to_string(), value: stage.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        self.invoke_method("RemoveConfiguration", &args)

    }


/// 

    /// * `module_name` -  (String)
    /// * `resource_property` -  (u8[])
    /// * `resource_type` -  (String)

    /// * `configurations` -  (OMI_BaseResource)
    /// * `return_value` -  (u32)
    pub fn resource_get(&self, resource_type: &String, module_name: &String, resource_property: &Vec<u8>, configurations: &mut OMI_BaseResource) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ResourceType".to_string(), value: resource_type.into() });
        args.push(MethodParameter { name: "ModuleName".to_string(), value: module_name.into() });
        args.push(MethodParameter { name: "resourceProperty".to_string(), value: resource_property.into() });

        let result = self.invoke_method("ResourceGet", &args)?;
        let configurations = result.get_value("configurations")?;
        Ok(result.return_value)

    }


/// 

    /// * `module_name` -  (String)
    /// * `resource_property` -  (u8[])
    /// * `resource_type` -  (String)

    /// * `reboot_required` -  (bool)
    /// * `return_value` -  (u32)
    pub fn resource_set(&self, resource_type: &String, module_name: &String, resource_property: &Vec<u8>, reboot_required: &mut bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ResourceType".to_string(), value: resource_type.into() });
        args.push(MethodParameter { name: "ModuleName".to_string(), value: module_name.into() });
        args.push(MethodParameter { name: "resourceProperty".to_string(), value: resource_property.into() });

        let result = self.invoke_method("ResourceSet", &args)?;
        let reboot_required = result.get_value("RebootRequired")?;
        Ok(result.return_value)

    }


/// 

    /// * `module_name` -  (String)
    /// * `resource_property` -  (u8[])
    /// * `resource_type` -  (String)

    /// * `in_desired_state` -  (bool)
    /// * `return_value` -  (u32)
    pub fn resource_test(&self, resource_type: &String, module_name: &String, resource_property: &Vec<u8>, in_desired_state: &mut bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ResourceType".to_string(), value: resource_type.into() });
        args.push(MethodParameter { name: "ModuleName".to_string(), value: module_name.into() });
        args.push(MethodParameter { name: "resourceProperty".to_string(), value: resource_property.into() });

        let result = self.invoke_method("ResourceTest", &args)?;
        let in_desired_state = result.get_value("InDesiredState")?;
        Ok(result.return_value)

    }


/// 

    /// * `break_all` -  (bool)

    /// * `return_value` -  (u32)
    pub fn enable_debug_configuration(&self, break_all: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "BreakAll".to_string(), value: break_all.into() });
        self.invoke_method("EnableDebugConfiguration", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn disable_debug_configuration(&self) -> Result<(), WmiError> {
        self.invoke_method("DisableDebugConfiguration", &[])

    }

}

