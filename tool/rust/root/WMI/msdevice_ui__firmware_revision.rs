// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSDeviceUI_FirmwareRevision struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSDeviceUI_FirmwareRevision {
    #[serde(flatten)]
    pub base: MSDeviceUI,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "FirmwareRevision")]
    pub firmware_revision: Option<String>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl MSDeviceUI_FirmwareRevision {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSDeviceUI::new(),
            active: None,
            firmware_revision: None,
            instance_name: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of FirmwareRevision
    pub fn set_firmware_revision(&mut self, value: String) {
        self.firmware_revision = Some(value);
    }

    /// Gets the value of FirmwareRevision
    pub fn get_firmware_revision(&self) -> Option<&String> {
        self.firmware_revision.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }
}

