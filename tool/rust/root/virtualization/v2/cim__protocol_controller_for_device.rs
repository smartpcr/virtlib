// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ProtocolControllerForDevice struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ProtocolControllerForDevice {
    #[serde(flatten)]
    pub base: CIM_Dependency,

/// This property describes the priority given to accesses of the device through this Controller. The highest priority path will have the lowest value for this parameter.
    #[serde(rename = "AccessPriority")]
    pub access_priority: Option<u16>,

/// The AccessState property describes the accessibility of the LogicalDevice through the ProtocolController. 
/// Unknown (0) indicates the instrumentation does not know whether access is or is not functioning. 
/// Active (2) indicates normal access. 
/// Inactive (3) indicates the instrumentation knows this path is not active, and one of the other values (below) does not apply. 
/// Replication in Progress (4) indicates that the path is temporarily inactive due to a replication activity. 
/// Mapping Inconsistency (5) indicates the instrumentation has detected that this path is inactive due to an inconsistency in the DeviceNumber/DeviceAccess configuration.
    #[serde(rename = "AccessState")]
    pub access_state: Option<ProtocolControllerForDevice_AccessState>,

/// Address of the associated Device in the context of the Antecedent Controller.
    #[serde(rename = "DeviceNumber")]
    pub device_number: Option<String>,
}

impl CIM_ProtocolControllerForDevice {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Dependency::new(),
            access_priority: None,
            access_state: None,
            device_number: None,
        }
    }


    /// Sets the value of AccessPriority
    pub fn set_access_priority(&mut self, value: u16) {
        self.access_priority = Some(value);
    }

    /// Gets the value of AccessPriority
    pub fn get_access_priority(&self) -> Option<&u16> {
        self.access_priority.as_ref()
    }

    /// Sets the value of AccessState
    pub fn set_access_state(&mut self, value: ProtocolControllerForDevice_AccessState) {
        self.access_state = Some(value);
    }

    /// Gets the value of AccessState
    pub fn get_access_state(&self) -> Option<&ProtocolControllerForDevice_AccessState> {
        self.access_state.as_ref()
    }

    /// Sets the value of DeviceNumber
    pub fn set_device_number(&mut self, value: String) {
        self.device_number = Some(value);
    }

    /// Gets the value of DeviceNumber
    pub fn get_device_number(&self) -> Option<&String> {
        self.device_number.as_ref()
    }
}

