// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_LogicalPort struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_LogicalPort {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,

/// 
    #[serde(rename = "MaxSpeed")]
    pub max_speed: Option<u64>,

/// 
    #[serde(rename = "OtherPortType")]
    pub other_port_type: Option<String>,

/// 
    #[serde(rename = "PortType")]
    pub port_type: Option<u16>,

/// 
    #[serde(rename = "RequestedSpeed")]
    pub requested_speed: Option<u64>,

/// 
    #[serde(rename = "Speed")]
    pub speed: Option<u64>,

/// 
    #[serde(rename = "UsageRestriction")]
    pub usage_restriction: Option<u16>,
}

impl CIM_LogicalPort {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
            max_speed: None,
            other_port_type: None,
            port_type: None,
            requested_speed: None,
            speed: None,
            usage_restriction: None,
        }
    }


    /// Sets the value of MaxSpeed
    pub fn set_max_speed(&mut self, value: u64) {
        self.max_speed = Some(value);
    }

    /// Gets the value of MaxSpeed
    pub fn get_max_speed(&self) -> Option<&u64> {
        self.max_speed.as_ref()
    }

    /// Sets the value of OtherPortType
    pub fn set_other_port_type(&mut self, value: String) {
        self.other_port_type = Some(value);
    }

    /// Gets the value of OtherPortType
    pub fn get_other_port_type(&self) -> Option<&String> {
        self.other_port_type.as_ref()
    }

    /// Sets the value of PortType
    pub fn set_port_type(&mut self, value: u16) {
        self.port_type = Some(value);
    }

    /// Gets the value of PortType
    pub fn get_port_type(&self) -> Option<&u16> {
        self.port_type.as_ref()
    }

    /// Sets the value of RequestedSpeed
    pub fn set_requested_speed(&mut self, value: u64) {
        self.requested_speed = Some(value);
    }

    /// Gets the value of RequestedSpeed
    pub fn get_requested_speed(&self) -> Option<&u64> {
        self.requested_speed.as_ref()
    }

    /// Sets the value of Speed
    pub fn set_speed(&mut self, value: u64) {
        self.speed = Some(value);
    }

    /// Gets the value of Speed
    pub fn get_speed(&self) -> Option<&u64> {
        self.speed.as_ref()
    }

    /// Sets the value of UsageRestriction
    pub fn set_usage_restriction(&mut self, value: u16) {
        self.usage_restriction = Some(value);
    }

    /// Gets the value of UsageRestriction
    pub fn get_usage_restriction(&self) -> Option<&u16> {
        self.usage_restriction.as_ref()
    }
}

