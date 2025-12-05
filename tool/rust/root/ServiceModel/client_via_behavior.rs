// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ClientViaBehavior struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClientViaBehavior {
    #[serde(flatten)]
    pub base: Behavior,

/// The ViaUri.
    #[serde(rename = "Uri")]
    pub uri: Option<String>,
}

impl ClientViaBehavior {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Behavior::new(),
            uri: None,
        }
    }


    /// Sets the value of Uri
    pub fn set_uri(&mut self, value: String) {
        self.uri = Some(value);
    }

    /// Gets the value of Uri
    pub fn get_uri(&self) -> Option<&String> {
        self.uri.as_ref()
    }
}

