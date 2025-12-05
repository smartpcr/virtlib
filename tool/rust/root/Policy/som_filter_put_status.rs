// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Policy
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SomFilterPutStatus struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SomFilterPutStatus {
    #[serde(flatten)]
    pub base: __ExtendedStatus,

/// 
    #[serde(rename = "RuleValidationResults")]
    pub rule_validation_results: Vec<u32>,
}

impl SomFilterPutStatus {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __ExtendedStatus::new(),
            rule_validation_results: Vec::new(),
        }
    }


    /// Sets the value of RuleValidationResults
    pub fn set_rule_validation_results(&mut self, value: Vec<u32>) {
        self.rule_validation_results = value;
    }

    /// Gets the value of RuleValidationResults
    pub fn get_rule_validation_results(&self) -> &Vec<u32> {
        &self.rule_validation_results
    }
}

