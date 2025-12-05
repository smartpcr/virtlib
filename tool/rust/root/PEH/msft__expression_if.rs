// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.PEH
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ExpressionIf struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ExpressionIf {
    #[serde(flatten)]
    pub base: MSFT_Expression,

/// 
    #[serde(rename = "condition")]
    pub condition: Option<MSFT_Expression>,

/// 
    #[serde(rename = "falsecase")]
    pub falsecase: Option<MSFT_Expression>,

/// 
    #[serde(rename = "truecase")]
    pub truecase: Option<MSFT_Expression>,
}

impl MSFT_ExpressionIf {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_Expression::new(),
            condition: None,
            falsecase: None,
            truecase: None,
        }
    }


    /// Sets the value of condition
    pub fn set_condition(&mut self, value: MSFT_Expression) {
        self.condition = Some(value);
    }

    /// Gets the value of condition
    pub fn get_condition(&self) -> Option<&MSFT_Expression> {
        self.condition.as_ref()
    }

    /// Sets the value of falsecase
    pub fn set_falsecase(&mut self, value: MSFT_Expression) {
        self.falsecase = Some(value);
    }

    /// Gets the value of falsecase
    pub fn get_falsecase(&self) -> Option<&MSFT_Expression> {
        self.falsecase.as_ref()
    }

    /// Sets the value of truecase
    pub fn set_truecase(&mut self, value: MSFT_Expression) {
        self.truecase = Some(value);
    }

    /// Gets the value of truecase
    pub fn get_truecase(&self) -> Option<&MSFT_Expression> {
        self.truecase.as_ref()
    }
}

