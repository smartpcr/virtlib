// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Container struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Container {
    #[serde(flatten)]
    pub base: CIM_Component,

/// 
    #[serde(rename = "LocationWithinContainer")]
    pub location_within_container: Option<String>,
}

impl CIM_Container {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Component::new(),
            location_within_container: None,
        }
    }


    /// Sets the value of LocationWithinContainer
    pub fn set_location_within_container(&mut self, value: String) {
        self.location_within_container = Some(value);
    }

    /// Gets the value of LocationWithinContainer
    pub fn get_location_within_container(&self) -> Option<&String> {
        self.location_within_container.as_ref()
    }
}

