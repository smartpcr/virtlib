// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// DataContractSerializerOperationBehavior struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataContractSerializerOperationBehavior {
    #[serde(flatten)]
    pub base: Behavior,

/// When enabled the IExtensibleDataObject interface on data contract types will be ignored.
    #[serde(rename = "IgnoreExtensionDataObject")]
    pub ignore_extension_data_object: Option<bool>,

/// Limits the maximum number of objects that may be deserialized by the data contract serializer in a single deserialization episode.
    #[serde(rename = "MaxItemsInObjectGraph")]
    pub max_items_in_object_graph: Option<i32>,

/// Defines the style of the SOAP message.
    #[serde(rename = "Style")]
    pub style: Option<String>,
}

impl DataContractSerializerOperationBehavior {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Behavior::new(),
            ignore_extension_data_object: None,
            max_items_in_object_graph: None,
            style: None,
        }
    }


    /// Sets the value of IgnoreExtensionDataObject
    pub fn set_ignore_extension_data_object(&mut self, value: bool) {
        self.ignore_extension_data_object = Some(value);
    }

    /// Gets the value of IgnoreExtensionDataObject
    pub fn get_ignore_extension_data_object(&self) -> Option<&bool> {
        self.ignore_extension_data_object.as_ref()
    }

    /// Sets the value of MaxItemsInObjectGraph
    pub fn set_max_items_in_object_graph(&mut self, value: i32) {
        self.max_items_in_object_graph = Some(value);
    }

    /// Gets the value of MaxItemsInObjectGraph
    pub fn get_max_items_in_object_graph(&self) -> Option<&i32> {
        self.max_items_in_object_graph.as_ref()
    }

    /// Sets the value of Style
    pub fn set_style(&mut self, value: String) {
        self.style = Some(value);
    }

    /// Gets the value of Style
    pub fn get_style(&self) -> Option<&String> {
        self.style.as_ref()
    }
}

