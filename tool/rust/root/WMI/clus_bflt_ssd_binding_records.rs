// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ClusBfltSsdBindingRecords struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusBfltSsdBindingRecords {

/// Cache Store Binding Records.
    #[serde(rename = "BindingRecords")]
    pub binding_records: Vec<ClusBfltHddBindingRecord>,

/// Number of Binding Records.
    #[serde(rename = "NumberOfBindingRecords")]
    pub number_of_binding_records: Option<u32>,
}

impl ClusBfltSsdBindingRecords {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            binding_records: Vec::new(),
            number_of_binding_records: None,
        }
    }


    /// Sets the value of BindingRecords
    pub fn set_binding_records(&mut self, value: Vec<ClusBfltHddBindingRecord>) {
        self.binding_records = value;
    }

    /// Gets the value of BindingRecords
    pub fn get_binding_records(&self) -> &Vec<ClusBfltHddBindingRecord> {
        &self.binding_records
    }

    /// Sets the value of NumberOfBindingRecords
    pub fn set_number_of_binding_records(&mut self, value: u32) {
        self.number_of_binding_records = Some(value);
    }

    /// Gets the value of NumberOfBindingRecords
    pub fn get_number_of_binding_records(&self) -> Option<&u32> {
        self.number_of_binding_records.as_ref()
    }
}

