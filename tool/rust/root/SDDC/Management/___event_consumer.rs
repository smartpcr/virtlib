// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.SDDC.Management
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __EventConsumer struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __EventConsumer {
    #[serde(flatten)]
    pub base: __IndicationRelated,

/// 
    #[serde(rename = "CreatorSID")]
    pub creator_sid: Vec<u8>,

/// 
    #[serde(rename = "MachineName")]
    pub machine_name: Option<String>,

/// 
    #[serde(rename = "MaximumQueueSize")]
    pub maximum_queue_size: Option<u32>,
}

impl __EventConsumer {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __IndicationRelated::new(),
            creator_sid: Vec::new(),
            machine_name: None,
            maximum_queue_size: None,
        }
    }


    /// Sets the value of CreatorSID
    pub fn set_creator_sid(&mut self, value: Vec<u8>) {
        self.creator_sid = value;
    }

    /// Gets the value of CreatorSID
    pub fn get_creator_sid(&self) -> &Vec<u8> {
        &self.creator_sid
    }

    /// Sets the value of MachineName
    pub fn set_machine_name(&mut self, value: String) {
        self.machine_name = Some(value);
    }

    /// Gets the value of MachineName
    pub fn get_machine_name(&self) -> Option<&String> {
        self.machine_name.as_ref()
    }

    /// Sets the value of MaximumQueueSize
    pub fn set_maximum_queue_size(&mut self, value: u32) {
        self.maximum_queue_size = Some(value);
    }

    /// Gets the value of MaximumQueueSize
    pub fn get_maximum_queue_size(&self) -> Option<&u32> {
        self.maximum_queue_size.as_ref()
    }
}

