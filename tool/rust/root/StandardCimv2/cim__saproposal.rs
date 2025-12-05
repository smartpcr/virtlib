// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_SAProposal struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_SAProposal {
    #[serde(flatten)]
    pub base: CIM_ScopedSettingData,
}

impl CIM_SAProposal {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ScopedSettingData::new(),
        }
    }

}

