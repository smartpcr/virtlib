// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_WindowsWorkflowFoundation4000_WFSystemWorkflow4000 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_WindowsWorkflowFoundation4000_WFSystemWorkflow4000 {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "WorkflowsAborted")]
    pub workflows_aborted: Option<u32>,

/// 
    #[serde(rename = "WorkflowsAbortedPersec")]
    pub workflows_aborted_persec: Option<u32>,

/// 
    #[serde(rename = "WorkflowsCompleted")]
    pub workflows_completed: Option<u32>,

/// 
    #[serde(rename = "WorkflowsCompletedPersec")]
    pub workflows_completed_persec: Option<u32>,

/// 
    #[serde(rename = "WorkflowsCreated")]
    pub workflows_created: Option<u32>,

/// 
    #[serde(rename = "WorkflowsCreatedPersec")]
    pub workflows_created_persec: Option<u32>,

/// 
    #[serde(rename = "WorkflowsExecuting")]
    pub workflows_executing: Option<u32>,

/// 
    #[serde(rename = "WorkflowsIdlePersec")]
    pub workflows_idle_persec: Option<u32>,

/// 
    #[serde(rename = "WorkflowsInMemory")]
    pub workflows_in_memory: Option<u32>,

/// 
    #[serde(rename = "WorkflowsLoaded")]
    pub workflows_loaded: Option<u32>,

/// 
    #[serde(rename = "WorkflowsLoadedPersec")]
    pub workflows_loaded_persec: Option<u32>,

/// 
    #[serde(rename = "WorkflowsPending")]
    pub workflows_pending: Option<u32>,

/// 
    #[serde(rename = "WorkflowsPersisted")]
    pub workflows_persisted: Option<u32>,

/// 
    #[serde(rename = "WorkflowsPersistedPersec")]
    pub workflows_persisted_persec: Option<u32>,

/// 
    #[serde(rename = "WorkflowsRunnable")]
    pub workflows_runnable: Option<u32>,

/// 
    #[serde(rename = "WorkflowsSuspended")]
    pub workflows_suspended: Option<u32>,

/// 
    #[serde(rename = "WorkflowsSuspendedPersec")]
    pub workflows_suspended_persec: Option<u32>,

/// 
    #[serde(rename = "WorkflowsTerminated")]
    pub workflows_terminated: Option<u32>,

/// 
    #[serde(rename = "WorkflowsTerminatedPersec")]
    pub workflows_terminated_persec: Option<u32>,

/// 
    #[serde(rename = "WorkflowsUnloaded")]
    pub workflows_unloaded: Option<u32>,

/// 
    #[serde(rename = "WorkflowsUnloadedPersec")]
    pub workflows_unloaded_persec: Option<u32>,
}

