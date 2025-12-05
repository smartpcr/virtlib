// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSI_Eventlog struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSI_Eventlog {
    #[serde(flatten)]
    pub base: __ExtrinsicEvent,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// Additional data to include in eventlog message, typically iSCSI Header
    #[serde(rename = "AdditionalData")]
    pub additional_data: Vec<u8>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// If zero then this event is not logged to system eventlog
    #[serde(rename = "LogToEventlog")]
    pub log_to_eventlog: Option<u32>,

/// Size of Additional Data
    #[serde(rename = "Size")]
    pub size: Option<u32>,

/// Type of eventlog message
    #[serde(rename = "Type")]
    pub type: Option<Eventlog_Type>,
}

impl MSiSCSI_Eventlog {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __ExtrinsicEvent::new(),
            active: None,
            additional_data: Vec::new(),
            instance_name: None,
            log_to_eventlog: None,
            size: None,
            type: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of AdditionalData
    pub fn set_additional_data(&mut self, value: Vec<u8>) {
        self.additional_data = value;
    }

    /// Gets the value of AdditionalData
    pub fn get_additional_data(&self) -> &Vec<u8> {
        &self.additional_data
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of LogToEventlog
    pub fn set_log_to_eventlog(&mut self, value: u32) {
        self.log_to_eventlog = Some(value);
    }

    /// Gets the value of LogToEventlog
    pub fn get_log_to_eventlog(&self) -> Option<&u32> {
        self.log_to_eventlog.as_ref()
    }

    /// Sets the value of Size
    pub fn set_size(&mut self, value: u32) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&u32> {
        self.size.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: Eventlog_Type) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&Eventlog_Type> {
        self.type.as_ref()
    }
}

