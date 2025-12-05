// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_WorkflowServiceHost4000_WorkflowServiceHost4000 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_WorkflowServiceHost4000_WorkflowServiceHost4000 {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "AverageWorkflowLoadTime")]
    pub average_workflow_load_time: Option<u32>,

/// 
    #[serde(rename = "AverageWorkflowLoadTime_Base")]
    pub average_workflow_load_time__base: Option<u32>,

/// 
    #[serde(rename = "AverageWorkflowPersistTime")]
    pub average_workflow_persist_time: Option<u32>,

/// 
    #[serde(rename = "AverageWorkflowPersistTime_Base")]
    pub average_workflow_persist_time__base: Option<u32>,

/// 
    #[serde(rename = "WorkflowsAborted")]
    pub workflows_aborted: Option<u32>,

/// 
    #[serde(rename = "WorkflowsAbortedPerSecond")]
    pub workflows_aborted_per_second: Option<u32>,

/// 
    #[serde(rename = "WorkflowsCompleted")]
    pub workflows_completed: Option<u32>,

/// 
    #[serde(rename = "WorkflowsCompletedPerSecond")]
    pub workflows_completed_per_second: Option<u32>,

/// 
    #[serde(rename = "WorkflowsCreated")]
    pub workflows_created: Option<u32>,

/// 
    #[serde(rename = "WorkflowsCreatedPerSecond")]
    pub workflows_created_per_second: Option<u32>,

/// 
    #[serde(rename = "WorkflowsExecuting")]
    pub workflows_executing: Option<u32>,

/// 
    #[serde(rename = "WorkflowsIdlePerSecond")]
    pub workflows_idle_per_second: Option<u32>,

/// 
    #[serde(rename = "WorkflowsInMemory")]
    pub workflows_in_memory: Option<u32>,

/// 
    #[serde(rename = "WorkflowsLoaded")]
    pub workflows_loaded: Option<u32>,

/// 
    #[serde(rename = "WorkflowsLoadedPerSecond")]
    pub workflows_loaded_per_second: Option<u32>,

/// 
    #[serde(rename = "WorkflowsPersisted")]
    pub workflows_persisted: Option<u32>,

/// 
    #[serde(rename = "WorkflowsPersistedPerSecond")]
    pub workflows_persisted_per_second: Option<u32>,

/// 
    #[serde(rename = "WorkflowsSuspended")]
    pub workflows_suspended: Option<u32>,

/// 
    #[serde(rename = "WorkflowsSuspendedPerSecond")]
    pub workflows_suspended_per_second: Option<u32>,

/// 
    #[serde(rename = "WorkflowsTerminated")]
    pub workflows_terminated: Option<u32>,

/// 
    #[serde(rename = "WorkflowsTerminatedPerSecond")]
    pub workflows_terminated_per_second: Option<u32>,

/// 
    #[serde(rename = "WorkflowsUnloaded")]
    pub workflows_unloaded: Option<u32>,

/// 
    #[serde(rename = "WorkflowsUnloadedPerSecond")]
    pub workflows_unloaded_per_second: Option<u32>,
}

