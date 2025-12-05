// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.PEH
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MintPipelineDefinition struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MintPipelineDefinition {
    #[serde(flatten)]
    pub base: MSFT_PipelineDefinition,

/// 
    #[serde(rename = "expression")]
    pub expression: Option<MSFT_Expression>,
}

impl MSFT_MintPipelineDefinition {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_PipelineDefinition::new(),
            expression: None,
        }
    }


    /// Sets the value of expression
    pub fn set_expression(&mut self, value: MSFT_Expression) {
        self.expression = Some(value);
    }

    /// Gets the value of expression
    pub fn get_expression(&self) -> Option<&MSFT_Expression> {
        self.expression.as_ref()
    }
}

