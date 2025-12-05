// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_ReceiveFilterInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_ReceiveFilterInfo {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "FilterId")]
    pub filter_id: Option<u32>,

/// 
    #[serde(rename = "FilterType")]
    pub filter_type: Option<u32>,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,

/// 
    #[serde(rename = "Header")]
    pub header: Option<MSNdis_ObjectHeader>,
}

impl MSNdis_ReceiveFilterInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            filter_id: None,
            filter_type: None,
            flags: None,
            header: None,
        }
    }


    /// Sets the value of FilterId
    pub fn set_filter_id(&mut self, value: u32) {
        self.filter_id = Some(value);
    }

    /// Gets the value of FilterId
    pub fn get_filter_id(&self) -> Option<&u32> {
        self.filter_id.as_ref()
    }

    /// Sets the value of FilterType
    pub fn set_filter_type(&mut self, value: u32) {
        self.filter_type = Some(value);
    }

    /// Gets the value of FilterType
    pub fn get_filter_type(&self) -> Option<&u32> {
        self.filter_type.as_ref()
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
}

