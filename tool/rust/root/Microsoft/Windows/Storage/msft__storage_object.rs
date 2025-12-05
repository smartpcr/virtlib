// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageObject struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageObject {

/// 
    #[serde(rename = "ObjectId")]
    pub object_id: Option<String>,

/// 
    #[serde(rename = "PassThroughClass")]
    pub pass_through_class: Option<String>,

/// 
    #[serde(rename = "PassThroughIds")]
    pub pass_through_ids: Option<String>,

/// 
    #[serde(rename = "PassThroughNamespace")]
    pub pass_through_namespace: Option<String>,

/// 
    #[serde(rename = "PassThroughServer")]
    pub pass_through_server: Option<String>,

/// 
    #[serde(rename = "UniqueId")]
    pub unique_id: Option<String>,
}

impl MSFT_StorageObject {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            object_id: None,
            pass_through_class: None,
            pass_through_ids: None,
            pass_through_namespace: None,
            pass_through_server: None,
            unique_id: None,
        }
    }


    /// Sets the value of ObjectId
    pub fn set_object_id(&mut self, value: String) {
        self.object_id = Some(value);
    }

    /// Gets the value of ObjectId
    pub fn get_object_id(&self) -> Option<&String> {
        self.object_id.as_ref()
    }

    /// Sets the value of PassThroughClass
    pub fn set_pass_through_class(&mut self, value: String) {
        self.pass_through_class = Some(value);
    }

    /// Gets the value of PassThroughClass
    pub fn get_pass_through_class(&self) -> Option<&String> {
        self.pass_through_class.as_ref()
    }

    /// Sets the value of PassThroughIds
    pub fn set_pass_through_ids(&mut self, value: String) {
        self.pass_through_ids = Some(value);
    }

    /// Gets the value of PassThroughIds
    pub fn get_pass_through_ids(&self) -> Option<&String> {
        self.pass_through_ids.as_ref()
    }

    /// Sets the value of PassThroughNamespace
    pub fn set_pass_through_namespace(&mut self, value: String) {
        self.pass_through_namespace = Some(value);
    }

    /// Gets the value of PassThroughNamespace
    pub fn get_pass_through_namespace(&self) -> Option<&String> {
        self.pass_through_namespace.as_ref()
    }

    /// Sets the value of PassThroughServer
    pub fn set_pass_through_server(&mut self, value: String) {
        self.pass_through_server = Some(value);
    }

    /// Gets the value of PassThroughServer
    pub fn get_pass_through_server(&self) -> Option<&String> {
        self.pass_through_server.as_ref()
    }

    /// Sets the value of UniqueId
    pub fn set_unique_id(&mut self, value: String) {
        self.unique_id = Some(value);
    }

    /// Gets the value of UniqueId
    pub fn get_unique_id(&self) -> Option<&String> {
        self.unique_id.as_ref()
    }
}

