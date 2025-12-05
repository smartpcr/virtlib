// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.PEH
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ExpressionValue_sint32 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ExpressionValue_sint32 {
    #[serde(flatten)]
    pub base: MSFT_ExpressionValue,

/// 
    #[serde(rename = "value")]
    pub value: Option<i32>,
}

impl MSFT_ExpressionValue_sint32 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_ExpressionValue::new(),
            value: None,
        }
    }


    /// Sets the value of value
    pub fn set_value(&mut self, value: i32) {
        self.value = Some(value);
    }

    /// Gets the value of value
    pub fn get_value(&self) -> Option<&i32> {
        self.value.as_ref()
    }
}

