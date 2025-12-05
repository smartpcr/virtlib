// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_GuestCommunicationServiceSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_GuestCommunicationServiceSettingData {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// EnabledStatePolicy is an integer enumeration that indicates the enabled, disabled or default state of the Msvm_GuestCommunicationServiceSettingData.Enabled (2) indicates that the communication service is set to the enabled state.
/// Disabled (3) indicates that the communication service is set to the disabled state.
/// Deferred (8) indicates that the communication service state depends on DefaultEnabledStatePolicy in Msvm_GuestInterfaceComponentSettingData.
/// 
    #[serde(rename = "EnabledStatePolicy")]
    pub enabled_state_policy: Option<GuestCommunicationServiceSettingData_EnabledStatePolicy>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

impl Msvm_GuestCommunicationServiceSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            enabled_state_policy: None,
            name: None,
        }
    }


    /// Sets the value of EnabledStatePolicy
    pub fn set_enabled_state_policy(&mut self, value: GuestCommunicationServiceSettingData_EnabledStatePolicy) {
        self.enabled_state_policy = Some(value);
    }

    /// Gets the value of EnabledStatePolicy
    pub fn get_enabled_state_policy(&self) -> Option<&GuestCommunicationServiceSettingData_EnabledStatePolicy> {
        self.enabled_state_policy.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}

