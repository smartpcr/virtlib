// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapter_RssProcessor struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapter_RssProcessor {

/// 
    #[serde(rename = "PreferenceIndex")]
    pub preference_index: Option<u16>,

/// 
    #[serde(rename = "ProcessorGroup")]
    pub processor_group: Option<u16>,

/// 
    #[serde(rename = "ProcessorNumber")]
    pub processor_number: Option<u8>,
}

impl MSFT_NetAdapter_RssProcessor {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            preference_index: None,
            processor_group: None,
            processor_number: None,
        }
    }


    /// Sets the value of PreferenceIndex
    pub fn set_preference_index(&mut self, value: u16) {
        self.preference_index = Some(value);
    }

    /// Gets the value of PreferenceIndex
    pub fn get_preference_index(&self) -> Option<&u16> {
        self.preference_index.as_ref()
    }

    /// Sets the value of ProcessorGroup
    pub fn set_processor_group(&mut self, value: u16) {
        self.processor_group = Some(value);
    }

    /// Gets the value of ProcessorGroup
    pub fn get_processor_group(&self) -> Option<&u16> {
        self.processor_group.as_ref()
    }

    /// Sets the value of ProcessorNumber
    pub fn set_processor_number(&mut self, value: u8) {
        self.processor_number = Some(value);
    }

    /// Gets the value of ProcessorNumber
    pub fn get_processor_number(&self) -> Option<&u8> {
        self.processor_number.as_ref()
    }
}

