// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Hardware
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Log struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Log {
    #[serde(flatten)]
    pub base: CIM_EnabledLogicalElement,

/// 
    #[serde(rename = "CurrentNumberOfRecords")]
    pub current_number_of_records: Option<u64>,

/// 
    #[serde(rename = "MaxNumberOfRecords")]
    pub max_number_of_records: Option<u64>,
}

impl CIM_Log {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_EnabledLogicalElement::new(),
            current_number_of_records: None,
            max_number_of_records: None,
        }
    }


    /// Sets the value of CurrentNumberOfRecords
    pub fn set_current_number_of_records(&mut self, value: u64) {
        self.current_number_of_records = Some(value);
    }

    /// Gets the value of CurrentNumberOfRecords
    pub fn get_current_number_of_records(&self) -> Option<&u64> {
        self.current_number_of_records.as_ref()
    }

    /// Sets the value of MaxNumberOfRecords
    pub fn set_max_number_of_records(&mut self, value: u64) {
        self.max_number_of_records = Some(value);
    }

    /// Gets the value of MaxNumberOfRecords
    pub fn get_max_number_of_records(&self) -> Option<&u64> {
        self.max_number_of_records.as_ref()
    }

/// 

    /// * `return_value` -  (u32)
    pub fn clear_log(&self) -> Result<(), WmiError> {
        self.invoke_method("ClearLog", &[])

    }

}

