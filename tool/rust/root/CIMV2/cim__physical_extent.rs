// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_PhysicalExtent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_PhysicalExtent {
    #[serde(flatten)]
    pub base: CIM_StorageExtent,

/// 
    #[serde(rename = "UnitsBeforeCheckDataInterleave")]
    pub units_before_check_data_interleave: Option<u64>,

/// 
    #[serde(rename = "UnitsOfCheckData")]
    pub units_of_check_data: Option<u64>,

/// 
    #[serde(rename = "UnitsOfUserData")]
    pub units_of_user_data: Option<u64>,
}

impl CIM_PhysicalExtent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_StorageExtent::new(),
            units_before_check_data_interleave: None,
            units_of_check_data: None,
            units_of_user_data: None,
        }
    }


    /// Sets the value of UnitsBeforeCheckDataInterleave
    pub fn set_units_before_check_data_interleave(&mut self, value: u64) {
        self.units_before_check_data_interleave = Some(value);
    }

    /// Gets the value of UnitsBeforeCheckDataInterleave
    pub fn get_units_before_check_data_interleave(&self) -> Option<&u64> {
        self.units_before_check_data_interleave.as_ref()
    }

    /// Sets the value of UnitsOfCheckData
    pub fn set_units_of_check_data(&mut self, value: u64) {
        self.units_of_check_data = Some(value);
    }

    /// Gets the value of UnitsOfCheckData
    pub fn get_units_of_check_data(&self) -> Option<&u64> {
        self.units_of_check_data.as_ref()
    }

    /// Sets the value of UnitsOfUserData
    pub fn set_units_of_user_data(&mut self, value: u64) {
        self.units_of_user_data = Some(value);
    }

    /// Gets the value of UnitsOfUserData
    pub fn get_units_of_user_data(&self) -> Option<&u64> {
        self.units_of_user_data.as_ref()
    }
}

