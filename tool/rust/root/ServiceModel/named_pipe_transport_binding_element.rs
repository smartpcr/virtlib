// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// NamedPipeTransportBindingElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NamedPipeTransportBindingElement {
    #[serde(flatten)]
    pub base: ConnectionOrientedTransportBindingElement,

/// The connection pool settings.
    #[serde(rename = "ConnectionPoolSettings")]
    pub connection_pool_settings: Option<NamedPipeConnectionPoolSettings>,
}

impl NamedPipeTransportBindingElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: ConnectionOrientedTransportBindingElement::new(),
            connection_pool_settings: None,
        }
    }


    /// Sets the value of ConnectionPoolSettings
    pub fn set_connection_pool_settings(&mut self, value: NamedPipeConnectionPoolSettings) {
        self.connection_pool_settings = Some(value);
    }

    /// Gets the value of ConnectionPoolSettings
    pub fn get_connection_pool_settings(&self) -> Option<&NamedPipeConnectionPoolSettings> {
        self.connection_pool_settings.as_ref()
    }
}

