// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.TaskScheduler
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_TaskSettings struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_TaskSettings {

/// 
    #[serde(rename = "AllowDemandStart")]
    pub allow_demand_start: Option<bool>,

/// 
    #[serde(rename = "AllowHardTerminate")]
    pub allow_hard_terminate: Option<bool>,

/// 
    #[serde(rename = "Compatibility")]
    pub compatibility: Option<TaskSettings_Compatibility>,

/// 
    #[serde(rename = "DeleteExpiredTaskAfter")]
    pub delete_expired_task_after: Option<String>,

/// 
    #[serde(rename = "DisallowStartIfOnBatteries")]
    pub disallow_start_if_on_batteries: Option<bool>,

/// 
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

/// 
    #[serde(rename = "ExecutionTimeLimit")]
    pub execution_time_limit: Option<String>,

/// 
    #[serde(rename = "Hidden")]
    pub hidden: Option<bool>,

/// 
    #[serde(rename = "IdleSettings")]
    pub idle_settings: Option<MSFT_TaskIdleSettings>,

/// 
    #[serde(rename = "MultipleInstances")]
    pub multiple_instances: Option<TaskSettings_MultipleInstances>,

/// 
    #[serde(rename = "NetworkSettings")]
    pub network_settings: Option<MSFT_TaskNetworkSettings>,

/// 
    #[serde(rename = "Priority")]
    pub priority: Option<u32>,

/// 
    #[serde(rename = "RestartCount")]
    pub restart_count: Option<u32>,

/// 
    #[serde(rename = "RestartInterval")]
    pub restart_interval: Option<String>,

/// 
    #[serde(rename = "RunOnlyIfIdle")]
    pub run_only_if_idle: Option<bool>,

/// 
    #[serde(rename = "RunOnlyIfNetworkAvailable")]
    pub run_only_if_network_available: Option<bool>,

/// 
    #[serde(rename = "StartWhenAvailable")]
    pub start_when_available: Option<bool>,

/// 
    #[serde(rename = "StopIfGoingOnBatteries")]
    pub stop_if_going_on_batteries: Option<bool>,

/// 
    #[serde(rename = "WakeToRun")]
    pub wake_to_run: Option<bool>,
}

