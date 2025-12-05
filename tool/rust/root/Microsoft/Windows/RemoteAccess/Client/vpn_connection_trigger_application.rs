// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.RemoteAccess.Client
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// VpnConnectionTriggerApplication struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VpnConnectionTriggerApplication {

/// 
    #[serde(rename = "ApplicationID")]
    pub application_id: Vec<String>,

/// 
    #[serde(rename = "ConnectionName")]
    pub connection_name: Option<String>,
}

impl VpnConnectionTriggerApplication {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            application_id: Vec::new(),
            connection_name: None,
        }
    }


    /// Sets the value of ApplicationID
    pub fn set_application_id(&mut self, value: Vec<String>) {
        self.application_id = value;
    }

    /// Gets the value of ApplicationID
    pub fn get_application_id(&self) -> &Vec<String> {
        &self.application_id
    }

    /// Sets the value of ConnectionName
    pub fn set_connection_name(&mut self, value: String) {
        self.connection_name = Some(value);
    }

    /// Gets the value of ConnectionName
    pub fn get_connection_name(&self) -> Option<&String> {
        self.connection_name.as_ref()
    }
}

