// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DNSClientCache struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DNSClientCache {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 680
    #[serde(rename = "Data")]
    pub data: Option<String>,

/// 675
    #[serde(rename = "DataLength")]
    pub data_length: Option<u16>,

/// 663
    #[serde(rename = "Entry")]
    pub entry: Option<String>,

/// 664
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 676
    #[serde(rename = "Section")]
    pub section: Option<DNSClientCache_Section>,

/// 681
    #[serde(rename = "Status")]
    pub status: Option<DNSClientCache_Status>,

/// 674
    #[serde(rename = "TimeToLive")]
    pub time_to_live: Option<u32>,

/// 665
    #[serde(rename = "Type")]
    pub type: Option<DNSClientCache_Type>,
}

impl MSFT_DNSClientCache {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            data: None,
            data_length: None,
            entry: None,
            name: None,
            section: None,
            status: None,
            time_to_live: None,
            type: None,
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

    /// Sets the value of DataLength
    pub fn set_data_length(&mut self, value: u16) {
        self.data_length = Some(value);
    }

    /// Gets the value of DataLength
    pub fn get_data_length(&self) -> Option<&u16> {
        self.data_length.as_ref()
    }

    /// Sets the value of Entry
    pub fn set_entry(&mut self, value: String) {
        self.entry = Some(value);
    }

    /// Gets the value of Entry
    pub fn get_entry(&self) -> Option<&String> {
        self.entry.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Section
    pub fn set_section(&mut self, value: DNSClientCache_Section) {
        self.section = Some(value);
    }

    /// Gets the value of Section
    pub fn get_section(&self) -> Option<&DNSClientCache_Section> {
        self.section.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: DNSClientCache_Status) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&DNSClientCache_Status> {
        self.status.as_ref()
    }

    /// Sets the value of TimeToLive
    pub fn set_time_to_live(&mut self, value: u32) {
        self.time_to_live = Some(value);
    }

    /// Gets the value of TimeToLive
    pub fn get_time_to_live(&self) -> Option<&u32> {
        self.time_to_live.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: DNSClientCache_Type) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&DNSClientCache_Type> {
        self.type.as_ref()
    }

/// 684

    /// * `return_value` -  (u32)
    pub fn clear(&self) -> Result<(), WmiError> {
        self.invoke_method("Clear", &[])

    }

}

