// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageObject struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageObject {

/// ObjectId is a mandatory property that is used to opaquely and uniquely identify an instance of a class. ObjectIds must be unique within the scope of the management server (which is hosting the provider). The ObjectId is created and maintained for use of the Storage Management Providers and their clients to track instances of objects. If an object is visible through two different paths (for example: there are two separate Storage Management Providers that point to the same storage subsystem) then the same object may appear with two different ObjectIds. For determining if two object instances are the same object, refer to the UniqueId property.
    #[serde(rename = "ObjectId")]
    pub object_id: Option<String>,

/// PassThroughClass is the WBEM class name of the proprietary storage provider object.
    #[serde(rename = "PassThroughClass")]
    pub pass_through_class: Option<String>,

/// PassThroughIds is a comma-separated list of all implementation specific keys. It is used by storage management applications to access the vendor proprietary object model. This field should be in the form: key1='value1',key2='value2'.
    #[serde(rename = "PassThroughIds")]
    pub pass_through_ids: Option<String>,

/// PassThroughNamespace is the WBEM namespace that contains the proprietary storage provider classes.
    #[serde(rename = "PassThroughNamespace")]
    pub pass_through_namespace: Option<String>,

/// PassThroughServer is the name or address of the computer system hosting the proprietary storage provider classes.
    #[serde(rename = "PassThroughServer")]
    pub pass_through_server: Option<String>,

/// UniqueId is a mandatory property that is used to uniquely identify a logical instance of a storage subsystem's object. This value must be the same for an object viewed by two or more provider instances (even if they are running on seperate management servers). UniqueId can be any globally unique, opaque value unless otherwise specified by a derived class.
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

