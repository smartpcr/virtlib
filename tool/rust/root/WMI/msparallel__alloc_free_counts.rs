// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSParallel_AllocFreeCounts struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSParallel_AllocFreeCounts {
    #[serde(flatten)]
    pub base: MSParallel,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "PortAllocates")]
    pub port_allocates: Option<u32>,

/// 
    #[serde(rename = "PortFrees")]
    pub port_frees: Option<u32>,
}

impl MSParallel_AllocFreeCounts {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSParallel::new(),
            active: None,
            instance_name: None,
            port_allocates: None,
            port_frees: None,
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

    /// Sets the value of PortAllocates
    pub fn set_port_allocates(&mut self, value: u32) {
        self.port_allocates = Some(value);
    }

    /// Gets the value of PortAllocates
    pub fn get_port_allocates(&self) -> Option<&u32> {
        self.port_allocates.as_ref()
    }

    /// Sets the value of PortFrees
    pub fn set_port_frees(&mut self, value: u32) {
        self.port_frees = Some(value);
    }

    /// Gets the value of PortFrees
    pub fn get_port_frees(&self) -> Option<&u32> {
        self.port_frees.as_ref()
    }
}

