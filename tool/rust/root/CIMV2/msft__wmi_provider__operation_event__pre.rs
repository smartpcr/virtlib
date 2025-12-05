// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msft_WmiProvider_OperationEvent_Pre struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msft_WmiProvider_OperationEvent_Pre {
    #[serde(flatten)]
    pub base: Msft_WmiProvider_OperationEvent,
}

impl Msft_WmiProvider_OperationEvent_Pre {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msft_WmiProvider_OperationEvent::new(),
        }
    }

}

