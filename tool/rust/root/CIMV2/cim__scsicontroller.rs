// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_SCSIController struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_SCSIController {
    #[serde(flatten)]
    pub base: CIM_Controller,

/// 
    #[serde(rename = "ControllerTimeouts")]
    pub controller_timeouts: Option<u32>,

/// 
    #[serde(rename = "MaxDataWidth")]
    pub max_data_width: Option<u32>,

/// 
    #[serde(rename = "MaxTransferRate")]
    pub max_transfer_rate: Option<u64>,

/// 
    #[serde(rename = "ProtectionManagement")]
    pub protection_management: Option<u16>,
}

impl CIM_SCSIController {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Controller::new(),
            controller_timeouts: None,
            max_data_width: None,
            max_transfer_rate: None,
            protection_management: None,
        }
    }


    /// Sets the value of ControllerTimeouts
    pub fn set_controller_timeouts(&mut self, value: u32) {
        self.controller_timeouts = Some(value);
    }

    /// Gets the value of ControllerTimeouts
    pub fn get_controller_timeouts(&self) -> Option<&u32> {
        self.controller_timeouts.as_ref()
    }

    /// Sets the value of MaxDataWidth
    pub fn set_max_data_width(&mut self, value: u32) {
        self.max_data_width = Some(value);
    }

    /// Gets the value of MaxDataWidth
    pub fn get_max_data_width(&self) -> Option<&u32> {
        self.max_data_width.as_ref()
    }

    /// Sets the value of MaxTransferRate
    pub fn set_max_transfer_rate(&mut self, value: u64) {
        self.max_transfer_rate = Some(value);
    }

    /// Gets the value of MaxTransferRate
    pub fn get_max_transfer_rate(&self) -> Option<&u64> {
        self.max_transfer_rate.as_ref()
    }

    /// Sets the value of ProtectionManagement
    pub fn set_protection_management(&mut self, value: u16) {
        self.protection_management = Some(value);
    }

    /// Gets the value of ProtectionManagement
    pub fn get_protection_management(&self) -> Option<&u16> {
        self.protection_management.as_ref()
    }
}

