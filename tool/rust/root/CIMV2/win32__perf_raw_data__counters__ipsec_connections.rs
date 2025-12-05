// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_IPsecConnections struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_IPsecConnections {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "Maxnumberofconnectionssinceboot")]
    pub maxnumberofconnectionssinceboot: Option<u32>,

/// 
    #[serde(rename = "Numberoffailedauthentications")]
    pub numberoffailedauthentications: Option<u64>,

/// 
    #[serde(rename = "TotalBytesInsincestart")]
    pub total_bytes_insincestart: Option<u64>,

/// 
    #[serde(rename = "TotalBytesOutsincestart")]
    pub total_bytes_outsincestart: Option<u64>,

/// 
    #[serde(rename = "TotalNumbercurrentConnections")]
    pub total_numbercurrent_connections: Option<u32>,

/// 
    #[serde(rename = "Totalnumberofcumulativeconnectionssinceboot")]
    pub totalnumberofcumulativeconnectionssinceboot: Option<u64>,
}

impl Win32_PerfRawData_Counters_IPsecConnections {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            maxnumberofconnectionssinceboot: None,
            numberoffailedauthentications: None,
            total_bytes_insincestart: None,
            total_bytes_outsincestart: None,
            total_numbercurrent_connections: None,
            totalnumberofcumulativeconnectionssinceboot: None,
        }
    }


    /// Sets the value of Maxnumberofconnectionssinceboot
    pub fn set_maxnumberofconnectionssinceboot(&mut self, value: u32) {
        self.maxnumberofconnectionssinceboot = Some(value);
    }

    /// Gets the value of Maxnumberofconnectionssinceboot
    pub fn get_maxnumberofconnectionssinceboot(&self) -> Option<&u32> {
        self.maxnumberofconnectionssinceboot.as_ref()
    }

    /// Sets the value of Numberoffailedauthentications
    pub fn set_numberoffailedauthentications(&mut self, value: u64) {
        self.numberoffailedauthentications = Some(value);
    }

    /// Gets the value of Numberoffailedauthentications
    pub fn get_numberoffailedauthentications(&self) -> Option<&u64> {
        self.numberoffailedauthentications.as_ref()
    }

    /// Sets the value of TotalBytesInsincestart
    pub fn set_total_bytes_insincestart(&mut self, value: u64) {
        self.total_bytes_insincestart = Some(value);
    }

    /// Gets the value of TotalBytesInsincestart
    pub fn get_total_bytes_insincestart(&self) -> Option<&u64> {
        self.total_bytes_insincestart.as_ref()
    }

    /// Sets the value of TotalBytesOutsincestart
    pub fn set_total_bytes_outsincestart(&mut self, value: u64) {
        self.total_bytes_outsincestart = Some(value);
    }

    /// Gets the value of TotalBytesOutsincestart
    pub fn get_total_bytes_outsincestart(&self) -> Option<&u64> {
        self.total_bytes_outsincestart.as_ref()
    }

    /// Sets the value of TotalNumbercurrentConnections
    pub fn set_total_numbercurrent_connections(&mut self, value: u32) {
        self.total_numbercurrent_connections = Some(value);
    }

    /// Gets the value of TotalNumbercurrentConnections
    pub fn get_total_numbercurrent_connections(&self) -> Option<&u32> {
        self.total_numbercurrent_connections.as_ref()
    }

    /// Sets the value of Totalnumberofcumulativeconnectionssinceboot
    pub fn set_totalnumberofcumulativeconnectionssinceboot(&mut self, value: u64) {
        self.totalnumberofcumulativeconnectionssinceboot = Some(value);
    }

    /// Gets the value of Totalnumberofcumulativeconnectionssinceboot
    pub fn get_totalnumberofcumulativeconnectionssinceboot(&self) -> Option<&u64> {
        self.totalnumberofcumulativeconnectionssinceboot.as_ref()
    }
}

