// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.PEH
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ExpressionIdentifier struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ExpressionIdentifier {
    #[serde(flatten)]
    pub base: MSFT_Expression,

/// 
    #[serde(rename = "name")]
    pub name: Option<String>,
}

impl MSFT_ExpressionIdentifier {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_Expression::new(),
            name: None,
        }
    }


    /// Sets the value of name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}

