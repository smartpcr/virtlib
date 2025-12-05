// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.PEH
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ExpressionValue struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ExpressionValue {
    #[serde(flatten)]
    pub base: MSFT_Expression,

/// 
    #[serde(rename = "hasValue")]
    pub has_value: Option<bool>,
}

impl MSFT_ExpressionValue {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_Expression::new(),
            has_value: None,
        }
    }


    /// Sets the value of hasValue
    pub fn set_has_value(&mut self, value: bool) {
        self.has_value = Some(value);
    }

    /// Gets the value of hasValue
    pub fn get_has_value(&self) -> Option<&bool> {
        self.has_value.as_ref()
    }
}

