// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetImPlatAdapter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetImPlatAdapter {
    #[serde(flatten)]
    pub base: CIM_EnabledLogicalElement,

/// 
    #[serde(rename = "FailureReason")]
    pub failure_reason: Option<u32>,

/// 
    #[serde(rename = "InterfaceDescription")]
    pub interface_description: Option<String>,

/// 
    #[serde(rename = "NumberOfFailures")]
    pub number_of_failures: Option<u32>,

/// 
    #[serde(rename = "ReceiveLinkSpeed")]
    pub receive_link_speed: Option<u64>,

/// 
    #[serde(rename = "Team")]
    pub team: Option<String>,

/// 
    #[serde(rename = "TransmitLinkSpeed")]
    pub transmit_link_speed: Option<u64>,
}

impl MSFT_NetImPlatAdapter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_EnabledLogicalElement::new(),
            failure_reason: None,
            interface_description: None,
            number_of_failures: None,
            receive_link_speed: None,
            team: None,
            transmit_link_speed: None,
        }
    }


    /// Sets the value of FailureReason
    pub fn set_failure_reason(&mut self, value: u32) {
        self.failure_reason = Some(value);
    }

    /// Gets the value of FailureReason
    pub fn get_failure_reason(&self) -> Option<&u32> {
        self.failure_reason.as_ref()
    }

    /// Sets the value of InterfaceDescription
    pub fn set_interface_description(&mut self, value: String) {
        self.interface_description = Some(value);
    }

    /// Gets the value of InterfaceDescription
    pub fn get_interface_description(&self) -> Option<&String> {
        self.interface_description.as_ref()
    }

    /// Sets the value of NumberOfFailures
    pub fn set_number_of_failures(&mut self, value: u32) {
        self.number_of_failures = Some(value);
    }

    /// Gets the value of NumberOfFailures
    pub fn get_number_of_failures(&self) -> Option<&u32> {
        self.number_of_failures.as_ref()
    }

    /// Sets the value of ReceiveLinkSpeed
    pub fn set_receive_link_speed(&mut self, value: u64) {
        self.receive_link_speed = Some(value);
    }

    /// Gets the value of ReceiveLinkSpeed
    pub fn get_receive_link_speed(&self) -> Option<&u64> {
        self.receive_link_speed.as_ref()
    }

    /// Sets the value of Team
    pub fn set_team(&mut self, value: String) {
        self.team = Some(value);
    }

    /// Gets the value of Team
    pub fn get_team(&self) -> Option<&String> {
        self.team.as_ref()
    }

    /// Sets the value of TransmitLinkSpeed
    pub fn set_transmit_link_speed(&mut self, value: u64) {
        self.transmit_link_speed = Some(value);
    }

    /// Gets the value of TransmitLinkSpeed
    pub fn get_transmit_link_speed(&self) -> Option<&u64> {
        self.transmit_link_speed.as_ref()
    }
}

