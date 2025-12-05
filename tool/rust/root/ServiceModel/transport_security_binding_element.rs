// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// TransportSecurityBindingElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TransportSecurityBindingElement {
    #[serde(flatten)]
    pub base: SecurityBindingElement,
}

impl TransportSecurityBindingElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SecurityBindingElement::new(),
        }
    }

}

