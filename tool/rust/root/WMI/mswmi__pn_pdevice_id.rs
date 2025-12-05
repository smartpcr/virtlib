// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSWmi_PnPDeviceId struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSWmi_PnPDeviceId {
    #[serde(flatten)]
    pub base: MS_WmiInternal,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "PnPDeviceId")]
    pub pn_pdevice_id: Option<String>,
}

impl MSWmi_PnPDeviceId {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MS_WmiInternal::new(),
            active: None,
            instance_name: None,
            pn_pdevice_id: None,
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

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of PnPDeviceId
    pub fn set_pn_pdevice_id(&mut self, value: String) {
        self.pn_pdevice_id = Some(value);
    }

    /// Gets the value of PnPDeviceId
    pub fn get_pn_pdevice_id(&self) -> Option<&String> {
        self.pn_pdevice_id.as_ref()
    }
}

