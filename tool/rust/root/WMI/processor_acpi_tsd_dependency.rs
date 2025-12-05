// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ProcessorAcpiTsdDependency struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProcessorAcpiTsdDependency {

/// 
    #[serde(rename = "CoordType")]
    pub coord_type: Option<u32>,

/// 
    #[serde(rename = "Domain")]
    pub domain: Option<u32>,

/// 
    #[serde(rename = "NumberOfEntries")]
    pub number_of_entries: Option<u8>,

/// 
    #[serde(rename = "NumProcessors")]
    pub num_processors: Option<u32>,

/// 
    #[serde(rename = "Reserved1")]
    pub reserved1: Option<u32>,

/// 
    #[serde(rename = "Reserved2")]
    pub reserved2: Option<u64>,

/// 
    #[serde(rename = "Revision")]
    pub revision: Option<u8>,
}

impl ProcessorAcpiTsdDependency {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            coord_type: None,
            domain: None,
            number_of_entries: None,
            num_processors: None,
            reserved1: None,
            reserved2: None,
            revision: None,
        }
    }


    /// Sets the value of CoordType
    pub fn set_coord_type(&mut self, value: u32) {
        self.coord_type = Some(value);
    }

    /// Gets the value of CoordType
    pub fn get_coord_type(&self) -> Option<&u32> {
        self.coord_type.as_ref()
    }

    /// Sets the value of Domain
    pub fn set_domain(&mut self, value: u32) {
        self.domain = Some(value);
    }

    /// Gets the value of Domain
    pub fn get_domain(&self) -> Option<&u32> {
        self.domain.as_ref()
    }

    /// Sets the value of NumberOfEntries
    pub fn set_number_of_entries(&mut self, value: u8) {
        self.number_of_entries = Some(value);
    }

    /// Gets the value of NumberOfEntries
    pub fn get_number_of_entries(&self) -> Option<&u8> {
        self.number_of_entries.as_ref()
    }

    /// Sets the value of NumProcessors
    pub fn set_num_processors(&mut self, value: u32) {
        self.num_processors = Some(value);
    }

    /// Gets the value of NumProcessors
    pub fn get_num_processors(&self) -> Option<&u32> {
        self.num_processors.as_ref()
    }

    /// Sets the value of Reserved1
    pub fn set_reserved1(&mut self, value: u32) {
        self.reserved1 = Some(value);
    }

    /// Gets the value of Reserved1
    pub fn get_reserved1(&self) -> Option<&u32> {
        self.reserved1.as_ref()
    }

    /// Sets the value of Reserved2
    pub fn set_reserved2(&mut self, value: u64) {
        self.reserved2 = Some(value);
    }

    /// Gets the value of Reserved2
    pub fn get_reserved2(&self) -> Option<&u64> {
        self.reserved2.as_ref()
    }

    /// Sets the value of Revision
    pub fn set_revision(&mut self, value: u8) {
        self.revision = Some(value);
    }

    /// Gets the value of Revision
    pub fn get_revision(&self) -> Option<&u8> {
        self.revision.as_ref()
    }
}

