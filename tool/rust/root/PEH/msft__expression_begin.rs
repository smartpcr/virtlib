// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.PEH
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ExpressionBegin struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ExpressionBegin {
    #[serde(flatten)]
    pub base: MSFT_Expression,

/// 
    #[serde(rename = "body")]
    pub body: Vec<MSFT_Expression>,
}

impl MSFT_ExpressionBegin {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_Expression::new(),
            body: Vec::new(),
        }
    }


    /// Sets the value of body
    pub fn set_body(&mut self, value: Vec<MSFT_Expression>) {
        self.body = value;
    }

    /// Gets the value of body
    pub fn get_body(&self) -> &Vec<MSFT_Expression> {
        &self.body
    }
}

