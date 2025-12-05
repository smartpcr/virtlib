// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_IPHTTPSSession struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_IPHTTPSSession {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "Bytesreceivedonthissession")]
    pub bytesreceivedonthissession: Option<u64>,

/// 
    #[serde(rename = "Bytessentonthissession")]
    pub bytessentonthissession: Option<u64>,

/// 
    #[serde(rename = "DurationDurationofthesessionSeconds")]
    pub duration_durationofthesession_seconds: Option<u64>,

/// 
    #[serde(rename = "ErrorsReceiveerrorsonthissession")]
    pub errors_receiveerrorsonthissession: Option<u64>,

/// 
    #[serde(rename = "ErrorsTransmiterrorsonthissession")]
    pub errors_transmiterrorsonthissession: Option<u64>,

/// 
    #[serde(rename = "Packetsreceivedonthissession")]
    pub packetsreceivedonthissession: Option<u64>,

/// 
    #[serde(rename = "Packetssentonthissession")]
    pub packetssentonthissession: Option<u64>,
}

impl Win32_PerfFormattedData_Counters_IPHTTPSSession {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            bytesreceivedonthissession: None,
            bytessentonthissession: None,
            duration_durationofthesession_seconds: None,
            errors_receiveerrorsonthissession: None,
            errors_transmiterrorsonthissession: None,
            packetsreceivedonthissession: None,
            packetssentonthissession: None,
        }
    }


    /// Sets the value of Bytesreceivedonthissession
    pub fn set_bytesreceivedonthissession(&mut self, value: u64) {
        self.bytesreceivedonthissession = Some(value);
    }

    /// Gets the value of Bytesreceivedonthissession
    pub fn get_bytesreceivedonthissession(&self) -> Option<&u64> {
        self.bytesreceivedonthissession.as_ref()
    }

    /// Sets the value of Bytessentonthissession
    pub fn set_bytessentonthissession(&mut self, value: u64) {
        self.bytessentonthissession = Some(value);
    }

    /// Gets the value of Bytessentonthissession
    pub fn get_bytessentonthissession(&self) -> Option<&u64> {
        self.bytessentonthissession.as_ref()
    }

    /// Sets the value of DurationDurationofthesessionSeconds
    pub fn set_duration_durationofthesession_seconds(&mut self, value: u64) {
        self.duration_durationofthesession_seconds = Some(value);
    }

    /// Gets the value of DurationDurationofthesessionSeconds
    pub fn get_duration_durationofthesession_seconds(&self) -> Option<&u64> {
        self.duration_durationofthesession_seconds.as_ref()
    }

    /// Sets the value of ErrorsReceiveerrorsonthissession
    pub fn set_errors_receiveerrorsonthissession(&mut self, value: u64) {
        self.errors_receiveerrorsonthissession = Some(value);
    }

    /// Gets the value of ErrorsReceiveerrorsonthissession
    pub fn get_errors_receiveerrorsonthissession(&self) -> Option<&u64> {
        self.errors_receiveerrorsonthissession.as_ref()
    }

    /// Sets the value of ErrorsTransmiterrorsonthissession
    pub fn set_errors_transmiterrorsonthissession(&mut self, value: u64) {
        self.errors_transmiterrorsonthissession = Some(value);
    }

    /// Gets the value of ErrorsTransmiterrorsonthissession
    pub fn get_errors_transmiterrorsonthissession(&self) -> Option<&u64> {
        self.errors_transmiterrorsonthissession.as_ref()
    }

    /// Sets the value of Packetsreceivedonthissession
    pub fn set_packetsreceivedonthissession(&mut self, value: u64) {
        self.packetsreceivedonthissession = Some(value);
    }

    /// Gets the value of Packetsreceivedonthissession
    pub fn get_packetsreceivedonthissession(&self) -> Option<&u64> {
        self.packetsreceivedonthissession.as_ref()
    }

    /// Sets the value of Packetssentonthissession
    pub fn set_packetssentonthissession(&mut self, value: u64) {
        self.packetssentonthissession = Some(value);
    }

    /// Gets the value of Packetssentonthissession
    pub fn get_packetssentonthissession(&self) -> Option<&u64> {
        self.packetssentonthissession.as_ref()
    }
}

