// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_NetworkConnectionDiagnosticInformation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_NetworkConnectionDiagnosticInformation {

/// The round trip time for the Ping request.
    #[serde(rename = "RoundTripTime")]
    pub round_trip_time: Option<u32>,
}

impl Msvm_NetworkConnectionDiagnosticInformation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            round_trip_time: None,
        }
    }


    /// Sets the value of RoundTripTime
    pub fn set_round_trip_time(&mut self, value: u32) {
        self.round_trip_time = Some(value);
    }

    /// Gets the value of RoundTripTime
    pub fn get_round_trip_time(&self) -> Option<&u32> {
        self.round_trip_time.as_ref()
    }
}

