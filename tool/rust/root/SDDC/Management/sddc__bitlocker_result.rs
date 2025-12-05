// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.SDDC.Management
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SDDC_BitlockerResult struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SDDC_BitlockerResult {

/// 
    #[serde(rename = "ErrorCode")]
    pub error_code: Option<u32>,

/// 
    #[serde(rename = "FailedPhase")]
    pub failed_phase: Option<u16>,

/// 
    #[serde(rename = "RecoveryPassword")]
    pub recovery_password: Option<String>,
}

impl SDDC_BitlockerResult {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            error_code: None,
            failed_phase: None,
            recovery_password: None,
        }
    }


    /// Sets the value of ErrorCode
    pub fn set_error_code(&mut self, value: u32) {
        self.error_code = Some(value);
    }

    /// Gets the value of ErrorCode
    pub fn get_error_code(&self) -> Option<&u32> {
        self.error_code.as_ref()
    }

    /// Sets the value of FailedPhase
    pub fn set_failed_phase(&mut self, value: u16) {
        self.failed_phase = Some(value);
    }

    /// Gets the value of FailedPhase
    pub fn get_failed_phase(&self) -> Option<&u16> {
        self.failed_phase.as_ref()
    }

    /// Sets the value of RecoveryPassword
    pub fn set_recovery_password(&mut self, value: String) {
        self.recovery_password = Some(value);
    }

    /// Gets the value of RecoveryPassword
    pub fn get_recovery_password(&self) -> Option<&String> {
        self.recovery_password.as_ref()
    }
}

