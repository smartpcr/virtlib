// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_PolmkrLocalPrinterSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_PolmkrLocalPrinterSetting {
    #[serde(flatten)]
    pub base: RSOP_PolmkrPrinterSetting,
}

impl RSOP_PolmkrLocalPrinterSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: RSOP_PolmkrPrinterSetting::new(),
        }
    }

}

