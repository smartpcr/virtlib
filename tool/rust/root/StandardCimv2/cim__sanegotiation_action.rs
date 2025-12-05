// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_SANegotiationAction struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_SANegotiationAction {
    #[serde(flatten)]
    pub base: CIM_SAAction,

/// 
    #[serde(rename = "IdleDurationSeconds")]
    pub idle_duration_seconds: Option<u64>,

/// 
    #[serde(rename = "MinLifetimeKilobytes")]
    pub min_lifetime_kilobytes: Option<u64>,

/// 
    #[serde(rename = "MinLifetimeSeconds")]
    pub min_lifetime_seconds: Option<u64>,
}

impl CIM_SANegotiationAction {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SAAction::new(),
            idle_duration_seconds: None,
            min_lifetime_kilobytes: None,
            min_lifetime_seconds: None,
        }
    }


    /// Sets the value of IdleDurationSeconds
    pub fn set_idle_duration_seconds(&mut self, value: u64) {
        self.idle_duration_seconds = Some(value);
    }

    /// Gets the value of IdleDurationSeconds
    pub fn get_idle_duration_seconds(&self) -> Option<&u64> {
        self.idle_duration_seconds.as_ref()
    }

    /// Sets the value of MinLifetimeKilobytes
    pub fn set_min_lifetime_kilobytes(&mut self, value: u64) {
        self.min_lifetime_kilobytes = Some(value);
    }

    /// Gets the value of MinLifetimeKilobytes
    pub fn get_min_lifetime_kilobytes(&self) -> Option<&u64> {
        self.min_lifetime_kilobytes.as_ref()
    }

    /// Sets the value of MinLifetimeSeconds
    pub fn set_min_lifetime_seconds(&mut self, value: u64) {
        self.min_lifetime_seconds = Some(value);
    }

    /// Gets the value of MinLifetimeSeconds
    pub fn get_min_lifetime_seconds(&self) -> Option<&u64> {
        self.min_lifetime_seconds.as_ref()
    }
}

