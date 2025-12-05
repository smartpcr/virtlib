// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetServiceStartFailedNone struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetServiceStartFailedNone {
    #[serde(flatten)]
    pub base: MSFT_SCMEventLogEvent,

/// 
    #[serde(rename = "NonExistingService")]
    pub non_existing_service: Option<String>,

/// 
    #[serde(rename = "Service")]
    pub service: Option<String>,
}

impl MSFT_NetServiceStartFailedNone {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_SCMEventLogEvent::new(),
            non_existing_service: None,
            service: None,
        }
    }


    /// Sets the value of NonExistingService
    pub fn set_non_existing_service(&mut self, value: String) {
        self.non_existing_service = Some(value);
    }

    /// Gets the value of NonExistingService
    pub fn get_non_existing_service(&self) -> Option<&String> {
        self.non_existing_service.as_ref()
    }

    /// Sets the value of Service
    pub fn set_service(&mut self, value: String) {
        self.service = Some(value);
    }

    /// Gets the value of Service
    pub fn get_service(&self) -> Option<&String> {
        self.service.as_ref()
    }
}

