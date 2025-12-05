// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HypercallPage struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HypercallPage {
    #[serde(flatten)]
    pub base: Image_V2,

/// 
    #[serde(rename = "HypercallPageVa")]
    pub hypercall_page_va: Option<u32>,
}

impl HypercallPage {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Image_V2::new(),
            hypercall_page_va: None,
        }
    }


    /// Sets the value of HypercallPageVa
    pub fn set_hypercall_page_va(&mut self, value: u32) {
        self.hypercall_page_va = Some(value);
    }

    /// Gets the value of HypercallPageVa
    pub fn get_hypercall_page_va(&self) -> Option<&u32> {
        self.hypercall_page_va.as_ref()
    }
}

