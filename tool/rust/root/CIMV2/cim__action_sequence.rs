// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ActionSequence struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ActionSequence {

/// 
    #[serde(rename = "Next")]
    pub next: Option<CIM_Action>,

/// 
    #[serde(rename = "Prior")]
    pub prior: Option<CIM_Action>,
}

impl CIM_ActionSequence {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            next: None,
            prior: None,
        }
    }


    /// Sets the value of Next
    pub fn set_next(&mut self, value: CIM_Action) {
        self.next = Some(value);
    }

    /// Gets the value of Next
    pub fn get_next(&self) -> Option<&CIM_Action> {
        self.next.as_ref()
    }

    /// Sets the value of Prior
    pub fn set_prior(&mut self, value: CIM_Action) {
        self.prior = Some(value);
    }

    /// Gets the value of Prior
    pub fn get_prior(&self) -> Option<&CIM_Action> {
        self.prior.as_ref()
    }
}

