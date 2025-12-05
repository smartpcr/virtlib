// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source TCPIPPrinterPort_Protocol
//////////////////////////////////////////////

/// TCPIPPrinterPort_Protocol enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum TCPIPPrinterPort_Protocol {
    /// Raw
    #[serde(rename = "Raw")]
    Raw = 0,
    /// Lpr
    #[serde(rename = "Lpr")]
    Lpr = 1,
}

impl Default for TCPIPPrinterPort_Protocol {
    fn default() -> Self {
        Self::Raw
    }
}

