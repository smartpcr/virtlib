// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SystemConfig_V2_IRQ struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemConfig_V2_IRQ {
    #[serde(flatten)]
    pub base: SystemConfig_V2,

/// 
    #[serde(rename = "DeviceDescription")]
    pub device_description: Option<String>,

/// 
    #[serde(rename = "DeviceDescriptionLen")]
    pub device_description_len: Option<u32>,

/// 
    #[serde(rename = "IRQAffinity")]
    pub irqaffinity: Option<u64>,

/// 
    #[serde(rename = "IRQNum")]
    pub irqnum: Option<u32>,
}

impl SystemConfig_V2_IRQ {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SystemConfig_V2::new(),
            device_description: None,
            device_description_len: None,
            irqaffinity: None,
            irqnum: None,
        }
    }


    /// Sets the value of DeviceDescription
    pub fn set_device_description(&mut self, value: String) {
        self.device_description = Some(value);
    }

    /// Gets the value of DeviceDescription
    pub fn get_device_description(&self) -> Option<&String> {
        self.device_description.as_ref()
    }

    /// Sets the value of DeviceDescriptionLen
    pub fn set_device_description_len(&mut self, value: u32) {
        self.device_description_len = Some(value);
    }

    /// Gets the value of DeviceDescriptionLen
    pub fn get_device_description_len(&self) -> Option<&u32> {
        self.device_description_len.as_ref()
    }

    /// Sets the value of IRQAffinity
    pub fn set_irqaffinity(&mut self, value: u64) {
        self.irqaffinity = Some(value);
    }

    /// Gets the value of IRQAffinity
    pub fn get_irqaffinity(&self) -> Option<&u64> {
        self.irqaffinity.as_ref()
    }

    /// Sets the value of IRQNum
    pub fn set_irqnum(&mut self, value: u32) {
        self.irqnum = Some(value);
    }

    /// Gets the value of IRQNum
    pub fn get_irqnum(&self) -> Option<&u32> {
        self.irqnum.as_ref()
    }
}

