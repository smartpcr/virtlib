// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ControlledBy struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ControlledBy {
    #[serde(flatten)]
    pub base: CIM_DeviceConnection,

/// This property describes the accessibility of the device through the antecedent controller.
    #[serde(rename = "AccessMode")]
    pub access_mode: Option<ControlledBy_AccessMode>,

/// The property describes the priority given to accesses of the device through this controller. The highest priority path will have the lowest value for this parameter.
    #[serde(rename = "AccessPriority")]
    pub access_priority: Option<u16>,

/// The State property indicates whether the Controller is actively commanding or accessing the Device (value=1) or not (value=2). Also, the value, "Unknown" (0), can be defined. This information is necessary when a LogicalDevice can be commanded by, or accessed through, multiple Controllers.
    #[serde(rename = "AccessState")]
    pub access_state: Option<ControlledBy_AccessState>,

/// Address of associated Device in context of the antecedent Controller.
    #[serde(rename = "DeviceNumber")]
    pub device_number: Option<String>,

/// Number of hard resets issued by the Controller. A hard reset returns the Device to its initialization or boot-up state. All internal Device state information and data are lost.
    #[serde(rename = "NumberOfHardResets")]
    pub number_of_hard_resets: Option<u32>,

/// Number of soft resets issued by the Controller. A soft reset does not completely clear current Device state or data. Exact semantics are dependent on the Device and on the protocols and mechanisms used to communicate with the Device.
    #[serde(rename = "NumberOfSoftResets")]
    pub number_of_soft_resets: Option<u32>,

/// The time that the downstream Device was last reset by the Controller.
    #[serde(rename = "TimeOfDeviceReset")]
    pub time_of_device_reset: Option<String>,
}

impl CIM_ControlledBy {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_DeviceConnection::new(),
            access_mode: None,
            access_priority: None,
            access_state: None,
            device_number: None,
            number_of_hard_resets: None,
            number_of_soft_resets: None,
            time_of_device_reset: None,
        }
    }


    /// Sets the value of AccessMode
    pub fn set_access_mode(&mut self, value: ControlledBy_AccessMode) {
        self.access_mode = Some(value);
    }

    /// Gets the value of AccessMode
    pub fn get_access_mode(&self) -> Option<&ControlledBy_AccessMode> {
        self.access_mode.as_ref()
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
    pub fn set_access_state(&mut self, value: ControlledBy_AccessState) {
        self.access_state = Some(value);
    }

    /// Gets the value of AccessState
    pub fn get_access_state(&self) -> Option<&ControlledBy_AccessState> {
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

    /// Sets the value of NumberOfHardResets
    pub fn set_number_of_hard_resets(&mut self, value: u32) {
        self.number_of_hard_resets = Some(value);
    }

    /// Gets the value of NumberOfHardResets
    pub fn get_number_of_hard_resets(&self) -> Option<&u32> {
        self.number_of_hard_resets.as_ref()
    }

    /// Sets the value of NumberOfSoftResets
    pub fn set_number_of_soft_resets(&mut self, value: u32) {
        self.number_of_soft_resets = Some(value);
    }

    /// Gets the value of NumberOfSoftResets
    pub fn get_number_of_soft_resets(&self) -> Option<&u32> {
        self.number_of_soft_resets.as_ref()
    }

    /// Sets the value of TimeOfDeviceReset
    pub fn set_time_of_device_reset(&mut self, value: String) {
        self.time_of_device_reset = Some(value);
    }

    /// Gets the value of TimeOfDeviceReset
    pub fn get_time_of_device_reset(&self) -> Option<&String> {
        self.time_of_device_reset.as_ref()
    }
}

