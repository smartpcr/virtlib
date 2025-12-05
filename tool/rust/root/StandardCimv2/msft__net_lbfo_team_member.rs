// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetLbfoTeamMember struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetLbfoTeamMember {
    #[serde(flatten)]
    pub base: MSFT_NetImPlatAdapter,

/// 396
    #[serde(rename = "AdministrativeMode")]
    pub administrative_mode: Option<u32>,

/// 397
    #[serde(rename = "OperationalMode")]
    pub operational_mode: Option<u32>,
}

impl MSFT_NetLbfoTeamMember {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetImPlatAdapter::new(),
            administrative_mode: None,
            operational_mode: None,
        }
    }


    /// Sets the value of AdministrativeMode
    pub fn set_administrative_mode(&mut self, value: u32) {
        self.administrative_mode = Some(value);
    }

    /// Gets the value of AdministrativeMode
    pub fn get_administrative_mode(&self) -> Option<&u32> {
        self.administrative_mode.as_ref()
    }

    /// Sets the value of OperationalMode
    pub fn set_operational_mode(&mut self, value: u32) {
        self.operational_mode = Some(value);
    }

    /// Gets the value of OperationalMode
    pub fn get_operational_mode(&self) -> Option<&u32> {
        self.operational_mode.as_ref()
    }
}

