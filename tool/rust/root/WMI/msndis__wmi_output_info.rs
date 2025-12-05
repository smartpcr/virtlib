// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_WmiOutputInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_WmiOutputInfo {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "DataOffset")]
    pub data_offset: Option<u32>,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,

/// 
    #[serde(rename = "Header")]
    pub header: Option<MSNdis_ObjectHeader>,

/// 
    #[serde(rename = "Padding1")]
    pub padding1: Option<u8>,

/// 
    #[serde(rename = "Padding2")]
    pub padding2: Option<u16>,

/// 
    #[serde(rename = "SupportedRevision")]
    pub supported_revision: Option<u8>,
}

impl MSNdis_WmiOutputInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            data_offset: None,
            flags: None,
            header: None,
            padding1: None,
            padding2: None,
            supported_revision: None,
        }
    }


    /// Sets the value of DataOffset
    pub fn set_data_offset(&mut self, value: u32) {
        self.data_offset = Some(value);
    }

    /// Gets the value of DataOffset
    pub fn get_data_offset(&self) -> Option<&u32> {
        self.data_offset.as_ref()
    }

    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: u32) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&u32> {
        self.flags.as_ref()
    }

    /// Sets the value of Header
    pub fn set_header(&mut self, value: MSNdis_ObjectHeader) {
        self.header = Some(value);
    }

    /// Gets the value of Header
    pub fn get_header(&self) -> Option<&MSNdis_ObjectHeader> {
        self.header.as_ref()
    }

    /// Sets the value of Padding1
    pub fn set_padding1(&mut self, value: u8) {
        self.padding1 = Some(value);
    }

    /// Gets the value of Padding1
    pub fn get_padding1(&self) -> Option<&u8> {
        self.padding1.as_ref()
    }

    /// Sets the value of Padding2
    pub fn set_padding2(&mut self, value: u16) {
        self.padding2 = Some(value);
    }

    /// Gets the value of Padding2
    pub fn get_padding2(&self) -> Option<&u16> {
        self.padding2.as_ref()
    }

    /// Sets the value of SupportedRevision
    pub fn set_supported_revision(&mut self, value: u8) {
        self.supported_revision = Some(value);
    }

    /// Gets the value of SupportedRevision
    pub fn get_supported_revision(&self) -> Option<&u8> {
        self.supported_revision.as_ref()
    }
}

