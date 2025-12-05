// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SPACES_StorageDepartureEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SPACES_StorageDepartureEvent {
    #[serde(flatten)]
    pub base: WSP_StorageDepartureEvent,
}

impl SPACES_StorageDepartureEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: WSP_StorageDepartureEvent::new(),
        }
    }

}

