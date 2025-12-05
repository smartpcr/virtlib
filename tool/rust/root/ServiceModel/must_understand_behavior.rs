// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MustUnderstandBehavior struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MustUnderstandBehavior {
    #[serde(flatten)]
    pub base: Behavior,

/// When true, all SOAP header with the MustUnderstand attribute that are not handled will cause the behavior to throw.
    #[serde(rename = "ValidateMustUnderstand")]
    pub validate_must_understand: Option<bool>,
}

impl MustUnderstandBehavior {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Behavior::new(),
            validate_must_understand: None,
        }
    }


    /// Sets the value of ValidateMustUnderstand
    pub fn set_validate_must_understand(&mut self, value: bool) {
        self.validate_must_understand = Some(value);
    }

    /// Gets the value of ValidateMustUnderstand
    pub fn get_validate_must_understand(&self) -> Option<&bool> {
        self.validate_must_understand.as_ref()
    }
}

