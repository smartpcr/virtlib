// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_WorkerVpProvider_HyperVWorkerVirtualProcessor struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_WorkerVpProvider_HyperVWorkerVirtualProcessor {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "InitialAPICID")]
    pub initial_apicid: Option<u64>,

/// 
    #[serde(rename = "InterceptDelayTimems")]
    pub intercept_delay_timems: Option<u64>,

/// 
    #[serde(rename = "InterceptsDelayed")]
    pub intercepts_delayed: Option<u64>,

/// 
    #[serde(rename = "MPIDR")]
    pub mpidr: Option<u64>,

/// 
    #[serde(rename = "TargetSubnode")]
    pub target_subnode: Option<u64>,
}

impl Win32_PerfFormattedData_WorkerVpProvider_HyperVWorkerVirtualProcessor {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            initial_apicid: None,
            intercept_delay_timems: None,
            intercepts_delayed: None,
            mpidr: None,
            target_subnode: None,
        }
    }


    /// Sets the value of InitialAPICID
    pub fn set_initial_apicid(&mut self, value: u64) {
        self.initial_apicid = Some(value);
    }

    /// Gets the value of InitialAPICID
    pub fn get_initial_apicid(&self) -> Option<&u64> {
        self.initial_apicid.as_ref()
    }

    /// Sets the value of InterceptDelayTimems
    pub fn set_intercept_delay_timems(&mut self, value: u64) {
        self.intercept_delay_timems = Some(value);
    }

    /// Gets the value of InterceptDelayTimems
    pub fn get_intercept_delay_timems(&self) -> Option<&u64> {
        self.intercept_delay_timems.as_ref()
    }

    /// Sets the value of InterceptsDelayed
    pub fn set_intercepts_delayed(&mut self, value: u64) {
        self.intercepts_delayed = Some(value);
    }

    /// Gets the value of InterceptsDelayed
    pub fn get_intercepts_delayed(&self) -> Option<&u64> {
        self.intercepts_delayed.as_ref()
    }

    /// Sets the value of MPIDR
    pub fn set_mpidr(&mut self, value: u64) {
        self.mpidr = Some(value);
    }

    /// Gets the value of MPIDR
    pub fn get_mpidr(&self) -> Option<&u64> {
        self.mpidr.as_ref()
    }

    /// Sets the value of TargetSubnode
    pub fn set_target_subnode(&mut self, value: u64) {
        self.target_subnode = Some(value);
    }

    /// Gets the value of TargetSubnode
    pub fn get_target_subnode(&self) -> Option<&u64> {
        self.target_subnode.as_ref()
    }
}

