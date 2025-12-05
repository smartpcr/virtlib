// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.InventoryLogging
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msft_MiQuery struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msft_MiQuery {
    #[serde(flatten)]
    pub base: Msft_MiStream,

/// 
    #[serde(rename = "Dialect")]
    pub dialect: Option<String>,

/// 
    #[serde(rename = "Expression")]
    pub expression: Option<String>,

/// 
    #[serde(rename = "NamespaceName")]
    pub namespace_name: Option<String>,
}

impl Msft_MiQuery {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msft_MiStream::new(),
            dialect: None,
            expression: None,
            namespace_name: None,
        }
    }


    /// Sets the value of Dialect
    pub fn set_dialect(&mut self, value: String) {
        self.dialect = Some(value);
    }

    /// Gets the value of Dialect
    pub fn get_dialect(&self) -> Option<&String> {
        self.dialect.as_ref()
    }

    /// Sets the value of Expression
    pub fn set_expression(&mut self, value: String) {
        self.expression = Some(value);
    }

    /// Gets the value of Expression
    pub fn get_expression(&self) -> Option<&String> {
        self.expression.as_ref()
    }

    /// Sets the value of NamespaceName
    pub fn set_namespace_name(&mut self, value: String) {
        self.namespace_name = Some(value);
    }

    /// Gets the value of NamespaceName
    pub fn get_namespace_name(&self) -> Option<&String> {
        self.namespace_name.as_ref()
    }
}

