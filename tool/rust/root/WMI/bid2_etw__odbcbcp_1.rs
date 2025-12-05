// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Bid2Etw_ODBCBCP_1 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Bid2Etw_ODBCBCP_1 {
    #[serde(flatten)]
    pub base: EventTrace,
}

impl Bid2Etw_ODBCBCP_1 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: EventTrace::new(),
        }
    }

}

