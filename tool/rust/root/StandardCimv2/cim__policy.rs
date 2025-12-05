// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Policy struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Policy {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "CommonName")]
    pub common_name: Option<String>,

/// 
    #[serde(rename = "PolicyKeywords")]
    pub policy_keywords: Vec<String>,
}

impl CIM_Policy {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            common_name: None,
            policy_keywords: Vec::new(),
        }
    }


    /// Sets the value of CommonName
    pub fn set_common_name(&mut self, value: String) {
        self.common_name = Some(value);
    }

    /// Gets the value of CommonName
    pub fn get_common_name(&self) -> Option<&String> {
        self.common_name.as_ref()
    }

    /// Sets the value of PolicyKeywords
    pub fn set_policy_keywords(&mut self, value: Vec<String>) {
        self.policy_keywords = value;
    }

    /// Gets the value of PolicyKeywords
    pub fn get_policy_keywords(&self) -> &Vec<String> {
        &self.policy_keywords
    }
}

