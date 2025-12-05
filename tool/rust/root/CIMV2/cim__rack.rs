// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Rack struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Rack {
    #[serde(flatten)]
    pub base: CIM_PhysicalFrame,

/// 
    #[serde(rename = "CountryDesignation")]
    pub country_designation: Option<String>,

/// 
    #[serde(rename = "TypeOfRack")]
    pub type_of_rack: Option<u16>,
}

impl CIM_Rack {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PhysicalFrame::new(),
            country_designation: None,
            type_of_rack: None,
        }
    }


    /// Sets the value of CountryDesignation
    pub fn set_country_designation(&mut self, value: String) {
        self.country_designation = Some(value);
    }

    /// Gets the value of CountryDesignation
    pub fn get_country_designation(&self) -> Option<&String> {
        self.country_designation.as_ref()
    }

    /// Sets the value of TypeOfRack
    pub fn set_type_of_rack(&mut self, value: u16) {
        self.type_of_rack = Some(value);
    }

    /// Gets the value of TypeOfRack
    pub fn get_type_of_rack(&self) -> Option<&u16> {
        self.type_of_rack.as_ref()
    }
}

