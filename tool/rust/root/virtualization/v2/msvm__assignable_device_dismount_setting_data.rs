// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_AssignableDeviceDismountSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_AssignableDeviceDismountSettingData {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// 
    #[serde(rename = "DeviceInstancePath")]
    pub device_instance_path: Option<String>,

/// 
    #[serde(rename = "DeviceLocationPath")]
    pub device_location_path: Option<String>,

/// 
    #[serde(rename = "RequireAcsSupport")]
    pub require_acs_support: Option<bool>,

/// 
    #[serde(rename = "RequireDeviceMitigations")]
    pub require_device_mitigations: Option<bool>,
}

impl Msvm_AssignableDeviceDismountSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            device_instance_path: None,
            device_location_path: None,
            require_acs_support: None,
            require_device_mitigations: None,
        }
    }


    /// Sets the value of DeviceInstancePath
    pub fn set_device_instance_path(&mut self, value: String) {
        self.device_instance_path = Some(value);
    }

    /// Gets the value of DeviceInstancePath
    pub fn get_device_instance_path(&self) -> Option<&String> {
        self.device_instance_path.as_ref()
    }

    /// Sets the value of DeviceLocationPath
    pub fn set_device_location_path(&mut self, value: String) {
        self.device_location_path = Some(value);
    }

    /// Gets the value of DeviceLocationPath
    pub fn get_device_location_path(&self) -> Option<&String> {
        self.device_location_path.as_ref()
    }

    /// Sets the value of RequireAcsSupport
    pub fn set_require_acs_support(&mut self, value: bool) {
        self.require_acs_support = Some(value);
    }

    /// Gets the value of RequireAcsSupport
    pub fn get_require_acs_support(&self) -> Option<&bool> {
        self.require_acs_support.as_ref()
    }

    /// Sets the value of RequireDeviceMitigations
    pub fn set_require_device_mitigations(&mut self, value: bool) {
        self.require_device_mitigations = Some(value);
    }

    /// Gets the value of RequireDeviceMitigations
    pub fn get_require_device_mitigations(&self) -> Option<&bool> {
        self.require_device_mitigations.as_ref()
    }
}

