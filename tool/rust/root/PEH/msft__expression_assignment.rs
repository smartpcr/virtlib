// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.PEH
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ExpressionAssignment struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ExpressionAssignment {
    #[serde(flatten)]
    pub base: MSFT_Expression,

/// 
    #[serde(rename = "lvalue")]
    pub lvalue: Option<MSFT_ExpressionIdentifier>,

/// 
    #[serde(rename = "rvalue")]
    pub rvalue: Option<MSFT_Expression>,
}

impl MSFT_ExpressionAssignment {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_Expression::new(),
            lvalue: None,
            rvalue: None,
        }
    }


    /// Sets the value of lvalue
    pub fn set_lvalue(&mut self, value: MSFT_ExpressionIdentifier) {
        self.lvalue = Some(value);
    }

    /// Gets the value of lvalue
    pub fn get_lvalue(&self) -> Option<&MSFT_ExpressionIdentifier> {
        self.lvalue.as_ref()
    }

    /// Sets the value of rvalue
    pub fn set_rvalue(&mut self, value: MSFT_Expression) {
        self.rvalue = Some(value);
    }

    /// Gets the value of rvalue
    pub fn get_rvalue(&self) -> Option<&MSFT_Expression> {
        self.rvalue.as_ref()
    }
}

