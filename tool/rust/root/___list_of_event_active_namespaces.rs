// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __ListOfEventActiveNamespaces struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __ListOfEventActiveNamespaces {
    #[serde(flatten)]
    pub base: __SystemClass,

/// 
    #[serde(rename = "Namespaces")]
    pub namespaces: Vec<String>,
}

impl __ListOfEventActiveNamespaces {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __SystemClass::new(),
            namespaces: Vec::new(),
        }
    }


    /// Sets the value of Namespaces
    pub fn set_namespaces(&mut self, value: Vec<String>) {
        self.namespaces = value;
    }

    /// Gets the value of Namespaces
    pub fn get_namespaces(&self) -> &Vec<String> {
        &self.namespaces
    }
}

