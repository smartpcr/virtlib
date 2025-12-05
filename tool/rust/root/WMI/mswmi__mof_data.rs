// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSWmi_MofData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSWmi_MofData {
    #[serde(flatten)]
    pub base: MS_WmiInternal,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "BinaryMofData")]
    pub binary_mof_data: Vec<u8>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "Size")]
    pub size: Option<u32>,

/// 
    #[serde(rename = "Unused1")]
    pub unused1: Option<u32>,

/// 
    #[serde(rename = "Unused2")]
    pub unused2: Option<u32>,

/// 
    #[serde(rename = "Unused4")]
    pub unused4: Option<u32>,
}

impl MSWmi_MofData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MS_WmiInternal::new(),
            active: None,
            binary_mof_data: Vec::new(),
            instance_name: None,
            size: None,
            unused1: None,
            unused2: None,
            unused4: None,
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

    /// Sets the value of BinaryMofData
    pub fn set_binary_mof_data(&mut self, value: Vec<u8>) {
        self.binary_mof_data = value;
    }

    /// Gets the value of BinaryMofData
    pub fn get_binary_mof_data(&self) -> &Vec<u8> {
        &self.binary_mof_data
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of Size
    pub fn set_size(&mut self, value: u32) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&u32> {
        self.size.as_ref()
    }

    /// Sets the value of Unused1
    pub fn set_unused1(&mut self, value: u32) {
        self.unused1 = Some(value);
    }

    /// Gets the value of Unused1
    pub fn get_unused1(&self) -> Option<&u32> {
        self.unused1.as_ref()
    }

    /// Sets the value of Unused2
    pub fn set_unused2(&mut self, value: u32) {
        self.unused2 = Some(value);
    }

    /// Gets the value of Unused2
    pub fn get_unused2(&self) -> Option<&u32> {
        self.unused2.as_ref()
    }

    /// Sets the value of Unused4
    pub fn set_unused4(&mut self, value: u32) {
        self.unused4 = Some(value);
    }

    /// Gets the value of Unused4
    pub fn get_unused4(&self) -> Option<&u32> {
        self.unused4.as_ref()
    }
}

