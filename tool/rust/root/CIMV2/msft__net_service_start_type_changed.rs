// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetServiceStartTypeChanged struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetServiceStartTypeChanged {
    #[serde(flatten)]
    pub base: MSFT_SCMEventLogEvent,

/// 
    #[serde(rename = "NewStartType")]
    pub new_start_type: Option<String>,

/// 
    #[serde(rename = "OldStartType")]
    pub old_start_type: Option<String>,

/// 
    #[serde(rename = "Service")]
    pub service: Option<String>,

/// 
    #[serde(rename = "sid")]
    pub sid: Option<String>,
}

impl MSFT_NetServiceStartTypeChanged {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_SCMEventLogEvent::new(),
            new_start_type: None,
            old_start_type: None,
            service: None,
            sid: None,
        }
    }


    /// Sets the value of NewStartType
    pub fn set_new_start_type(&mut self, value: String) {
        self.new_start_type = Some(value);
    }

    /// Gets the value of NewStartType
    pub fn get_new_start_type(&self) -> Option<&String> {
        self.new_start_type.as_ref()
    }

    /// Sets the value of OldStartType
    pub fn set_old_start_type(&mut self, value: String) {
        self.old_start_type = Some(value);
    }

    /// Gets the value of OldStartType
    pub fn get_old_start_type(&self) -> Option<&String> {
        self.old_start_type.as_ref()
    }

    /// Sets the value of Service
    pub fn set_service(&mut self, value: String) {
        self.service = Some(value);
    }

    /// Gets the value of Service
    pub fn get_service(&self) -> Option<&String> {
        self.service.as_ref()
    }

    /// Sets the value of sid
    pub fn set_sid(&mut self, value: String) {
        self.sid = Some(value);
    }

    /// Gets the value of sid
    pub fn get_sid(&self) -> Option<&String> {
        self.sid.as_ref()
    }
}