impl Win32_PerfRawData_WorkflowServiceHost4000_WorkflowServiceHost4000 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            average_workflow_load_time: None,
            average_workflow_load_time__base: None,
            average_workflow_persist_time: None,
            average_workflow_persist_time__base: None,
            workflows_aborted: None,
            workflows_aborted_per_second: None,
            workflows_completed: None,
            workflows_completed_per_second: None,
            workflows_created: None,
            workflows_created_per_second: None,
            workflows_executing: None,
            workflows_idle_per_second: None,
            workflows_in_memory: None,
            workflows_loaded: None,
            workflows_loaded_per_second: None,
            workflows_persisted: None,
            workflows_persisted_per_second: None,
            workflows_suspended: None,
            workflows_suspended_per_second: None,
            workflows_terminated: None,
            workflows_terminated_per_second: None,
            workflows_unloaded: None,
            workflows_unloaded_per_second: None,
        }
    }


    /// Sets the value of AverageWorkflowLoadTime
    pub fn set_average_workflow_load_time(&mut self, value: u32) {
        self.average_workflow_load_time = Some(value);
    }

    /// Gets the value of AverageWorkflowLoadTime
    pub fn get_average_workflow_load_time(&self) -> Option<&u32> {
        self.average_workflow_load_time.as_ref()
    }

    /// Sets the value of AverageWorkflowLoadTime_Base
    pub fn set_average_workflow_load_time__base(&mut self, value: u32) {
        self.average_workflow_load_time__base = Some(value);
    }

    /// Gets the value of AverageWorkflowLoadTime_Base
    pub fn get_average_workflow_load_time__base(&self) -> Option<&u32> {
        self.average_workflow_load_time__base.as_ref()
    }

    /// Sets the value of AverageWorkflowPersistTime
    pub fn set_average_workflow_persist_time(&mut self, value: u32) {
        self.average_workflow_persist_time = Some(value);
    }

    /// Gets the value of AverageWorkflowPersistTime
    pub fn get_average_workflow_persist_time(&self) -> Option<&u32> {
        self.average_workflow_persist_time.as_ref()
    }

    /// Sets the value of AverageWorkflowPersistTime_Base
    pub fn set_average_workflow_persist_time__base(&mut self, value: u32) {
        self.average_workflow_persist_time__base = Some(value);
    }

    /// Gets the value of AverageWorkflowPersistTime_Base
    pub fn get_average_workflow_persist_time__base(&self) -> Option<&u32> {
        self.average_workflow_persist_time__base.as_ref()
    }

    /// Sets the value of WorkflowsAborted
    pub fn set_workflows_aborted(&mut self, value: u32) {
        self.workflows_aborted = Some(value);
    }

    /// Gets the value of WorkflowsAborted
    pub fn get_workflows_aborted(&self) -> Option<&u32> {
        self.workflows_aborted.as_ref()
    }

    /// Sets the value of WorkflowsAbortedPerSecond
    pub fn set_workflows_aborted_per_second(&mut self, value: u32) {
        self.workflows_aborted_per_second = Some(value);
    }

    /// Gets the value of WorkflowsAbortedPerSecond
    pub fn get_workflows_aborted_per_second(&self) -> Option<&u32> {
        self.workflows_aborted_per_second.as_ref()
    }

    /// Sets the value of WorkflowsCompleted
    pub fn set_workflows_completed(&mut self, value: u32) {
        self.workflows_completed = Some(value);
    }

    /// Gets the value of WorkflowsCompleted
    pub fn get_workflows_completed(&self) -> Option<&u32> {
        self.workflows_completed.as_ref()
    }

    /// Sets the value of WorkflowsCompletedPerSecond
    pub fn set_workflows_completed_per_second(&mut self, value: u32) {
        self.workflows_completed_per_second = Some(value);
    }

    /// Gets the value of WorkflowsCompletedPerSecond
    pub fn get_workflows_completed_per_second(&self) -> Option<&u32> {
        self.workflows_completed_per_second.as_ref()
    }

    /// Sets the value of WorkflowsCreated
    pub fn set_workflows_created(&mut self, value: u32) {
        self.workflows_created = Some(value);
    }

    /// Gets the value of WorkflowsCreated
    pub fn get_workflows_created(&self) -> Option<&u32> {
        self.workflows_created.as_ref()
    }

    /// Sets the value of WorkflowsCreatedPerSecond
    pub fn set_workflows_created_per_second(&mut self, value: u32) {
        self.workflows_created_per_second = Some(value);
    }

    /// Gets the value of WorkflowsCreatedPerSecond
    pub fn get_workflows_created_per_second(&self) -> Option<&u32> {
        self.workflows_created_per_second.as_ref()
    }

    /// Sets the value of WorkflowsExecuting
    pub fn set_workflows_executing(&mut self, value: u32) {
        self.workflows_executing = Some(value);
    }

    /// Gets the value of WorkflowsExecuting
    pub fn get_workflows_executing(&self) -> Option<&u32> {
        self.workflows_executing.as_ref()
    }

    /// Sets the value of WorkflowsIdlePerSecond
    pub fn set_workflows_idle_per_second(&mut self, value: u32) {
        self.workflows_idle_per_second = Some(value);
    }

    /// Gets the value of WorkflowsIdlePerSecond
    pub fn get_workflows_idle_per_second(&self) -> Option<&u32> {
        self.workflows_idle_per_second.as_ref()
    }

    /// Sets the value of WorkflowsInMemory
    pub fn set_workflows_in_memory(&mut self, value: u32) {
        self.workflows_in_memory = Some(value);
    }

    /// Gets the value of WorkflowsInMemory
    pub fn get_workflows_in_memory(&self) -> Option<&u32> {
        self.workflows_in_memory.as_ref()
    }

    /// Sets the value of WorkflowsLoaded
    pub fn set_workflows_loaded(&mut self, value: u32) {
        self.workflows_loaded = Some(value);
    }

    /// Gets the value of WorkflowsLoaded
    pub fn get_workflows_loaded(&self) -> Option<&u32> {
        self.workflows_loaded.as_ref()
    }

    /// Sets the value of WorkflowsLoadedPerSecond
    pub fn set_workflows_loaded_per_second(&mut self, value: u32) {
        self.workflows_loaded_per_second = Some(value);
    }

    /// Gets the value of WorkflowsLoadedPerSecond
    pub fn get_workflows_loaded_per_second(&self) -> Option<&u32> {
        self.workflows_loaded_per_second.as_ref()
    }

    /// Sets the value of WorkflowsPersisted
    pub fn set_workflows_persisted(&mut self, value: u32) {
        self.workflows_persisted = Some(value);
    }

    /// Gets the value of WorkflowsPersisted
    pub fn get_workflows_persisted(&self) -> Option<&u32> {
        self.workflows_persisted.as_ref()
    }

    /// Sets the value of WorkflowsPersistedPerSecond
    pub fn set_workflows_persisted_per_second(&mut self, value: u32) {
        self.workflows_persisted_per_second = Some(value);
    }

    /// Gets the value of WorkflowsPersistedPerSecond
    pub fn get_workflows_persisted_per_second(&self) -> Option<&u32> {
        self.workflows_persisted_per_second.as_ref()
    }

    /// Sets the value of WorkflowsSuspended
    pub fn set_workflows_suspended(&mut self, value: u32) {
        self.workflows_suspended = Some(value);
    }

    /// Gets the value of WorkflowsSuspended
    pub fn get_workflows_suspended(&self) -> Option<&u32> {
        self.workflows_suspended.as_ref()
    }

    /// Sets the value of WorkflowsSuspendedPerSecond
    pub fn set_workflows_suspended_per_second(&mut self, value: u32) {
        self.workflows_suspended_per_second = Some(value);
    }

    /// Gets the value of WorkflowsSuspendedPerSecond
    pub fn get_workflows_suspended_per_second(&self) -> Option<&u32> {
        self.workflows_suspended_per_second.as_ref()
    }

    /// Sets the value of WorkflowsTerminated
    pub fn set_workflows_terminated(&mut self, value: u32) {
        self.workflows_terminated = Some(value);
    }

    /// Gets the value of WorkflowsTerminated
    pub fn get_workflows_terminated(&self) -> Option<&u32> {
        self.workflows_terminated.as_ref()
    }

    /// Sets the value of WorkflowsTerminatedPerSecond
    pub fn set_workflows_terminated_per_second(&mut self, value: u32) {
        self.workflows_terminated_per_second = Some(value);
    }

    /// Gets the value of WorkflowsTerminatedPerSecond
    pub fn get_workflows_terminated_per_second(&self) -> Option<&u32> {
        self.workflows_terminated_per_second.as_ref()
    }

    /// Sets the value of WorkflowsUnloaded
    pub fn set_workflows_unloaded(&mut self, value: u32) {
        self.workflows_unloaded = Some(value);
    }

    /// Gets the value of WorkflowsUnloaded
    pub fn get_workflows_unloaded(&self) -> Option<&u32> {
        self.workflows_unloaded.as_ref()
    }

    /// Sets the value of WorkflowsUnloadedPerSecond
    pub fn set_workflows_unloaded_per_second(&mut self, value: u32) {
        self.workflows_unloaded_per_second = Some(value);
    }

    /// Gets the value of WorkflowsUnloadedPerSecond
    pub fn get_workflows_unloaded_per_second(&self) -> Option<&u32> {
        self.workflows_unloaded_per_second.as_ref()
    }
}

