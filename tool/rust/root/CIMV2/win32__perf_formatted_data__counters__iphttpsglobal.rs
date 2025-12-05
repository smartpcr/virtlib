// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_IPHTTPSGlobal struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_IPHTTPSGlobal {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "DropsNeighborresolutiontimeouts")]
    pub drops_neighborresolutiontimeouts: Option<u64>,

/// 
    #[serde(rename = "ErrorsAuthenticationErrors")]
    pub errors_authentication_errors: Option<u64>,

/// 
    #[serde(rename = "ErrorsReceiveerrorsontheserver")]
    pub errors_receiveerrorsontheserver: Option<u64>,

/// 
    #[serde(rename = "ErrorsTransmiterrorsontheserver")]
    pub errors_transmiterrorsontheserver: Option<u64>,

/// 
    #[serde(rename = "InTotalbytesreceived")]
    pub in_totalbytesreceived: Option<u64>,

/// 
    #[serde(rename = "InTotalpacketsreceived")]
    pub in_totalpacketsreceived: Option<u64>,

/// 
    #[serde(rename = "OutTotalbytesforwarded")]
    pub out_totalbytesforwarded: Option<u64>,

/// 
    #[serde(rename = "OutTotalbytessent")]
    pub out_totalbytessent: Option<u64>,

/// 
    #[serde(rename = "OutTotalpacketssent")]
    pub out_totalpacketssent: Option<u64>,

/// 
    #[serde(rename = "SessionsTotalsessions")]
    pub sessions_totalsessions: Option<u64>,
}

impl Win32_PerfFormattedData_Counters_IPHTTPSGlobal {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            drops_neighborresolutiontimeouts: None,
            errors_authentication_errors: None,
            errors_receiveerrorsontheserver: None,
            errors_transmiterrorsontheserver: None,
            in_totalbytesreceived: None,
            in_totalpacketsreceived: None,
            out_totalbytesforwarded: None,
            out_totalbytessent: None,
            out_totalpacketssent: None,
            sessions_totalsessions: None,
        }
    }


    /// Sets the value of DropsNeighborresolutiontimeouts
    pub fn set_drops_neighborresolutiontimeouts(&mut self, value: u64) {
        self.drops_neighborresolutiontimeouts = Some(value);
    }

    /// Gets the value of DropsNeighborresolutiontimeouts
    pub fn get_drops_neighborresolutiontimeouts(&self) -> Option<&u64> {
        self.drops_neighborresolutiontimeouts.as_ref()
    }

    /// Sets the value of ErrorsAuthenticationErrors
    pub fn set_errors_authentication_errors(&mut self, value: u64) {
        self.errors_authentication_errors = Some(value);
    }

    /// Gets the value of ErrorsAuthenticationErrors
    pub fn get_errors_authentication_errors(&self) -> Option<&u64> {
        self.errors_authentication_errors.as_ref()
    }

    /// Sets the value of ErrorsReceiveerrorsontheserver
    pub fn set_errors_receiveerrorsontheserver(&mut self, value: u64) {
        self.errors_receiveerrorsontheserver = Some(value);
    }

    /// Gets the value of ErrorsReceiveerrorsontheserver
    pub fn get_errors_receiveerrorsontheserver(&self) -> Option<&u64> {
        self.errors_receiveerrorsontheserver.as_ref()
    }

    /// Sets the value of ErrorsTransmiterrorsontheserver
    pub fn set_errors_transmiterrorsontheserver(&mut self, value: u64) {
        self.errors_transmiterrorsontheserver = Some(value);
    }

    /// Gets the value of ErrorsTransmiterrorsontheserver
    pub fn get_errors_transmiterrorsontheserver(&self) -> Option<&u64> {
        self.errors_transmiterrorsontheserver.as_ref()
    }

    /// Sets the value of InTotalbytesreceived
    pub fn set_in_totalbytesreceived(&mut self, value: u64) {
        self.in_totalbytesreceived = Some(value);
    }

    /// Gets the value of InTotalbytesreceived
    pub fn get_in_totalbytesreceived(&self) -> Option<&u64> {
        self.in_totalbytesreceived.as_ref()
    }

    /// Sets the value of InTotalpacketsreceived
    pub fn set_in_totalpacketsreceived(&mut self, value: u64) {
        self.in_totalpacketsreceived = Some(value);
    }

    /// Gets the value of InTotalpacketsreceived
    pub fn get_in_totalpacketsreceived(&self) -> Option<&u64> {
        self.in_totalpacketsreceived.as_ref()
    }

    /// Sets the value of OutTotalbytesforwarded
    pub fn set_out_totalbytesforwarded(&mut self, value: u64) {
        self.out_totalbytesforwarded = Some(value);
    }

    /// Gets the value of OutTotalbytesforwarded
    pub fn get_out_totalbytesforwarded(&self) -> Option<&u64> {
        self.out_totalbytesforwarded.as_ref()
    }

    /// Sets the value of OutTotalbytessent
    pub fn set_out_totalbytessent(&mut self, value: u64) {
        self.out_totalbytessent = Some(value);
    }

    /// Gets the value of OutTotalbytessent
    pub fn get_out_totalbytessent(&self) -> Option<&u64> {
        self.out_totalbytessent.as_ref()
    }

    /// Sets the value of OutTotalpacketssent
    pub fn set_out_totalpacketssent(&mut self, value: u64) {
        self.out_totalpacketssent = Some(value);
    }

    /// Gets the value of OutTotalpacketssent
    pub fn get_out_totalpacketssent(&self) -> Option<&u64> {
        self.out_totalpacketssent.as_ref()
    }

    /// Sets the value of SessionsTotalsessions
    pub fn set_sessions_totalsessions(&mut self, value: u64) {
        self.sessions_totalsessions = Some(value);
    }

    /// Gets the value of SessionsTotalsessions
    pub fn get_sessions_totalsessions(&self) -> Option<&u64> {
        self.sessions_totalsessions.as_ref()
    }
}

