// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_UpdateTemplate struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_UpdateTemplate {

/// 
    #[serde(rename = "CoordinatorOptions")]
    pub coordinator_options: Option<String>,

/// 
    #[serde(rename = "DrainPluginOptions")]
    pub drain_plugin_options: Option<String>,

/// 
    #[serde(rename = "DrainPlugins")]
    pub drain_plugins: Vec<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Reason")]
    pub reason: Option<String>,

/// 
    #[serde(rename = "UpdaterPluginOptions")]
    pub updater_plugin_options: Option<String>,

/// 
    #[serde(rename = "UpdaterPlugins")]
    pub updater_plugins: Vec<String>,

/// 
    #[serde(rename = "ValidatorPluginOptions")]
    pub validator_plugin_options: Option<String>,

/// 
    #[serde(rename = "ValidatorPlugins")]
    pub validator_plugins: Vec<String>,
}

impl MSCluster_UpdateTemplate {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            coordinator_options: None,
            drain_plugin_options: None,
            drain_plugins: Vec::new(),
            name: None,
            reason: None,
            updater_plugin_options: None,
            updater_plugins: Vec::new(),
            validator_plugin_options: None,
            validator_plugins: Vec::new(),
        }
    }


    /// Sets the value of CoordinatorOptions
    pub fn set_coordinator_options(&mut self, value: String) {
        self.coordinator_options = Some(value);
    }

    /// Gets the value of CoordinatorOptions
    pub fn get_coordinator_options(&self) -> Option<&String> {
        self.coordinator_options.as_ref()
    }

    /// Sets the value of DrainPluginOptions
    pub fn set_drain_plugin_options(&mut self, value: String) {
        self.drain_plugin_options = Some(value);
    }

    /// Gets the value of DrainPluginOptions
    pub fn get_drain_plugin_options(&self) -> Option<&String> {
        self.drain_plugin_options.as_ref()
    }

    /// Sets the value of DrainPlugins
    pub fn set_drain_plugins(&mut self, value: Vec<String>) {
        self.drain_plugins = value;
    }

    /// Gets the value of DrainPlugins
    pub fn get_drain_plugins(&self) -> &Vec<String> {
        &self.drain_plugins
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Reason
    pub fn set_reason(&mut self, value: String) {
        self.reason = Some(value);
    }

    /// Gets the value of Reason
    pub fn get_reason(&self) -> Option<&String> {
        self.reason.as_ref()
    }

    /// Sets the value of UpdaterPluginOptions
    pub fn set_updater_plugin_options(&mut self, value: String) {
        self.updater_plugin_options = Some(value);
    }

    /// Gets the value of UpdaterPluginOptions
    pub fn get_updater_plugin_options(&self) -> Option<&String> {
        self.updater_plugin_options.as_ref()
    }

    /// Sets the value of UpdaterPlugins
    pub fn set_updater_plugins(&mut self, value: Vec<String>) {
        self.updater_plugins = value;
    }

    /// Gets the value of UpdaterPlugins
    pub fn get_updater_plugins(&self) -> &Vec<String> {
        &self.updater_plugins
    }

    /// Sets the value of ValidatorPluginOptions
    pub fn set_validator_plugin_options(&mut self, value: String) {
        self.validator_plugin_options = Some(value);
    }

    /// Gets the value of ValidatorPluginOptions
    pub fn get_validator_plugin_options(&self) -> Option<&String> {
        self.validator_plugin_options.as_ref()
    }

    /// Sets the value of ValidatorPlugins
    pub fn set_validator_plugins(&mut self, value: Vec<String>) {
        self.validator_plugins = value;
    }

    /// Gets the value of ValidatorPlugins
    pub fn get_validator_plugins(&self) -> &Vec<String> {
        &self.validator_plugins
    }

/// 

    /// * `coordinator_options` -  (String)
    /// * `drain_plugin_options` -  (String)
    /// * `drain_plugins` -  (String[])
    /// * `name` -  (String)
    /// * `reason` -  (String)
    /// * `updater_plugin_options` -  (String)
    /// * `updater_plugins` -  (String[])
    /// * `validator_plugin_options` -  (String)
    /// * `validator_plugins` -  (String[])

    /// * `created_update_template` -  (MSCluster_UpdateTemplate)
    /// * `return_value` -  (u32)
    pub fn create_update_template(&self, name: &String, coordinator_options: &String, updater_plugins: &Vec<String>, updater_plugin_options: &String, validator_plugins: &Vec<String>, validator_plugin_options: &String, drain_plugins: &Vec<String>, drain_plugin_options: &String, reason: &String, created_update_template: &mut MSCluster_UpdateTemplate) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "CoordinatorOptions".to_string(), value: coordinator_options.into() });
        args.push(MethodParameter { name: "UpdaterPlugins".to_string(), value: updater_plugins.into() });
        args.push(MethodParameter { name: "UpdaterPluginOptions".to_string(), value: updater_plugin_options.into() });
        args.push(MethodParameter { name: "ValidatorPlugins".to_string(), value: validator_plugins.into() });
        args.push(MethodParameter { name: "ValidatorPluginOptions".to_string(), value: validator_plugin_options.into() });
        args.push(MethodParameter { name: "DrainPlugins".to_string(), value: drain_plugins.into() });
        args.push(MethodParameter { name: "DrainPluginOptions".to_string(), value: drain_plugin_options.into() });
        args.push(MethodParameter { name: "Reason".to_string(), value: reason.into() });

        let result = self.invoke_method("CreateUpdateTemplate", &args)?;
        let created_update_template = result.get_value("CreatedUpdateTemplate")?;
        Ok(result.return_value)

    }


/// 

    /// * `coordinator_options` -  (String)
    /// * `drain_plugin_options` -  (String)
    /// * `drain_plugins` -  (String[])
    /// * `new_name` -  (String)
    /// * `reason` -  (String)
    /// * `updater_plugin_options` -  (String)
    /// * `updater_plugins` -  (String[])
    /// * `validator_plugin_options` -  (String)
    /// * `validator_plugins` -  (String[])

    /// * `return_value` -  (u32)
    pub fn set_update_template(&self, new_name: &String, coordinator_options: &String, updater_plugins: &Vec<String>, updater_plugin_options: &String, validator_plugins: &Vec<String>, validator_plugin_options: &String, drain_plugins: &Vec<String>, drain_plugin_options: &String, reason: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NewName".to_string(), value: new_name.into() });
        args.push(MethodParameter { name: "CoordinatorOptions".to_string(), value: coordinator_options.into() });
        args.push(MethodParameter { name: "UpdaterPlugins".to_string(), value: updater_plugins.into() });
        args.push(MethodParameter { name: "UpdaterPluginOptions".to_string(), value: updater_plugin_options.into() });
        args.push(MethodParameter { name: "ValidatorPlugins".to_string(), value: validator_plugins.into() });
        args.push(MethodParameter { name: "ValidatorPluginOptions".to_string(), value: validator_plugin_options.into() });
        args.push(MethodParameter { name: "DrainPlugins".to_string(), value: drain_plugins.into() });
        args.push(MethodParameter { name: "DrainPluginOptions".to_string(), value: drain_plugin_options.into() });
        args.push(MethodParameter { name: "Reason".to_string(), value: reason.into() });
        self.invoke_method("SetUpdateTemplate", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn remove_update_template(&self) -> Result<(), WmiError> {
        self.invoke_method("RemoveUpdateTemplate", &[])

    }

}

