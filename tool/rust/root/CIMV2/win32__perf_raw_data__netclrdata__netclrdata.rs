// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_NETCLRData_NETCLRData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_NETCLRData_NETCLRData {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "SqlClientCurrentNumberconnectionpools")]
    pub sql_client_current_numberconnectionpools: Option<u32>,

/// 
    #[serde(rename = "SqlClientCurrentNumberpooledandnonpooledconnections")]
    pub sql_client_current_numberpooledandnonpooledconnections: Option<u32>,

/// 
    #[serde(rename = "SqlClientCurrentNumberpooledconnections")]
    pub sql_client_current_numberpooledconnections: Option<u32>,

/// 
    #[serde(rename = "SqlClientPeakNumberpooledconnections")]
    pub sql_client_peak_numberpooledconnections: Option<u32>,

/// 
    #[serde(rename = "SqlClientTotalNumberfailedcommands")]
    pub sql_client_total_numberfailedcommands: Option<u32>,

/// 
    #[serde(rename = "SqlClientTotalNumberfailedconnects")]
    pub sql_client_total_numberfailedconnects: Option<u32>,
}

impl Win32_PerfRawData_NETCLRData_NETCLRData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            sql_client_current_numberconnectionpools: None,
            sql_client_current_numberpooledandnonpooledconnections: None,
            sql_client_current_numberpooledconnections: None,
            sql_client_peak_numberpooledconnections: None,
            sql_client_total_numberfailedcommands: None,
            sql_client_total_numberfailedconnects: None,
        }
    }


    /// Sets the value of SqlClientCurrentNumberconnectionpools
    pub fn set_sql_client_current_numberconnectionpools(&mut self, value: u32) {
        self.sql_client_current_numberconnectionpools = Some(value);
    }

    /// Gets the value of SqlClientCurrentNumberconnectionpools
    pub fn get_sql_client_current_numberconnectionpools(&self) -> Option<&u32> {
        self.sql_client_current_numberconnectionpools.as_ref()
    }

    /// Sets the value of SqlClientCurrentNumberpooledandnonpooledconnections
    pub fn set_sql_client_current_numberpooledandnonpooledconnections(&mut self, value: u32) {
        self.sql_client_current_numberpooledandnonpooledconnections = Some(value);
    }

    /// Gets the value of SqlClientCurrentNumberpooledandnonpooledconnections
    pub fn get_sql_client_current_numberpooledandnonpooledconnections(&self) -> Option<&u32> {
        self.sql_client_current_numberpooledandnonpooledconnections.as_ref()
    }

    /// Sets the value of SqlClientCurrentNumberpooledconnections
    pub fn set_sql_client_current_numberpooledconnections(&mut self, value: u32) {
        self.sql_client_current_numberpooledconnections = Some(value);
    }

    /// Gets the value of SqlClientCurrentNumberpooledconnections
    pub fn get_sql_client_current_numberpooledconnections(&self) -> Option<&u32> {
        self.sql_client_current_numberpooledconnections.as_ref()
    }

    /// Sets the value of SqlClientPeakNumberpooledconnections
    pub fn set_sql_client_peak_numberpooledconnections(&mut self, value: u32) {
        self.sql_client_peak_numberpooledconnections = Some(value);
    }

    /// Gets the value of SqlClientPeakNumberpooledconnections
    pub fn get_sql_client_peak_numberpooledconnections(&self) -> Option<&u32> {
        self.sql_client_peak_numberpooledconnections.as_ref()
    }

    /// Sets the value of SqlClientTotalNumberfailedcommands
    pub fn set_sql_client_total_numberfailedcommands(&mut self, value: u32) {
        self.sql_client_total_numberfailedcommands = Some(value);
    }

    /// Gets the value of SqlClientTotalNumberfailedcommands
    pub fn get_sql_client_total_numberfailedcommands(&self) -> Option<&u32> {
        self.sql_client_total_numberfailedcommands.as_ref()
    }

    /// Sets the value of SqlClientTotalNumberfailedconnects
    pub fn set_sql_client_total_numberfailedconnects(&mut self, value: u32) {
        self.sql_client_total_numberfailedconnects = Some(value);
    }

    /// Gets the value of SqlClientTotalNumberfailedconnects
    pub fn get_sql_client_total_numberfailedconnects(&self) -> Option<&u32> {
        self.sql_client_total_numberfailedconnects.as_ref()
    }
}

