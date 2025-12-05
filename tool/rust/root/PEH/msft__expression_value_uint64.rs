// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.PEH
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ExpressionValue_uint64 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ExpressionValue_uint64 {
    #[serde(flatten)]
    pub base: MSFT_ExpressionValue,

/// 
    #[serde(rename = "value")]
    pub value: Option<u64>,
}

impl MSFT_ExpressionValue_uint64 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_ExpressionValue::new(),
            value: None,
        }
    }


    /// Sets the value of value
    pub fn set_value(&mut self, value: u64) {
        self.value = Some(value);
    }

    /// Gets the value of value
    pub fn get_value(&self) -> Option<&u64> {
        self.value.as_ref()
    }
}

