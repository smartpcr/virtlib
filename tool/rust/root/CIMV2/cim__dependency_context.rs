// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_DependencyContext struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_DependencyContext {

/// 
    #[serde(rename = "Context")]
    pub context: Option<CIM_Configuration>,

/// 
    #[serde(rename = "Dependency")]
    pub dependency: Option<CIM_Dependency>,
}

impl CIM_DependencyContext {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            context: None,
            dependency: None,
        }
    }


    /// Sets the value of Context
    pub fn set_context(&mut self, value: CIM_Configuration) {
        self.context = Some(value);
    }

    /// Gets the value of Context
    pub fn get_context(&self) -> Option<&CIM_Configuration> {
        self.context.as_ref()
    }

    /// Sets the value of Dependency
    pub fn set_dependency(&mut self, value: CIM_Dependency) {
        self.dependency = Some(value);
    }

    /// Gets the value of Dependency
    pub fn get_dependency(&self) -> Option<&CIM_Dependency> {
        self.dependency.as_ref()
    }
}

