// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_SARule struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_SARule {
    #[serde(flatten)]
    pub base: CIM_PolicyRule,

/// 
    #[serde(rename = "LimitNegotiation")]
    pub limit_negotiation: Option<u16>,
}

impl CIM_SARule {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PolicyRule::new(),
            limit_negotiation: None,
        }
    }


    /// Sets the value of LimitNegotiation
    pub fn set_limit_negotiation(&mut self, value: u16) {
        self.limit_negotiation = Some(value);
    }

    /// Gets the value of LimitNegotiation
    pub fn get_limit_negotiation(&self) -> Option<&u16> {
        self.limit_negotiation.as_ref()
    }
}

