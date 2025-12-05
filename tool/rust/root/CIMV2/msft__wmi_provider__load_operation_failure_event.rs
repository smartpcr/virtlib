// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msft_WmiProvider_LoadOperationFailureEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msft_WmiProvider_LoadOperationFailureEvent {
    #[serde(flatten)]
    pub base: Msft_WmiProvider_OperationEvent,

/// 
    #[serde(rename = "Clsid")]
    pub clsid: Option<String>,

/// 
    #[serde(rename = "InProcServer")]
    pub in_proc_server: Option<bool>,

/// 
    #[serde(rename = "InProcServerPath")]
    pub in_proc_server_path: Option<String>,

/// 
    #[serde(rename = "LocalServer")]
    pub local_server: Option<bool>,

/// 
    #[serde(rename = "LocalServerPath")]
    pub local_server_path: Option<String>,

/// 
    #[serde(rename = "ResultCode")]
    pub result_code: Option<u32>,

/// 
    #[serde(rename = "ServerName")]
    pub server_name: Option<String>,

/// 
    #[serde(rename = "Synchronisation")]
    pub synchronisation: Option<u32>,

/// 
    #[serde(rename = "ThreadingModel")]
    pub threading_model: Option<u32>,
}

impl Msft_WmiProvider_LoadOperationFailureEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msft_WmiProvider_OperationEvent::new(),
            clsid: None,
            in_proc_server: None,
            in_proc_server_path: None,
            local_server: None,
            local_server_path: None,
            result_code: None,
            server_name: None,
            synchronisation: None,
            threading_model: None,
        }
    }


    /// Sets the value of Clsid
    pub fn set_clsid(&mut self, value: String) {
        self.clsid = Some(value);
    }

    /// Gets the value of Clsid
    pub fn get_clsid(&self) -> Option<&String> {
        self.clsid.as_ref()
    }

    /// Sets the value of InProcServer
    pub fn set_in_proc_server(&mut self, value: bool) {
        self.in_proc_server = Some(value);
    }

    /// Gets the value of InProcServer
    pub fn get_in_proc_server(&self) -> Option<&bool> {
        self.in_proc_server.as_ref()
    }

    /// Sets the value of InProcServerPath
    pub fn set_in_proc_server_path(&mut self, value: String) {
        self.in_proc_server_path = Some(value);
    }

    /// Gets the value of InProcServerPath
    pub fn get_in_proc_server_path(&self) -> Option<&String> {
        self.in_proc_server_path.as_ref()
    }

    /// Sets the value of LocalServer
    pub fn set_local_server(&mut self, value: bool) {
        self.local_server = Some(value);
    }

    /// Gets the value of LocalServer
    pub fn get_local_server(&self) -> Option<&bool> {
        self.local_server.as_ref()
    }

    /// Sets the value of LocalServerPath
    pub fn set_local_server_path(&mut self, value: String) {
        self.local_server_path = Some(value);
    }

    /// Gets the value of LocalServerPath
    pub fn get_local_server_path(&self) -> Option<&String> {
        self.local_server_path.as_ref()
    }

    /// Sets the value of ResultCode
    pub fn set_result_code(&mut self, value: u32) {
        self.result_code = Some(value);
    }

    /// Gets the value of ResultCode
    pub fn get_result_code(&self) -> Option<&u32> {
        self.result_code.as_ref()
    }

    /// Sets the value of ServerName
    pub fn set_server_name(&mut self, value: String) {
        self.server_name = Some(value);
    }

    /// Gets the value of ServerName
    pub fn get_server_name(&self) -> Option<&String> {
        self.server_name.as_ref()
    }

    /// Sets the value of Synchronisation
    pub fn set_synchronisation(&mut self, value: u32) {
        self.synchronisation = Some(value);
    }

    /// Gets the value of Synchronisation
    pub fn get_synchronisation(&self) -> Option<&u32> {
        self.synchronisation.as_ref()
    }

    /// Sets the value of ThreadingModel
    pub fn set_threading_model(&mut self, value: u32) {
        self.threading_model = Some(value);
    }

    /// Gets the value of ThreadingModel
    pub fn get_threading_model(&self) -> Option<&u32> {
        self.threading_model.as_ref()
    }
}

