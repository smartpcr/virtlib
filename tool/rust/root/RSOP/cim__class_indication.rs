// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ClassIndication struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ClassIndication {
    #[serde(flatten)]
    pub base: CIM_Indication,

/// The current definition of the class that is created, changed or deleted in the schema. In the case of a CIM_ClassDeletion Indication, the definition for the class just prior to deletion should be placed in this property.
    #[serde(rename = "ClassDefinition")]
    pub class_definition: Option<serde_json::Value>,
}

impl CIM_ClassIndication {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Indication::new(),
            class_definition: None,
        }
    }


    /// Sets the value of ClassDefinition
    pub fn set_class_definition(&mut self, value: serde_json::Value) {
        self.class_definition = Some(value);
    }

    /// Gets the value of ClassDefinition
    pub fn get_class_definition(&self) -> Option<&serde_json::Value> {
        self.class_definition.as_ref()
    }
}

