// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_WmiReceiveScaleCapabilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_WmiReceiveScaleCapabilities {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "CapabilitiesFlags")]
    pub capabilities_flags: Option<u32>,

/// 
    #[serde(rename = "Header")]
    pub header: Option<MSNdis_ObjectHeader>,

/// 
    #[serde(rename = "NumberOfInterruptMessages")]
    pub number_of_interrupt_messages: Option<u32>,

/// 
    #[serde(rename = "NumberOfReceiveQueues")]
    pub number_of_receive_queues: Option<u32>,
}

impl MSNdis_WmiReceiveScaleCapabilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            capabilities_flags: None,
            header: None,
            number_of_interrupt_messages: None,
            number_of_receive_queues: None,
        }
    }


    /// Sets the value of CapabilitiesFlags
    pub fn set_capabilities_flags(&mut self, value: u32) {
        self.capabilities_flags = Some(value);
    }

    /// Gets the value of CapabilitiesFlags
    pub fn get_capabilities_flags(&self) -> Option<&u32> {
        self.capabilities_flags.as_ref()
    }

    /// Sets the value of Header
    pub fn set_header(&mut self, value: MSNdis_ObjectHeader) {
        self.header = Some(value);
    }

    /// Gets the value of Header
    pub fn get_header(&self) -> Option<&MSNdis_ObjectHeader> {
        self.header.as_ref()
    }

    /// Sets the value of NumberOfInterruptMessages
    pub fn set_number_of_interrupt_messages(&mut self, value: u32) {
        self.number_of_interrupt_messages = Some(value);
    }

    /// Gets the value of NumberOfInterruptMessages
    pub fn get_number_of_interrupt_messages(&self) -> Option<&u32> {
        self.number_of_interrupt_messages.as_ref()
    }

    /// Sets the value of NumberOfReceiveQueues
    pub fn set_number_of_receive_queues(&mut self, value: u32) {
        self.number_of_receive_queues = Some(value);
    }

    /// Gets the value of NumberOfReceiveQueues
    pub fn get_number_of_receive_queues(&self) -> Option<&u32> {
        self.number_of_receive_queues.as_ref()
    }
}

