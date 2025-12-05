// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CritSecInit struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CritSecInit {
    #[serde(flatten)]
    pub base: CritSecTrace,

/// 
    #[serde(rename = "CritSecAddr")]
    pub crit_sec_addr: Option<u32>,

/// 
    #[serde(rename = "SpinCount")]
    pub spin_count: Option<u32>,
}

impl CritSecInit {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CritSecTrace::new(),
            crit_sec_addr: None,
            spin_count: None,
        }
    }


    /// Sets the value of CritSecAddr
    pub fn set_crit_sec_addr(&mut self, value: u32) {
        self.crit_sec_addr = Some(value);
    }

    /// Gets the value of CritSecAddr
    pub fn get_crit_sec_addr(&self) -> Option<&u32> {
        self.crit_sec_addr.as_ref()
    }

    /// Sets the value of SpinCount
    pub fn set_spin_count(&mut self, value: u32) {
        self.spin_count = Some(value);
    }

    /// Gets the value of SpinCount
    pub fn get_spin_count(&self) -> Option<&u32> {
        self.spin_count.as_ref()
    }
}

