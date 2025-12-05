// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_InterruptModerationParameters struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_InterruptModerationParameters {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,

/// 
    #[serde(rename = "Header")]
    pub header: Option<MSNdis_ObjectHeader>,

/// 
    #[serde(rename = "InterruptModeration")]
    pub interrupt_moderation: Option<u32>,
}

impl MSNdis_InterruptModerationParameters {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            flags: None,
            header: None,
            interrupt_moderation: None,
        }
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

    /// Sets the value of InterruptModeration
    pub fn set_interrupt_moderation(&mut self, value: u32) {
        self.interrupt_moderation = Some(value);
    }

    /// Gets the value of InterruptModeration
    pub fn get_interrupt_moderation(&self) -> Option<&u32> {
        self.interrupt_moderation.as_ref()
    }
}

