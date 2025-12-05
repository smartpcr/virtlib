// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_LprPrinterPort struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_LprPrinterPort {
    #[serde(flatten)]
    pub base: MSFT_PrinterPort,

/// 
    #[serde(rename = "HostName")]
    pub host_name: Option<String>,

/// 
    #[serde(rename = "PrinterName")]
    pub printer_name: Option<String>,
}

impl MSFT_LprPrinterPort {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_PrinterPort::new(),
            host_name: None,
            printer_name: None,
        }
    }


    /// Sets the value of HostName
    pub fn set_host_name(&mut self, value: String) {
        self.host_name = Some(value);
    }

    /// Gets the value of HostName
    pub fn get_host_name(&self) -> Option<&String> {
        self.host_name.as_ref()
    }

    /// Sets the value of PrinterName
    pub fn set_printer_name(&mut self, value: String) {
        self.printer_name = Some(value);
    }

    /// Gets the value of PrinterName
    pub fn get_printer_name(&self) -> Option<&String> {
        self.printer_name.as_ref()
    }
}

