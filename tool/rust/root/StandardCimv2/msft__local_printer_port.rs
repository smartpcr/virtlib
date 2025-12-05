// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_LocalPrinterPort struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_LocalPrinterPort {
    #[serde(flatten)]
    pub base: MSFT_PrinterPort,
}

impl MSFT_LocalPrinterPort {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_PrinterPort::new(),
        }
    }

}

