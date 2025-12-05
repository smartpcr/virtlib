// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_WMI_GenericNonCOMEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_WMI_GenericNonCOMEvent {
    #[serde(flatten)]
    pub base: __ExtrinsicEvent,

/// 
    #[serde(rename = "ProcessId")]
    pub process_id: Option<u32>,

/// 
    #[serde(rename = "PropertyNames")]
    pub property_names: Vec<String>,

/// 
    #[serde(rename = "PropertyValues")]
    pub property_values: Vec<String>,

/// 
    #[serde(rename = "ProviderName")]
    pub provider_name: Option<String>,
}

impl MSFT_WMI_GenericNonCOMEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __ExtrinsicEvent::new(),
            process_id: None,
            property_names: Vec::new(),
            property_values: Vec::new(),
            provider_name: None,
        }
    }


    /// Sets the value of ProcessId
    pub fn set_process_id(&mut self, value: u32) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessId
    pub fn get_process_id(&self) -> Option<&u32> {
        self.process_id.as_ref()
    }

    /// Sets the value of PropertyNames
    pub fn set_property_names(&mut self, value: Vec<String>) {
        self.property_names = value;
    }

    /// Gets the value of PropertyNames
    pub fn get_property_names(&self) -> &Vec<String> {
        &self.property_names
    }

    /// Sets the value of PropertyValues
    pub fn set_property_values(&mut self, value: Vec<String>) {
        self.property_values = value;
    }

    /// Gets the value of PropertyValues
    pub fn get_property_values(&self) -> &Vec<String> {
        &self.property_values
    }

    /// Sets the value of ProviderName
    pub fn set_provider_name(&mut self, value: String) {
        self.provider_name = Some(value);
    }

    /// Gets the value of ProviderName
    pub fn get_provider_name(&self) -> Option<&String> {
        self.provider_name.as_ref()
    }
}

