// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSAgp_Information struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSAgp_Information {
    #[serde(flatten)]
    pub base: MSAgp,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "AgpCommand")]
    pub agp_command: Option<u32>,

/// 
    #[serde(rename = "AgpStatus")]
    pub agp_status: Option<u32>,

/// 
    #[serde(rename = "ApertureBase")]
    pub aperture_base: Option<u64>,

/// 
    #[serde(rename = "ApertureLength")]
    pub aperture_length: Option<u32>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl MSAgp_Information {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSAgp::new(),
            active: None,
            agp_command: None,
            agp_status: None,
            aperture_base: None,
            aperture_length: None,
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

    /// Sets the value of AgpCommand
    pub fn set_agp_command(&mut self, value: u32) {
        self.agp_command = Some(value);
    }

    /// Gets the value of AgpCommand
    pub fn get_agp_command(&self) -> Option<&u32> {
        self.agp_command.as_ref()
    }

    /// Sets the value of AgpStatus
    pub fn set_agp_status(&mut self, value: u32) {
        self.agp_status = Some(value);
    }

    /// Gets the value of AgpStatus
    pub fn get_agp_status(&self) -> Option<&u32> {
        self.agp_status.as_ref()
    }

    /// Sets the value of ApertureBase
    pub fn set_aperture_base(&mut self, value: u64) {
        self.aperture_base = Some(value);
    }

    /// Gets the value of ApertureBase
    pub fn get_aperture_base(&self) -> Option<&u64> {
        self.aperture_base.as_ref()
    }

    /// Sets the value of ApertureLength
    pub fn set_aperture_length(&mut self, value: u32) {
        self.aperture_length = Some(value);
    }

    /// Gets the value of ApertureLength
    pub fn get_aperture_length(&self) -> Option<&u32> {
        self.aperture_length.as_ref()
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

