// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.PEH
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ExpressionLoop struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ExpressionLoop {
    #[serde(flatten)]
    pub base: MSFT_Expression,

/// 
    #[serde(rename = "body")]
    pub body: Option<MSFT_Expression>,

/// 
    #[serde(rename = "condition")]
    pub condition: Option<MSFT_Expression>,
}

impl MSFT_ExpressionLoop {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_Expression::new(),
            body: None,
            condition: None,
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

    /// Sets the value of condition
    pub fn set_condition(&mut self, value: MSFT_Expression) {
        self.condition = Some(value);
    }

    /// Gets the value of condition
    pub fn get_condition(&self) -> Option<&MSFT_Expression> {
        self.condition.as_ref()
    }
}

