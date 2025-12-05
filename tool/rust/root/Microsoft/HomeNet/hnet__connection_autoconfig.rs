// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.HomeNet
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HNet_ConnectionAutoconfig struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HNet_ConnectionAutoconfig {

/// 
    #[serde(rename = "Connection")]
    pub connection: Option<HNet_Connection>,
}

impl HNet_ConnectionAutoconfig {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            connection: None,
        }
    }


    /// Sets the value of Connection
    pub fn set_connection(&mut self, value: HNet_Connection) {
        self.connection = Some(value);
    }

    /// Gets the value of Connection
    pub fn get_connection(&self) -> Option<&HNet_Connection> {
        self.connection.as_ref()
    }
}

