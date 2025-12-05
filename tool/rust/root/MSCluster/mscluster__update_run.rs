// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_UpdateRun struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_UpdateRun {

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "NodeResult")]
    pub node_result: Vec<String>,

/// 
    #[serde(rename = "RunEndTime")]
    pub run_end_time: Option<String>,

/// 
    #[serde(rename = "RunParameters")]
    pub run_parameters: Option<MSCluster_UpdateTemplate>,

/// 
    #[serde(rename = "RunStartTime")]
    pub run_start_time: Option<String>,

/// 
    #[serde(rename = "StartReason")]
    pub start_reason: Option<String>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<String>,

/// 
    #[serde(rename = "StopReason")]
    pub stop_reason: Option<String>,
}

impl MSCluster_UpdateRun {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            name: None,
            node_result: Vec::new(),
            run_end_time: None,
            run_parameters: None,
            run_start_time: None,
            start_reason: None,
            status: None,
            stop_reason: None,
        }
    }


    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of NodeResult
    pub fn set_node_result(&mut self, value: Vec<String>) {
        self.node_result = value;
    }

    /// Gets the value of NodeResult
    pub fn get_node_result(&self) -> &Vec<String> {
        &self.node_result
    }

    /// Sets the value of RunEndTime
    pub fn set_run_end_time(&mut self, value: String) {
        self.run_end_time = Some(value);
    }

    /// Gets the value of RunEndTime
    pub fn get_run_end_time(&self) -> Option<&String> {
        self.run_end_time.as_ref()
    }

    /// Sets the value of RunParameters
    pub fn set_run_parameters(&mut self, value: MSCluster_UpdateTemplate) {
        self.run_parameters = Some(value);
    }

    /// Gets the value of RunParameters
    pub fn get_run_parameters(&self) -> Option<&MSCluster_UpdateTemplate> {
        self.run_parameters.as_ref()
    }

    /// Sets the value of RunStartTime
    pub fn set_run_start_time(&mut self, value: String) {
        self.run_start_time = Some(value);
    }

    /// Gets the value of RunStartTime
    pub fn get_run_start_time(&self) -> Option<&String> {
        self.run_start_time.as_ref()
    }

    /// Sets the value of StartReason
    pub fn set_start_reason(&mut self, value: String) {
        self.start_reason = Some(value);
    }

    /// Gets the value of StartReason
    pub fn get_start_reason(&self) -> Option<&String> {
        self.start_reason.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: String) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&String> {
        self.status.as_ref()
    }

    /// Sets the value of StopReason
    pub fn set_stop_reason(&mut self, value: String) {
        self.stop_reason = Some(value);
    }

    /// Gets the value of StopReason
    pub fn get_stop_reason(&self) -> Option<&String> {
        self.stop_reason.as_ref()
    }

/// 

    /// * `coordinator_options` -  (String)
    /// * `drain_plugin_options` -  (String)
    /// * `drain_plugins` -  (String[])
    /// * `name` -  (String)
    /// * `reason` -  (String)
    /// * `template_name` -  (String)
    /// * `updater_plugin_options` -  (String)
    /// * `updater_plugins` -  (String[])
    /// * `validator_plugin_options` -  (String)
    /// * `validator_plugins` -  (String[])

    /// * `return_value` -  (u32)
    /// * `started_update_run` -  (MSCluster_UpdateRun)
    pub fn start_update_run(&self, name: &String, template_name: &String, coordinator_options: &String, updater_plugins: &Vec<String>, updater_plugin_options: &String, validator_plugins: &Vec<String>, validator_plugin_options: &String, drain_plugins: &Vec<String>, drain_plugin_options: &String, reason: &String, started_update_run: &mut MSCluster_UpdateRun) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "TemplateName".to_string(), value: template_name.into() });
        args.push(MethodParameter { name: "CoordinatorOptions".to_string(), value: coordinator_options.into() });
        args.push(MethodParameter { name: "UpdaterPlugins".to_string(), value: updater_plugins.into() });
        args.push(MethodParameter { name: "UpdaterPluginOptions".to_string(), value: updater_plugin_options.into() });
        args.push(MethodParameter { name: "ValidatorPlugins".to_string(), value: validator_plugins.into() });
        args.push(MethodParameter { name: "ValidatorPluginOptions".to_string(), value: validator_plugin_options.into() });
        args.push(MethodParameter { name: "DrainPlugins".to_string(), value: drain_plugins.into() });
        args.push(MethodParameter { name: "DrainPluginOptions".to_string(), value: drain_plugin_options.into() });
        args.push(MethodParameter { name: "Reason".to_string(), value: reason.into() });

        let result = self.invoke_method("StartUpdateRun", &args)?;
        let started_update_run = result.get_value("StartedUpdateRun")?;
        Ok(result.return_value)

    }


/// 

    /// * `reason` -  (String)

    /// * `return_value` -  (u32)
    pub fn stop_update_run(&self, reason: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Reason".to_string(), value: reason.into() });
        self.invoke_method("StopUpdateRun", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn remove_update_run(&self) -> Result<(), WmiError> {
        self.invoke_method("RemoveUpdateRun", &[])

    }

}

