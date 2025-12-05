// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.SDNDiagnostics
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ClassModification struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ClassModification {
    #[serde(flatten)]
    pub base: CIM_ClassIndication,

/// A copy of the 'previous' class definition whose change generated the Indication. PreviousClassDefinition contains an 'older' copy of the class' information, as compared to what is found in the ClassDefinition property (inherited from ClassIndication).
    #[serde(rename = "PreviousClassDefinition")]
    pub previous_class_definition: Option<serde_json::Value>,
}

impl CIM_ClassModification {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ClassIndication::new(),
            previous_class_definition: None,
        }
    }


    /// Sets the value of PreviousClassDefinition
    pub fn set_previous_class_definition(&mut self, value: serde_json::Value) {
        self.previous_class_definition = Some(value);
    }

    /// Gets the value of PreviousClassDefinition
    pub fn get_previous_class_definition(&self) -> Option<&serde_json::Value> {
        self.previous_class_definition.as_ref()
    }
}

