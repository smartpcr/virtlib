// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_KvpExchangeDataItem struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_KvpExchangeDataItem {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "Data")]
    pub data: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Source")]
    pub source: Option<u16>,
}

impl Msvm_KvpExchangeDataItem {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            data: None,
            name: None,
            source: None,
        }
    }


    /// Sets the value of Data
    pub fn set_data(&mut self, value: String) {
        self.data = Some(value);
    }

    /// Gets the value of Data
    pub fn get_data(&self) -> Option<&String> {
        self.data.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Source
    pub fn set_source(&mut self, value: u16) {
        self.source = Some(value);
    }

    /// Gets the value of Source
    pub fn get_source(&self) -> Option<&u16> {
        self.source.as_ref()
    }
}

