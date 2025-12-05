// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ClientApplicationSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ClientApplicationSetting {

/// 
    #[serde(rename = "Application")]
    pub application: Option<Win32_DCOMApplication>,

/// 
    #[serde(rename = "Client")]
    pub client: Option<CIM_DataFile>,
}

impl Win32_ClientApplicationSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            application: None,
            client: None,
        }
    }


    /// Sets the value of Application
    pub fn set_application(&mut self, value: Win32_DCOMApplication) {
        self.application = Some(value);
    }

    /// Gets the value of Application
    pub fn get_application(&self) -> Option<&Win32_DCOMApplication> {
        self.application.as_ref()
    }

    /// Sets the value of Client
    pub fn set_client(&mut self, value: CIM_DataFile) {
        self.client = Some(value);
    }

    /// Gets the value of Client
    pub fn get_client(&self) -> Option<&CIM_DataFile> {
        self.client.as_ref()
    }
}

