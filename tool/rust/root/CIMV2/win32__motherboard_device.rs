// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_MotherboardDevice struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_MotherboardDevice {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,

/// 
    #[serde(rename = "PrimaryBusType")]
    pub primary_bus_type: Option<String>,

/// 
    #[serde(rename = "RevisionNumber")]
    pub revision_number: Option<String>,

/// 
    #[serde(rename = "SecondaryBusType")]
    pub secondary_bus_type: Option<String>,
}

impl Win32_MotherboardDevice {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
            primary_bus_type: None,
            revision_number: None,
            secondary_bus_type: None,
        }
    }


    /// Sets the value of PrimaryBusType
    pub fn set_primary_bus_type(&mut self, value: String) {
        self.primary_bus_type = Some(value);
    }

    /// Gets the value of PrimaryBusType
    pub fn get_primary_bus_type(&self) -> Option<&String> {
        self.primary_bus_type.as_ref()
    }

    /// Sets the value of RevisionNumber
    pub fn set_revision_number(&mut self, value: String) {
        self.revision_number = Some(value);
    }

    /// Gets the value of RevisionNumber
    pub fn get_revision_number(&self) -> Option<&String> {
        self.revision_number.as_ref()
    }

    /// Sets the value of SecondaryBusType
    pub fn set_secondary_bus_type(&mut self, value: String) {
        self.secondary_bus_type = Some(value);
    }

    /// Gets the value of SecondaryBusType
    pub fn get_secondary_bus_type(&self) -> Option<&String> {
        self.secondary_bus_type.as_ref()
    }
}

