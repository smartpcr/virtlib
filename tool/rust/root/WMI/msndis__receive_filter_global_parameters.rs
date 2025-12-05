// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_ReceiveFilterGlobalParameters struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_ReceiveFilterGlobalParameters {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "EnabledFilterTypes")]
    pub enabled_filter_types: Option<u32>,

/// 
    #[serde(rename = "EnabledQueueTypes")]
    pub enabled_queue_types: Option<u32>,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,

/// 
    #[serde(rename = "Header")]
    pub header: Option<MSNdis_ObjectHeader>,
}

impl MSNdis_ReceiveFilterGlobalParameters {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            enabled_filter_types: None,
            enabled_queue_types: None,
            flags: None,
            header: None,
        }
    }


    /// Sets the value of EnabledFilterTypes
    pub fn set_enabled_filter_types(&mut self, value: u32) {
        self.enabled_filter_types = Some(value);
    }

    /// Gets the value of EnabledFilterTypes
    pub fn get_enabled_filter_types(&self) -> Option<&u32> {
        self.enabled_filter_types.as_ref()
    }

    /// Sets the value of EnabledQueueTypes
    pub fn set_enabled_queue_types(&mut self, value: u32) {
        self.enabled_queue_types = Some(value);
    }

    /// Gets the value of EnabledQueueTypes
    pub fn get_enabled_queue_types(&self) -> Option<&u32> {
        self.enabled_queue_types.as_ref()
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