impl MSFT_TaskSettings {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_demand_start: None,
            allow_hard_terminate: None,
            compatibility: None,
            delete_expired_task_after: None,
            disallow_start_if_on_batteries: None,
            enabled: None,
            execution_time_limit: None,
            hidden: None,
            idle_settings: None,
            multiple_instances: None,
            network_settings: None,
            priority: None,
            restart_count: None,
            restart_interval: None,
            run_only_if_idle: None,
            run_only_if_network_available: None,
            start_when_available: None,
            stop_if_going_on_batteries: None,
            wake_to_run: None,
        }
    }


    /// Sets the value of AllowDemandStart
    pub fn set_allow_demand_start(&mut self, value: bool) {
        self.allow_demand_start = Some(value);
    }

    /// Gets the value of AllowDemandStart
    pub fn get_allow_demand_start(&self) -> Option<&bool> {
        self.allow_demand_start.as_ref()
    }

    /// Sets the value of AllowHardTerminate
    pub fn set_allow_hard_terminate(&mut self, value: bool) {
        self.allow_hard_terminate = Some(value);
    }

    /// Gets the value of AllowHardTerminate
    pub fn get_allow_hard_terminate(&self) -> Option<&bool> {
        self.allow_hard_terminate.as_ref()
    }

    /// Sets the value of Compatibility
    pub fn set_compatibility(&mut self, value: TaskSettings_Compatibility) {
        self.compatibility = Some(value);
    }

    /// Gets the value of Compatibility
    pub fn get_compatibility(&self) -> Option<&TaskSettings_Compatibility> {
        self.compatibility.as_ref()
    }

    /// Sets the value of DeleteExpiredTaskAfter
    pub fn set_delete_expired_task_after(&mut self, value: String) {
        self.delete_expired_task_after = Some(value);
    }

    /// Gets the value of DeleteExpiredTaskAfter
    pub fn get_delete_expired_task_after(&self) -> Option<&String> {
        self.delete_expired_task_after.as_ref()
    }

    /// Sets the value of DisallowStartIfOnBatteries
    pub fn set_disallow_start_if_on_batteries(&mut self, value: bool) {
        self.disallow_start_if_on_batteries = Some(value);
    }

    /// Gets the value of DisallowStartIfOnBatteries
    pub fn get_disallow_start_if_on_batteries(&self) -> Option<&bool> {
        self.disallow_start_if_on_batteries.as_ref()
    }

    /// Sets the value of Enabled
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = Some(value);
    }

    /// Gets the value of Enabled
    pub fn get_enabled(&self) -> Option<&bool> {
        self.enabled.as_ref()
    }

    /// Sets the value of ExecutionTimeLimit
    pub fn set_execution_time_limit(&mut self, value: String) {
        self.execution_time_limit = Some(value);
    }

    /// Gets the value of ExecutionTimeLimit
    pub fn get_execution_time_limit(&self) -> Option<&String> {
        self.execution_time_limit.as_ref()
    }

    /// Sets the value of Hidden
    pub fn set_hidden(&mut self, value: bool) {
        self.hidden = Some(value);
    }

    /// Gets the value of Hidden
    pub fn get_hidden(&self) -> Option<&bool> {
        self.hidden.as_ref()
    }

    /// Sets the value of IdleSettings
    pub fn set_idle_settings(&mut self, value: MSFT_TaskIdleSettings) {
        self.idle_settings = Some(value);
    }

    /// Gets the value of IdleSettings
    pub fn get_idle_settings(&self) -> Option<&MSFT_TaskIdleSettings> {
        self.idle_settings.as_ref()
    }

    /// Sets the value of MultipleInstances
    pub fn set_multiple_instances(&mut self, value: TaskSettings_MultipleInstances) {
        self.multiple_instances = Some(value);
    }

    /// Gets the value of MultipleInstances
    pub fn get_multiple_instances(&self) -> Option<&TaskSettings_MultipleInstances> {
        self.multiple_instances.as_ref()
    }

    /// Sets the value of NetworkSettings
    pub fn set_network_settings(&mut self, value: MSFT_TaskNetworkSettings) {
        self.network_settings = Some(value);
    }

    /// Gets the value of NetworkSettings
    pub fn get_network_settings(&self) -> Option<&MSFT_TaskNetworkSettings> {
        self.network_settings.as_ref()
    }

    /// Sets the value of Priority
    pub fn set_priority(&mut self, value: u32) {
        self.priority = Some(value);
    }

    /// Gets the value of Priority
    pub fn get_priority(&self) -> Option<&u32> {
        self.priority.as_ref()
    }

    /// Sets the value of RestartCount
    pub fn set_restart_count(&mut self, value: u32) {
        self.restart_count = Some(value);
    }

    /// Gets the value of RestartCount
    pub fn get_restart_count(&self) -> Option<&u32> {
        self.restart_count.as_ref()
    }

    /// Sets the value of RestartInterval
    pub fn set_restart_interval(&mut self, value: String) {
        self.restart_interval = Some(value);
    }

    /// Gets the value of RestartInterval
    pub fn get_restart_interval(&self) -> Option<&String> {
        self.restart_interval.as_ref()
    }

    /// Sets the value of RunOnlyIfIdle
    pub fn set_run_only_if_idle(&mut self, value: bool) {
        self.run_only_if_idle = Some(value);
    }

    /// Gets the value of RunOnlyIfIdle
    pub fn get_run_only_if_idle(&self) -> Option<&bool> {
        self.run_only_if_idle.as_ref()
    }

    /// Sets the value of RunOnlyIfNetworkAvailable
    pub fn set_run_only_if_network_available(&mut self, value: bool) {
        self.run_only_if_network_available = Some(value);
    }

    /// Gets the value of RunOnlyIfNetworkAvailable
    pub fn get_run_only_if_network_available(&self) -> Option<&bool> {
        self.run_only_if_network_available.as_ref()
    }

    /// Sets the value of StartWhenAvailable
    pub fn set_start_when_available(&mut self, value: bool) {
        self.start_when_available = Some(value);
    }

    /// Gets the value of StartWhenAvailable
    pub fn get_start_when_available(&self) -> Option<&bool> {
        self.start_when_available.as_ref()
    }

    /// Sets the value of StopIfGoingOnBatteries
    pub fn set_stop_if_going_on_batteries(&mut self, value: bool) {
        self.stop_if_going_on_batteries = Some(value);
    }

    /// Gets the value of StopIfGoingOnBatteries
    pub fn get_stop_if_going_on_batteries(&self) -> Option<&bool> {
        self.stop_if_going_on_batteries.as_ref()
    }

    /// Sets the value of WakeToRun
    pub fn set_wake_to_run(&mut self, value: bool) {
        self.wake_to_run = Some(value);
    }

    /// Gets the value of WakeToRun
    pub fn get_wake_to_run(&self) -> Option<&bool> {
        self.wake_to_run.as_ref()
    }
}

