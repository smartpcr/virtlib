// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_BIOSFeature struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_BIOSFeature {
    #[serde(flatten)]
    pub base: CIM_SoftwareFeature,

/// 
    #[serde(rename = "CharacteristicDescriptions")]
    pub characteristic_descriptions: Vec<String>,

/// 
    #[serde(rename = "Characteristics")]
    pub characteristics: Vec<u16>,
}

impl CIM_BIOSFeature {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SoftwareFeature::new(),
            characteristic_descriptions: Vec::new(),
            characteristics: Vec::new(),
        }
    }


    /// Sets the value of CharacteristicDescriptions
    pub fn set_characteristic_descriptions(&mut self, value: Vec<String>) {
        self.characteristic_descriptions = value;
    }

    /// Gets the value of CharacteristicDescriptions
    pub fn get_characteristic_descriptions(&self) -> &Vec<String> {
        &self.characteristic_descriptions
    }

    /// Sets the value of Characteristics
    pub fn set_characteristics(&mut self, value: Vec<u16>) {
        self.characteristics = value;
    }

    /// Gets the value of Characteristics
    pub fn get_characteristics(&self) -> &Vec<u16> {
        &self.characteristics
    }
}

