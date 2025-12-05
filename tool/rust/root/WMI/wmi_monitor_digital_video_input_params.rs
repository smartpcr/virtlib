// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WmiMonitorDigitalVideoInputParams struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WmiMonitorDigitalVideoInputParams {
    #[serde(flatten)]
    pub base: MSMonitorClass,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "IsDFP1xCompatible")]
    pub is_dfp1x_compatible: Option<bool>,
}

impl WmiMonitorDigitalVideoInputParams {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSMonitorClass::new(),
            active: None,
            instance_name: None,
            is_dfp1x_compatible: None,
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

    /// Sets the value of IsDFP1xCompatible
    pub fn set_is_dfp1x_compatible(&mut self, value: bool) {
        self.is_dfp1x_compatible = Some(value);
    }

    /// Gets the value of IsDFP1xCompatible
    pub fn get_is_dfp1x_compatible(&self) -> Option<&bool> {
        self.is_dfp1x_compatible.as_ref()
    }
}

