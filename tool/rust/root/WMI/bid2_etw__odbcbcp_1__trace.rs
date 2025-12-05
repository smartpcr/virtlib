// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Bid2Etw_ODBCBCP_1_Trace struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Bid2Etw_ODBCBCP_1_Trace {
    #[serde(flatten)]
    pub base: Bid2Etw_ODBCBCP_1,
}

impl Bid2Etw_ODBCBCP_1_Trace {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Bid2Etw_ODBCBCP_1::new(),
        }
    }

}

