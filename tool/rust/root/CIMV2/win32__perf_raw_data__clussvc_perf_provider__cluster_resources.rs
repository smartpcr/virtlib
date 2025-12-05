// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_ClussvcPerfProvider_ClusterResources struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_ClussvcPerfProvider_ClusterResources {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "ResourceControls")]
    pub resource_controls: Option<u64>,

/// 
    #[serde(rename = "ResourceControlsPersec")]
    pub resource_controls_persec: Option<u64>,

/// 
    #[serde(rename = "ResourceFailure")]
    pub resource_failure: Option<u64>,

/// 
    #[serde(rename = "ResourceFailureAccessViolation")]
    pub resource_failure_access_violation: Option<u64>,

/// 
    #[serde(rename = "ResourceFailureDeadlock")]
    pub resource_failure_deadlock: Option<u64>,

/// 
    #[serde(rename = "ResourcesOnline")]
    pub resources_online: Option<u64>,

/// 
    #[serde(rename = "ResourceTypeControls")]
    pub resource_type_controls: Option<u64>,

/// 
    #[serde(rename = "ResourceTypeControlsPersec")]
    pub resource_type_controls_persec: Option<u64>,
}

impl Win32_PerfRawData_ClussvcPerfProvider_ClusterResources {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            resource_controls: None,
            resource_controls_persec: None,
            resource_failure: None,
            resource_failure_access_violation: None,
            resource_failure_deadlock: None,
            resources_online: None,
            resource_type_controls: None,
            resource_type_controls_persec: None,
        }
    }


    /// Sets the value of ResourceControls
    pub fn set_resource_controls(&mut self, value: u64) {
        self.resource_controls = Some(value);
    }

    /// Gets the value of ResourceControls
    pub fn get_resource_controls(&self) -> Option<&u64> {
        self.resource_controls.as_ref()
    }

    /// Sets the value of ResourceControlsPersec
    pub fn set_resource_controls_persec(&mut self, value: u64) {
        self.resource_controls_persec = Some(value);
    }

    /// Gets the value of ResourceControlsPersec
    pub fn get_resource_controls_persec(&self) -> Option<&u64> {
        self.resource_controls_persec.as_ref()
    }

    /// Sets the value of ResourceFailure
    pub fn set_resource_failure(&mut self, value: u64) {
        self.resource_failure = Some(value);
    }

    /// Gets the value of ResourceFailure
    pub fn get_resource_failure(&self) -> Option<&u64> {
        self.resource_failure.as_ref()
    }

    /// Sets the value of ResourceFailureAccessViolation
    pub fn set_resource_failure_access_violation(&mut self, value: u64) {
        self.resource_failure_access_violation = Some(value);
    }

    /// Gets the value of ResourceFailureAccessViolation
    pub fn get_resource_failure_access_violation(&self) -> Option<&u64> {
        self.resource_failure_access_violation.as_ref()
    }

    /// Sets the value of ResourceFailureDeadlock
    pub fn set_resource_failure_deadlock(&mut self, value: u64) {
        self.resource_failure_deadlock = Some(value);
    }

    /// Gets the value of ResourceFailureDeadlock
    pub fn get_resource_failure_deadlock(&self) -> Option<&u64> {
        self.resource_failure_deadlock.as_ref()
    }

    /// Sets the value of ResourcesOnline
    pub fn set_resources_online(&mut self, value: u64) {
        self.resources_online = Some(value);
    }

    /// Gets the value of ResourcesOnline
    pub fn get_resources_online(&self) -> Option<&u64> {
        self.resources_online.as_ref()
    }

    /// Sets the value of ResourceTypeControls
    pub fn set_resource_type_controls(&mut self, value: u64) {
        self.resource_type_controls = Some(value);
    }

    /// Gets the value of ResourceTypeControls
    pub fn get_resource_type_controls(&self) -> Option<&u64> {
        self.resource_type_controls.as_ref()
    }

    /// Sets the value of ResourceTypeControlsPersec
    pub fn set_resource_type_controls_persec(&mut self, value: u64) {
        self.resource_type_controls_persec = Some(value);
    }

    /// Gets the value of ResourceTypeControlsPersec
    pub fn get_resource_type_controls_persec(&self) -> Option<&u64> {
        self.resource_type_controls_persec.as_ref()
    }
}

