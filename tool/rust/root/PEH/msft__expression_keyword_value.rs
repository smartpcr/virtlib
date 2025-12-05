// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.PEH
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ExpressionKeywordValue struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ExpressionKeywordValue {
    #[serde(flatten)]
    pub base: MSFT_Expression,

/// 
    #[serde(rename = "keyword")]
    pub keyword: Option<String>,
}

impl MSFT_ExpressionKeywordValue {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_Expression::new(),
            keyword: None,
        }
    }


    /// Sets the value of keyword
    pub fn set_keyword(&mut self, value: String) {
        self.keyword = Some(value);
    }

    /// Gets the value of keyword
    pub fn get_keyword(&self) -> Option<&String> {
        self.keyword.as_ref()
    }
}

