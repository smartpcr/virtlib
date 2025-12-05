// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Dependency struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Dependency {

/// 
    #[serde(rename = "Antecedent")]
    pub antecedent: Option<CIM_ManagedSystemElement>,

/// 
    #[serde(rename = "Dependent")]
    pub dependent: Option<CIM_ManagedSystemElement>,
}

impl CIM_Dependency {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            antecedent: None,
            dependent: None,
        }
    }


    /// Sets the value of Antecedent
    pub fn set_antecedent(&mut self, value: CIM_ManagedSystemElement) {
        self.antecedent = Some(value);
    }

    /// Gets the value of Antecedent
    pub fn get_antecedent(&self) -> Option<&CIM_ManagedSystemElement> {
        self.antecedent.as_ref()
    }

    /// Sets the value of Dependent
    pub fn set_dependent(&mut self, value: CIM_ManagedSystemElement) {
        self.dependent = Some(value);
    }

    /// Gets the value of Dependent
    pub fn get_dependent(&self) -> Option<&CIM_ManagedSystemElement> {
        self.dependent.as_ref()
    }
}

