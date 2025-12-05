// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_GuestServiceInterfaceComponentSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_GuestServiceInterfaceComponentSettingData {
    #[serde(flatten)]
    pub base: CIM_ResourceAllocationSettingData,

/// 
    #[serde(rename = "DefaultEnabledStatePolicy")]
    pub default_enabled_state_policy: Option<GuestServiceInterfaceComponentSettingData_DefaultEnabledStatePolicy>,

/// 
    #[serde(rename = "EnabledState")]
    pub enabled_state: Option<GuestServiceInterfaceComponentSettingData_EnabledState>,
}

impl Msvm_GuestServiceInterfaceComponentSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ResourceAllocationSettingData::new(),
            default_enabled_state_policy: None,
            enabled_state: None,
        }
    }


    /// Sets the value of DefaultEnabledStatePolicy
    pub fn set_default_enabled_state_policy(&mut self, value: GuestServiceInterfaceComponentSettingData_DefaultEnabledStatePolicy) {
        self.default_enabled_state_policy = Some(value);
    }

    /// Gets the value of DefaultEnabledStatePolicy
    pub fn get_default_enabled_state_policy(&self) -> Option<&GuestServiceInterfaceComponentSettingData_DefaultEnabledStatePolicy> {
        self.default_enabled_state_policy.as_ref()
    }

    /// Sets the value of EnabledState
    pub fn set_enabled_state(&mut self, value: GuestServiceInterfaceComponentSettingData_EnabledState) {
        self.enabled_state = Some(value);
    }

    /// Gets the value of EnabledState
    pub fn get_enabled_state(&self) -> Option<&GuestServiceInterfaceComponentSettingData_EnabledState> {
        self.enabled_state.as_ref()
    }
}

impl Msvm_GuestServiceInterfaceComponentSettingData {
    /// Gets the related Msvm_VirtualSystemSettingData object(s)
    pub fn get_related__virtual_system_setting_data(&self) -> Result<Msvm_VirtualSystemSettingData, WmiError> {
        self.get_related("Msvm_VirtualSystemSettingData")
    }

}

