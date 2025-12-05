// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.PEH
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ExpressionKeywordParameter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ExpressionKeywordParameter {
    #[serde(flatten)]
    pub base: MSFT_ExpressionIdentifier,

/// 
    #[serde(rename = "keywordalias")]
    pub keywordalias: Option<String>,
}

impl MSFT_ExpressionKeywordParameter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_ExpressionIdentifier::new(),
            keywordalias: None,
        }
    }


    /// Sets the value of keywordalias
    pub fn set_keywordalias(&mut self, value: String) {
        self.keywordalias = Some(value);
    }

    /// Gets the value of keywordalias
    pub fn get_keywordalias(&self) -> Option<&String> {
        self.keywordalias.as_ref()
    }
}

