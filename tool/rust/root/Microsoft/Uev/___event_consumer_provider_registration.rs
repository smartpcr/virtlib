// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Uev
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __EventConsumerProviderRegistration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __EventConsumerProviderRegistration {
    #[serde(flatten)]
    pub base: __ProviderRegistration,

/// 
    #[serde(rename = "ConsumerClassNames")]
    pub consumer_class_names: Vec<String>,
}

impl __EventConsumerProviderRegistration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __ProviderRegistration::new(),
            consumer_class_names: Vec::new(),
        }
    }


    /// Sets the value of ConsumerClassNames
    pub fn set_consumer_class_names(&mut self, value: Vec<String>) {
        self.consumer_class_names = value;
    }

    /// Gets the value of ConsumerClassNames
    pub fn get_consumer_class_names(&self) -> &Vec<String> {
        &self.consumer_class_names
    }
}

