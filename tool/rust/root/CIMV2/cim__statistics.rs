// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Statistics struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Statistics {

/// 
    #[serde(rename = "Element")]
    pub element: Option<CIM_ManagedSystemElement>,

/// 
    #[serde(rename = "Stats")]
    pub stats: Option<CIM_StatisticalInformation>,
}

impl CIM_Statistics {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            element: None,
            stats: None,
        }
    }


    /// Sets the value of Element
    pub fn set_element(&mut self, value: CIM_ManagedSystemElement) {
        self.element = Some(value);
    }

    /// Gets the value of Element
    pub fn get_element(&self) -> Option<&CIM_ManagedSystemElement> {
        self.element.as_ref()
    }

    /// Sets the value of Stats
    pub fn set_stats(&mut self, value: CIM_StatisticalInformation) {
        self.stats = Some(value);
    }

    /// Gets the value of Stats
    pub fn get_stats(&self) -> Option<&CIM_StatisticalInformation> {
        self.stats.as_ref()
    }
}

