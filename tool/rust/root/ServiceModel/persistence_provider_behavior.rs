// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PersistenceProviderBehavior struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PersistenceProviderBehavior {
    #[serde(flatten)]
    pub base: Behavior,

/// Specifies the interval after which persistence operations are considered timed out.
    #[serde(rename = "PersistenceOperationTimeout")]
    pub persistence_operation_timeout: Option<String>,

/// Specifies the CLR type of the instance of persistence provider factory configured for durable services.
    #[serde(rename = "PersistenceProviderFactoryType")]
    pub persistence_provider_factory_type: Option<String>,
}

impl PersistenceProviderBehavior {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Behavior::new(),
            persistence_operation_timeout: None,
            persistence_provider_factory_type: None,
        }
    }


    /// Sets the value of PersistenceOperationTimeout
    pub fn set_persistence_operation_timeout(&mut self, value: String) {
        self.persistence_operation_timeout = Some(value);
    }

    /// Gets the value of PersistenceOperationTimeout
    pub fn get_persistence_operation_timeout(&self) -> Option<&String> {
        self.persistence_operation_timeout.as_ref()
    }

    /// Sets the value of PersistenceProviderFactoryType
    pub fn set_persistence_provider_factory_type(&mut self, value: String) {
        self.persistence_provider_factory_type = Some(value);
    }

    /// Gets the value of PersistenceProviderFactoryType
    pub fn get_persistence_provider_factory_type(&self) -> Option<&String> {
        self.persistence_provider_factory_type.as_ref()
    }
}

