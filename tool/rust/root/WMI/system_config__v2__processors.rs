// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SystemConfig_V2_Processors struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemConfig_V2_Processors {
    #[serde(flatten)]
    pub base: SystemConfig_V2,

/// 
    #[serde(rename = "FeatureSet")]
    pub feature_set: Option<u32>,

/// 
    #[serde(rename = "ProcessorIdentifier")]
    pub processor_identifier: Vec<char>,

/// 
    #[serde(rename = "ProcessorIndex")]
    pub processor_index: Option<u32>,

/// 
    #[serde(rename = "ProcessorName")]
    pub processor_name: Vec<char>,

/// 
    #[serde(rename = "ProcessorSpeed")]
    pub processor_speed: Option<u32>,

/// 
    #[serde(rename = "VendorIdentifier")]
    pub vendor_identifier: Vec<char>,
}

impl SystemConfig_V2_Processors {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SystemConfig_V2::new(),
            feature_set: None,
            processor_identifier: Vec::new(),
            processor_index: None,
            processor_name: Vec::new(),
            processor_speed: None,
            vendor_identifier: Vec::new(),
        }
    }


    /// Sets the value of FeatureSet
    pub fn set_feature_set(&mut self, value: u32) {
        self.feature_set = Some(value);
    }

    /// Gets the value of FeatureSet
    pub fn get_feature_set(&self) -> Option<&u32> {
        self.feature_set.as_ref()
    }

    /// Sets the value of ProcessorIdentifier
    pub fn set_processor_identifier(&mut self, value: Vec<char>) {
        self.processor_identifier = value;
    }

    /// Gets the value of ProcessorIdentifier
    pub fn get_processor_identifier(&self) -> &Vec<char> {
        &self.processor_identifier
    }

    /// Sets the value of ProcessorIndex
    pub fn set_processor_index(&mut self, value: u32) {
        self.processor_index = Some(value);
    }

    /// Gets the value of ProcessorIndex
    pub fn get_processor_index(&self) -> Option<&u32> {
        self.processor_index.as_ref()
    }

    /// Sets the value of ProcessorName
    pub fn set_processor_name(&mut self, value: Vec<char>) {
        self.processor_name = value;
    }

    /// Gets the value of ProcessorName
    pub fn get_processor_name(&self) -> &Vec<char> {
        &self.processor_name
    }

    /// Sets the value of ProcessorSpeed
    pub fn set_processor_speed(&mut self, value: u32) {
        self.processor_speed = Some(value);
    }

    /// Gets the value of ProcessorSpeed
    pub fn get_processor_speed(&self) -> Option<&u32> {
        self.processor_speed.as_ref()
    }

    /// Sets the value of VendorIdentifier
    pub fn set_vendor_identifier(&mut self, value: Vec<char>) {
        self.vendor_identifier = value;
    }

    /// Gets the value of VendorIdentifier
    pub fn get_vendor_identifier(&self) -> &Vec<char> {
        &self.vendor_identifier
    }
}

