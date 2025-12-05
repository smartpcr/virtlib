// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_PolmkrPowerOptionsV2Setting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_PolmkrPowerOptionsV2Setting {
    #[serde(flatten)]
    pub base: RSOP_PolmkrPowerSetting,
}

impl RSOP_PolmkrPowerOptionsV2Setting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: RSOP_PolmkrPowerSetting::new(),
        }
    }

}

