// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_SummaryInformationBase struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_SummaryInformationBase {
    #[serde(flatten)]
    pub base: CIM_View,

/// 
    #[serde(rename = "CreationTime")]
    pub creation_time: Option<String>,

/// 
    #[serde(rename = "EnabledState")]
    pub enabled_state: Option<u16>,

/// 
    #[serde(rename = "EnhancedSessionModeState")]
    pub enhanced_session_mode_state: Option<u16>,

/// 
    #[serde(rename = "HealthState")]
    pub health_state: Option<u16>,

/// 
    #[serde(rename = "HostComputerSystemName")]
    pub host_computer_system_name: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Notes")]
    pub notes: Option<String>,

/// 
    #[serde(rename = "NumberOfProcessors")]
    pub number_of_processors: Option<u16>,

/// 
    #[serde(rename = "OperationalStatus")]
    pub operational_status: Vec<u16>,

/// 
    #[serde(rename = "OtherEnabledState")]
    pub other_enabled_state: Option<String>,

/// 
    #[serde(rename = "StatusDescriptions")]
    pub status_descriptions: Vec<String>,

/// 
    #[serde(rename = "UpTime")]
    pub up_time: Option<u64>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<String>,

/// 
    #[serde(rename = "VirtualSwitchNames")]
    pub virtual_switch_names: Vec<String>,

/// 
    #[serde(rename = "VirtualSystemSubType")]
    pub virtual_system_sub_type: Option<SummaryInformationBase_VirtualSystemSubType>,
}

impl Msvm_SummaryInformationBase {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_View::new(),
            creation_time: None,
            enabled_state: None,
            enhanced_session_mode_state: None,
            health_state: None,
            host_computer_system_name: None,
            name: None,
            notes: None,
            number_of_processors: None,
            operational_status: Vec::new(),
            other_enabled_state: None,
            status_descriptions: Vec::new(),
            up_time: None,
            version: None,
            virtual_switch_names: Vec::new(),
            virtual_system_sub_type: None,
        }
    }


    /// Sets the value of CreationTime
    pub fn set_creation_time(&mut self, value: String) {
        self.creation_time = Some(value);
    }

    /// Gets the value of CreationTime
    pub fn get_creation_time(&self) -> Option<&String> {
        self.creation_time.as_ref()
    }

    /// Sets the value of EnabledState
    pub fn set_enabled_state(&mut self, value: u16) {
        self.enabled_state = Some(value);
    }

    /// Gets the value of EnabledState
    pub fn get_enabled_state(&self) -> Option<&u16> {
        self.enabled_state.as_ref()
    }

    /// Sets the value of EnhancedSessionModeState
    pub fn set_enhanced_session_mode_state(&mut self, value: u16) {
        self.enhanced_session_mode_state = Some(value);
    }

    /// Gets the value of EnhancedSessionModeState
    pub fn get_enhanced_session_mode_state(&self) -> Option<&u16> {
        self.enhanced_session_mode_state.as_ref()
    }

    /// Sets the value of HealthState
    pub fn set_health_state(&mut self, value: u16) {
        self.health_state = Some(value);
    }

    /// Gets the value of HealthState
    pub fn get_health_state(&self) -> Option<&u16> {
        self.health_state.as_ref()
    }

    /// Sets the value of HostComputerSystemName
    pub fn set_host_computer_system_name(&mut self, value: String) {
        self.host_computer_system_name = Some(value);
    }

    /// Gets the value of HostComputerSystemName
    pub fn get_host_computer_system_name(&self) -> Option<&String> {
        self.host_computer_system_name.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Notes
    pub fn set_notes(&mut self, value: String) {
        self.notes = Some(value);
    }

    /// Gets the value of Notes
    pub fn get_notes(&self) -> Option<&String> {
        self.notes.as_ref()
    }

    /// Sets the value of NumberOfProcessors
    pub fn set_number_of_processors(&mut self, value: u16) {
        self.number_of_processors = Some(value);
    }

    /// Gets the value of NumberOfProcessors
    pub fn get_number_of_processors(&self) -> Option<&u16> {
        self.number_of_processors.as_ref()
    }

    /// Sets the value of OperationalStatus
    pub fn set_operational_status(&mut self, value: Vec<u16>) {
        self.operational_status = value;
    }

    /// Gets the value of OperationalStatus
    pub fn get_operational_status(&self) -> &Vec<u16> {
        &self.operational_status
    }

    /// Sets the value of OtherEnabledState
    pub fn set_other_enabled_state(&mut self, value: String) {
        self.other_enabled_state = Some(value);
    }

    /// Gets the value of OtherEnabledState
    pub fn get_other_enabled_state(&self) -> Option<&String> {
        self.other_enabled_state.as_ref()
    }

    /// Sets the value of StatusDescriptions
    pub fn set_status_descriptions(&mut self, value: Vec<String>) {
        self.status_descriptions = value;
    }

    /// Gets the value of StatusDescriptions
    pub fn get_status_descriptions(&self) -> &Vec<String> {
        &self.status_descriptions
    }

    /// Sets the value of UpTime
    pub fn set_up_time(&mut self, value: u64) {
        self.up_time = Some(value);
    }

    /// Gets the value of UpTime
    pub fn get_up_time(&self) -> Option<&u64> {
        self.up_time.as_ref()
    }

    /// Sets the value of Version
    pub fn set_version(&mut self, value: String) {
        self.version = Some(value);
    }

    /// Gets the value of Version
    pub fn get_version(&self) -> Option<&String> {
        self.version.as_ref()
    }

    /// Sets the value of VirtualSwitchNames
    pub fn set_virtual_switch_names(&mut self, value: Vec<String>) {
        self.virtual_switch_names = value;
    }

    /// Gets the value of VirtualSwitchNames
    pub fn get_virtual_switch_names(&self) -> &Vec<String> {
        &self.virtual_switch_names
    }

    /// Sets the value of VirtualSystemSubType
    pub fn set_virtual_system_sub_type(&mut self, value: SummaryInformationBase_VirtualSystemSubType) {
        self.virtual_system_sub_type = Some(value);
    }

    /// Gets the value of VirtualSystemSubType
    pub fn get_virtual_system_sub_type(&self) -> Option<&SummaryInformationBase_VirtualSystemSubType> {
        self.virtual_system_sub_type.as_ref()
    }
}

impl Msvm_SummaryInformationBase {
    /// Gets the related Msvm_ComputerSystem object(s)
    pub fn get_related__computer_system(&self) -> Result<Msvm_ComputerSystem, WmiError> {
        self.get_related("Msvm_ComputerSystem")
    }

}

