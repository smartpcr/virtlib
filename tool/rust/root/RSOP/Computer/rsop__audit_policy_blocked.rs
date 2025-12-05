// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_AuditPolicyBlocked struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_AuditPolicyBlocked {
    #[serde(flatten)]
    pub base: RSOP_SecuritySettingsBlocked,

/// 
    #[serde(rename = "Category")]
    pub category: Option<String>,

/// 
    #[serde(rename = "Failure")]
    pub failure: Option<bool>,

/// 
    #[serde(rename = "Success")]
    pub success: Option<bool>,
}

impl RSOP_AuditPolicyBlocked {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: RSOP_SecuritySettingsBlocked::new(),
            category: None,
            failure: None,
            success: None,
        }
    }


    /// Sets the value of Category
    pub fn set_category(&mut self, value: String) {
        self.category = Some(value);
    }

    /// Gets the value of Category
    pub fn get_category(&self) -> Option<&String> {
        self.category.as_ref()
    }

    /// Sets the value of Failure
    pub fn set_failure(&mut self, value: bool) {
        self.failure = Some(value);
    }

    /// Gets the value of Failure
    pub fn get_failure(&self) -> Option<&bool> {
        self.failure.as_ref()
    }

    /// Sets the value of Success
    pub fn set_success(&mut self, value: bool) {
        self.success = Some(value);
    }

    /// Gets the value of Success
    pub fn get_success(&self) -> Option<&bool> {
        self.success.as_ref()
    }
}

