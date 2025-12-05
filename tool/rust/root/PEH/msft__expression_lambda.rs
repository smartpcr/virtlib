// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.PEH
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ExpressionLambda struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ExpressionLambda {
    #[serde(flatten)]
    pub base: MSFT_Expression,

/// 
    #[serde(rename = "body")]
    pub body: Option<MSFT_Expression>,

/// 
    #[serde(rename = "parameters")]
    pub parameters: Vec<MSFT_ExpressionIdentifier>,

/// 
    #[serde(rename = "pipeline")]
    pub pipeline: Option<MSFT_ExpressionIdentifier>,
}

impl MSFT_ExpressionLambda {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_Expression::new(),
            body: None,
            parameters: Vec::new(),
            pipeline: None,
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

    /// Sets the value of parameters
    pub fn set_parameters(&mut self, value: Vec<MSFT_ExpressionIdentifier>) {
        self.parameters = value;
    }

    /// Gets the value of parameters
    pub fn get_parameters(&self) -> &Vec<MSFT_ExpressionIdentifier> {
        &self.parameters
    }

    /// Sets the value of pipeline
    pub fn set_pipeline(&mut self, value: MSFT_ExpressionIdentifier) {
        self.pipeline = Some(value);
    }

    /// Gets the value of pipeline
    pub fn get_pipeline(&self) -> Option<&MSFT_ExpressionIdentifier> {
        self.pipeline.as_ref()
    }
}

