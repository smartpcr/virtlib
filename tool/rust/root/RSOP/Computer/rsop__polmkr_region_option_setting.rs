// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_PolmkrRegionOptionSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_PolmkrRegionOptionSetting {
    #[serde(flatten)]
    pub base: RSOP_PolmkrProSetting,
}

impl RSOP_PolmkrRegionOptionSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: RSOP_PolmkrProSetting::new(),
        }
    }

}

