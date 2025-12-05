// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WSAT_TraceEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WSAT_TraceEvent {
    #[serde(flatten)]
    pub base: WSAT_TraceProvider,
}

impl WSAT_TraceEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: WSAT_TraceProvider::new(),
        }
    }

}

