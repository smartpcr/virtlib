// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_3DPrinter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_3DPrinter {
    #[serde(flatten)]
    pub base: MSFT_Printer,
}

impl MSFT_3DPrinter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_Printer::new(),
        }
    }

}

