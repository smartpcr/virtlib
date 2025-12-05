// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WSP_StorageArrivalEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WSP_StorageArrivalEvent {
    #[serde(flatten)]
    pub base: MSFT_StorageArrivalEvent,
}

impl WSP_StorageArrivalEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageArrivalEvent::new(),
        }
    }

}

