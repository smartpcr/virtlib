// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_IEESC struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_IEESC {

/// 
    #[serde(rename = "EscEnabled")]
    pub esc_enabled: Option<bool>,

/// 
    #[serde(rename = "rsopID")]
    pub rsop_id: Option<String>,

/// 
    #[serde(rename = "rsopPrecedence")]
    pub rsop_precedence: Option<i32>,
}

impl RSOP_IEESC {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            esc_enabled: None,
            rsop_id: None,
            rsop_precedence: None,
        }
    }


    /// Sets the value of EscEnabled
    pub fn set_esc_enabled(&mut self, value: bool) {
        self.esc_enabled = Some(value);
    }

    /// Gets the value of EscEnabled
    pub fn get_esc_enabled(&self) -> Option<&bool> {
        self.esc_enabled.as_ref()
    }

    /// Sets the value of rsopID
    pub fn set_rsop_id(&mut self, value: String) {
        self.rsop_id = Some(value);
    }

    /// Gets the value of rsopID
    pub fn get_rsop_id(&self) -> Option<&String> {
        self.rsop_id.as_ref()
    }

    /// Sets the value of rsopPrecedence
    pub fn set_rsop_precedence(&mut self, value: i32) {
        self.rsop_precedence = Some(value);
    }

    /// Gets the value of rsopPrecedence
    pub fn get_rsop_precedence(&self) -> Option<&i32> {
        self.rsop_precedence.as_ref()
    }
}

