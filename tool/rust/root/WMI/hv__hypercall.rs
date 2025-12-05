// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HV_Hypercall struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HV_Hypercall {
    #[serde(flatten)]
    pub base: PerfInfo_V2,

/// 
    #[serde(rename = "CallCode")]
    pub call_code: Option<u32>,

/// 
    #[serde(rename = "IsFast")]
    pub is_fast: Option<u8>,

/// 
    #[serde(rename = "IsNested")]
    pub is_nested: Option<u8>,
}

impl HV_Hypercall {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: PerfInfo_V2::new(),
            call_code: None,
            is_fast: None,
            is_nested: None,
        }
    }


    /// Sets the value of CallCode
    pub fn set_call_code(&mut self, value: u32) {
        self.call_code = Some(value);
    }

    /// Gets the value of CallCode
    pub fn get_call_code(&self) -> Option<&u32> {
        self.call_code.as_ref()
    }

    /// Sets the value of IsFast
    pub fn set_is_fast(&mut self, value: u8) {
        self.is_fast = Some(value);
    }

    /// Gets the value of IsFast
    pub fn get_is_fast(&self) -> Option<&u8> {
        self.is_fast.as_ref()
    }

    /// Sets the value of IsNested
    pub fn set_is_nested(&mut self, value: u8) {
        self.is_nested = Some(value);
    }

    /// Gets the value of IsNested
    pub fn get_is_nested(&self) -> Option<&u8> {
        self.is_nested.as_ref()
    }
}

