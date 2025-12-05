// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ServiceServiceDependency struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ServiceServiceDependency {
    #[serde(flatten)]
    pub base: CIM_Dependency,

/// 
    #[serde(rename = "TypeOfDependency")]
    pub type_of_dependency: Option<u16>,
}

impl CIM_ServiceServiceDependency {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Dependency::new(),
            type_of_dependency: None,
        }
    }


    /// Sets the value of TypeOfDependency
    pub fn set_type_of_dependency(&mut self, value: u16) {
        self.type_of_dependency = Some(value);
    }

    /// Gets the value of TypeOfDependency
    pub fn get_type_of_dependency(&self) -> Option<&u16> {
        self.type_of_dependency.as_ref()
    }
}

