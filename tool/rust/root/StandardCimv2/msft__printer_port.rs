// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_PrinterPort struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_PrinterPort {
    #[serde(flatten)]
    pub base: CIM_ManagedSystemElement,

/// 
    #[serde(rename = "ComputerName")]
    pub computer_name: Option<String>,

/// 
    #[serde(rename = "PortMonitor")]
    pub port_monitor: Option<String>,
}

impl MSFT_PrinterPort {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedSystemElement::new(),
            computer_name: None,
            port_monitor: None,
        }
    }


    /// Sets the value of ComputerName
    pub fn set_computer_name(&mut self, value: String) {
        self.computer_name = Some(value);
    }

    /// Gets the value of ComputerName
    pub fn get_computer_name(&self) -> Option<&String> {
        self.computer_name.as_ref()
    }

    /// Sets the value of PortMonitor
    pub fn set_port_monitor(&mut self, value: String) {
        self.port_monitor = Some(value);
    }

    /// Gets the value of PortMonitor
    pub fn get_port_monitor(&self) -> Option<&String> {
        self.port_monitor.as_ref()
    }
}

