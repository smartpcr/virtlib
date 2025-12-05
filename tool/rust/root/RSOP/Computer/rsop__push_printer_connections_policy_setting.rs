// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_PushPrinterConnectionsPolicySetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_PushPrinterConnectionsPolicySetting {
    #[serde(flatten)]
    pub base: RSOP_PolicySetting,

/// Whether the the connection is applied per machine, per user1 = User, 2 = Machine.
    #[serde(rename = "ConnectionType")]
    pub connection_type: Option<PushPrinterConnectionsPolicySetting_ConnectionType>,

/// Indicates whether the print connectionhas been deleted.
    #[serde(rename = "deleted")]
    pub deleted: Option<bool>,

/// Printer name,
    #[serde(rename = "printerName")]
    pub printer_name: Option<String>,

/// The final result of pushed printer connection. 0 indicate success
    #[serde(rename = "PushResult")]
    pub push_result: Option<u32>,

/// Short server name
    #[serde(rename = "serverName")]
    pub server_name: Option<String>,

/// Short server name
    #[serde(rename = "uncName")]
    pub unc_name: Option<String>,
}

impl RSOP_PushPrinterConnectionsPolicySetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: RSOP_PolicySetting::new(),
            connection_type: None,
            deleted: None,
            printer_name: None,
            push_result: None,
            server_name: None,
            unc_name: None,
        }
    }


    /// Sets the value of ConnectionType
    pub fn set_connection_type(&mut self, value: PushPrinterConnectionsPolicySetting_ConnectionType) {
        self.connection_type = Some(value);
    }

    /// Gets the value of ConnectionType
    pub fn get_connection_type(&self) -> Option<&PushPrinterConnectionsPolicySetting_ConnectionType> {
        self.connection_type.as_ref()
    }

    /// Sets the value of deleted
    pub fn set_deleted(&mut self, value: bool) {
        self.deleted = Some(value);
    }

    /// Gets the value of deleted
    pub fn get_deleted(&self) -> Option<&bool> {
        self.deleted.as_ref()
    }

    /// Sets the value of printerName
    pub fn set_printer_name(&mut self, value: String) {
        self.printer_name = Some(value);
    }

    /// Gets the value of printerName
    pub fn get_printer_name(&self) -> Option<&String> {
        self.printer_name.as_ref()
    }

    /// Sets the value of PushResult
    pub fn set_push_result(&mut self, value: u32) {
        self.push_result = Some(value);
    }

    /// Gets the value of PushResult
    pub fn get_push_result(&self) -> Option<&u32> {
        self.push_result.as_ref()
    }

    /// Sets the value of serverName
    pub fn set_server_name(&mut self, value: String) {
        self.server_name = Some(value);
    }

    /// Gets the value of serverName
    pub fn get_server_name(&self) -> Option<&String> {
        self.server_name.as_ref()
    }

    /// Sets the value of uncName
    pub fn set_unc_name(&mut self, value: String) {
        self.unc_name = Some(value);
    }

    /// Gets the value of uncName
    pub fn get_unc_name(&self) -> Option<&String> {
        self.unc_name.as_ref()
    }
}

