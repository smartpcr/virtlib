// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_EnumerateAdapterEx struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_EnumerateAdapterEx {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "EnumerateAdapter")]
    pub enumerate_adapter: Option<MSNdis_WmiEnumAdapter>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl MSNdis_EnumerateAdapterEx {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            active: None,
            enumerate_adapter: None,
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

    /// Sets the value of EnumerateAdapter
    pub fn set_enumerate_adapter(&mut self, value: MSNdis_WmiEnumAdapter) {
        self.enumerate_adapter = Some(value);
    }

    /// Gets the value of EnumerateAdapter
    pub fn get_enumerate_adapter(&self) -> Option<&MSNdis_WmiEnumAdapter> {
        self.enumerate_adapter.as_ref()
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

