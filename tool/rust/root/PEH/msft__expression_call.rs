// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.PEH
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ExpressionCall struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ExpressionCall {
    #[serde(flatten)]
    pub base: MSFT_Expression,

/// 
    #[serde(rename = "arguments")]
    pub arguments: Vec<MSFT_Expression>,

/// 
    #[serde(rename = "function")]
    pub function: Option<MSFT_Expression>,

/// 
    #[serde(rename = "pipeline")]
    pub pipeline: Option<MSFT_Expression>,
}

impl MSFT_ExpressionCall {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_Expression::new(),
            arguments: Vec::new(),
            function: None,
            pipeline: None,
        }
    }


    /// Sets the value of arguments
    pub fn set_arguments(&mut self, value: Vec<MSFT_Expression>) {
        self.arguments = value;
    }

    /// Gets the value of arguments
    pub fn get_arguments(&self) -> &Vec<MSFT_Expression> {
        &self.arguments
    }

    /// Sets the value of function
    pub fn set_function(&mut self, value: MSFT_Expression) {
        self.function = Some(value);
    }

    /// Gets the value of function
    pub fn get_function(&self) -> Option<&MSFT_Expression> {
        self.function.as_ref()
    }

    /// Sets the value of pipeline
    pub fn set_pipeline(&mut self, value: MSFT_Expression) {
        self.pipeline = Some(value);
    }

    /// Gets the value of pipeline
    pub fn get_pipeline(&self) -> Option<&MSFT_Expression> {
        self.pipeline.as_ref()
    }
}