impl Win32_PerfRawData_WindowsWorkflowFoundation4000_WFSystemWorkflow4000 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            workflows_aborted: None,
            workflows_aborted_persec: None,
            workflows_completed: None,
            workflows_completed_persec: None,
            workflows_created: None,
            workflows_created_persec: None,
            workflows_executing: None,
            workflows_idle_persec: None,
            workflows_in_memory: None,
            workflows_loaded: None,
            workflows_loaded_persec: None,
            workflows_pending: None,
            workflows_persisted: None,
            workflows_persisted_persec: None,
            workflows_runnable: None,
            workflows_suspended: None,
            workflows_suspended_persec: None,
            workflows_terminated: None,
            workflows_terminated_persec: None,
            workflows_unloaded: None,
            workflows_unloaded_persec: None,
        }
    }


    /// Sets the value of WorkflowsAborted
    pub fn set_workflows_aborted(&mut self, value: u32) {
        self.workflows_aborted = Some(value);
    }

    /// Gets the value of WorkflowsAborted
    pub fn get_workflows_aborted(&self) -> Option<&u32> {
        self.workflows_aborted.as_ref()
    }

    /// Sets the value of WorkflowsAbortedPersec
    pub fn set_workflows_aborted_persec(&mut self, value: u32) {
        self.workflows_aborted_persec = Some(value);
    }

    /// Gets the value of WorkflowsAbortedPersec
    pub fn get_workflows_aborted_persec(&self) -> Option<&u32> {
        self.workflows_aborted_persec.as_ref()
    }

    /// Sets the value of WorkflowsCompleted
    pub fn set_workflows_completed(&mut self, value: u32) {
        self.workflows_completed = Some(value);
    }

    /// Gets the value of WorkflowsCompleted
    pub fn get_workflows_completed(&self) -> Option<&u32> {
        self.workflows_completed.as_ref()
    }

    /// Sets the value of WorkflowsCompletedPersec
    pub fn set_workflows_completed_persec(&mut self, value: u32) {
        self.workflows_completed_persec = Some(value);
    }

    /// Gets the value of WorkflowsCompletedPersec
    pub fn get_workflows_completed_persec(&self) -> Option<&u32> {
        self.workflows_completed_persec.as_ref()
    }

    /// Sets the value of WorkflowsCreated
    pub fn set_workflows_created(&mut self, value: u32) {
        self.workflows_created = Some(value);
    }

    /// Gets the value of WorkflowsCreated
    pub fn get_workflows_created(&self) -> Option<&u32> {
        self.workflows_created.as_ref()
    }

    /// Sets the value of WorkflowsCreatedPersec
    pub fn set_workflows_created_persec(&mut self, value: u32) {
        self.workflows_created_persec = Some(value);
    }

    /// Gets the value of WorkflowsCreatedPersec
    pub fn get_workflows_created_persec(&self) -> Option<&u32> {
        self.workflows_created_persec.as_ref()
    }

    /// Sets the value of WorkflowsExecuting
    pub fn set_workflows_executing(&mut self, value: u32) {
        self.workflows_executing = Some(value);
    }

    /// Gets the value of WorkflowsExecuting
    pub fn get_workflows_executing(&self) -> Option<&u32> {
        self.workflows_executing.as_ref()
    }

    /// Sets the value of WorkflowsIdlePersec
    pub fn set_workflows_idle_persec(&mut self, value: u32) {
        self.workflows_idle_persec = Some(value);
    }

    /// Gets the value of WorkflowsIdlePersec
    pub fn get_workflows_idle_persec(&self) -> Option<&u32> {
        self.workflows_idle_persec.as_ref()
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

    /// Sets the value of WorkflowsLoadedPersec
    pub fn set_workflows_loaded_persec(&mut self, value: u32) {
        self.workflows_loaded_persec = Some(value);
    }

    /// Gets the value of WorkflowsLoadedPersec
    pub fn get_workflows_loaded_persec(&self) -> Option<&u32> {
        self.workflows_loaded_persec.as_ref()
    }

    /// Sets the value of WorkflowsPending
    pub fn set_workflows_pending(&mut self, value: u32) {
        self.workflows_pending = Some(value);
    }

    /// Gets the value of WorkflowsPending
    pub fn get_workflows_pending(&self) -> Option<&u32> {
        self.workflows_pending.as_ref()
    }

    /// Sets the value of WorkflowsPersisted
    pub fn set_workflows_persisted(&mut self, value: u32) {
        self.workflows_persisted = Some(value);
    }

    /// Gets the value of WorkflowsPersisted
    pub fn get_workflows_persisted(&self) -> Option<&u32> {
        self.workflows_persisted.as_ref()
    }

    /// Sets the value of WorkflowsPersistedPersec
    pub fn set_workflows_persisted_persec(&mut self, value: u32) {
        self.workflows_persisted_persec = Some(value);
    }

    /// Gets the value of WorkflowsPersistedPersec
    pub fn get_workflows_persisted_persec(&self) -> Option<&u32> {
        self.workflows_persisted_persec.as_ref()
    }

    /// Sets the value of WorkflowsRunnable
    pub fn set_workflows_runnable(&mut self, value: u32) {
        self.workflows_runnable = Some(value);
    }

    /// Gets the value of WorkflowsRunnable
    pub fn get_workflows_runnable(&self) -> Option<&u32> {
        self.workflows_runnable.as_ref()
    }

    /// Sets the value of WorkflowsSuspended
    pub fn set_workflows_suspended(&mut self, value: u32) {
        self.workflows_suspended = Some(value);
    }

    /// Gets the value of WorkflowsSuspended
    pub fn get_workflows_suspended(&self) -> Option<&u32> {
        self.workflows_suspended.as_ref()
    }

    /// Sets the value of WorkflowsSuspendedPersec
    pub fn set_workflows_suspended_persec(&mut self, value: u32) {
        self.workflows_suspended_persec = Some(value);
    }

    /// Gets the value of WorkflowsSuspendedPersec
    pub fn get_workflows_suspended_persec(&self) -> Option<&u32> {
        self.workflows_suspended_persec.as_ref()
    }

    /// Sets the value of WorkflowsTerminated
    pub fn set_workflows_terminated(&mut self, value: u32) {
        self.workflows_terminated = Some(value);
    }

    /// Gets the value of WorkflowsTerminated
    pub fn get_workflows_terminated(&self) -> Option<&u32> {
        self.workflows_terminated.as_ref()
    }

    /// Sets the value of WorkflowsTerminatedPersec
    pub fn set_workflows_terminated_persec(&mut self, value: u32) {
        self.workflows_terminated_persec = Some(value);
    }

    /// Gets the value of WorkflowsTerminatedPersec
    pub fn get_workflows_terminated_persec(&self) -> Option<&u32> {
        self.workflows_terminated_persec.as_ref()
    }

    /// Sets the value of WorkflowsUnloaded
    pub fn set_workflows_unloaded(&mut self, value: u32) {
        self.workflows_unloaded = Some(value);
    }

    /// Gets the value of WorkflowsUnloaded
    pub fn get_workflows_unloaded(&self) -> Option<&u32> {
        self.workflows_unloaded.as_ref()
    }

    /// Sets the value of WorkflowsUnloadedPersec
    pub fn set_workflows_unloaded_persec(&mut self, value: u32) {
        self.workflows_unloaded_persec = Some(value);
    }

    /// Gets the value of WorkflowsUnloadedPersec
    pub fn get_workflows_unloaded_persec(&self) -> Option<&u32> {
        self.workflows_unloaded_persec.as_ref()
    }
}

