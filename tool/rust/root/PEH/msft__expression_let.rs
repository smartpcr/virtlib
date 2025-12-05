// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.PEH
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ExpressionLet struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ExpressionLet {
    #[serde(flatten)]
    pub base: MSFT_Expression,

/// 
    #[serde(rename = "body")]
    pub body: Option<MSFT_Expression>,

/// 
    #[serde(rename = "initializers")]
    pub initializers: Vec<MSFT_ExpressionAssignment>,
}

impl MSFT_ExpressionLet {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_Expression::new(),
            body: None,
            initializers: Vec::new(),
        }
    }


    /// Sets the value of body
    pub fn set_body(&mut self, value: MSFT_Expression) {
        self.body = Some(value);
    }

    /// Gets the value of body
    pub fn get_body(&self) -> Option<&MSFT_Expression> {
        self.body.as_ref()
    }

    /// Sets the value of initializers
    pub fn set_initializers(&mut self, value: Vec<MSFT_ExpressionAssignment>) {
        self.initializers = value;
    }

    /// Gets the value of initializers
    pub fn get_initializers(&self) -> &Vec<MSFT_ExpressionAssignment> {
        &self.initializers
    }
}

