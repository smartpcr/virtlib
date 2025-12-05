// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetBadAccount struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetBadAccount {
    #[serde(flatten)]
    pub base: MSFT_SCMEventLogEvent,
}

impl MSFT_NetBadAccount {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_SCMEventLogEvent::new(),
        }
    }

}

