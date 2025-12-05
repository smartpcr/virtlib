// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ReplicaPeer struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ReplicaPeer {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,

/// Indicates if the ReplicaPeer is primary i.e: If the ReplicaPeer is a System Element and not a Synced Element
    #[serde(rename = "IsPrimary")]
    pub is_primary: Option<bool>,

/// Populated when "Type" has the value of "EmbeddedInstance".
    #[serde(rename = "PeerObject")]
    pub peer_object: Option<MSFT_StorageObject>,

/// The ObjectId of the replica peer within the replica's storage subsystem.
    #[serde(rename = "PeerObjectId")]
    pub peer_object_id: Option<String>,

/// The name of the replica peer within the replica's storage subsystem.
    #[serde(rename = "PeerObjectName")]
    pub peer_object_name: Option<String>,

/// The object type of this replica peer.
    #[serde(rename = "PeerObjectType")]
    pub peer_object_type: Option<ReplicaPeer_PeerObjectType>,

/// If the SMP is of type 2 - 'SMI-S', this field contains the protocol, computer host name, and port of the SMI-S server. This field will be NULL otherwise.
    #[serde(rename = "PeerProviderURI")]
    pub peer_provider_uri: Option<String>,

/// The subsystem name of the replica peer within the replica's storage subsystem.
    #[serde(rename = "PeerSubsystemName")]
    pub peer_subsystem_name: Option<String>,

/// The UniqueId of the replica peer within the replica's storage subsystem.
    #[serde(rename = "PeerUniqueId")]
    pub peer_unique_id: Option<String>,
}

impl MSFT_ReplicaPeer {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageObject::new(),
            is_primary: None,
            peer_object: None,
            peer_object_id: None,
            peer_object_name: None,
            peer_object_type: None,
            peer_provider_uri: None,
            peer_subsystem_name: None,
            peer_unique_id: None,
        }
    }


    /// Sets the value of IsPrimary
    pub fn set_is_primary(&mut self, value: bool) {
        self.is_primary = Some(value);
    }

    /// Gets the value of IsPrimary
    pub fn get_is_primary(&self) -> Option<&bool> {
        self.is_primary.as_ref()
    }

    /// Sets the value of PeerObject
    pub fn set_peer_object(&mut self, value: MSFT_StorageObject) {
        self.peer_object = Some(value);
    }

    /// Gets the value of PeerObject
    pub fn get_peer_object(&self) -> Option<&MSFT_StorageObject> {
        self.peer_object.as_ref()
    }

    /// Sets the value of PeerObjectId
    pub fn set_peer_object_id(&mut self, value: String) {
        self.peer_object_id = Some(value);
    }

    /// Gets the value of PeerObjectId
    pub fn get_peer_object_id(&self) -> Option<&String> {
        self.peer_object_id.as_ref()
    }

    /// Sets the value of PeerObjectName
    pub fn set_peer_object_name(&mut self, value: String) {
        self.peer_object_name = Some(value);
    }

    /// Gets the value of PeerObjectName
    pub fn get_peer_object_name(&self) -> Option<&String> {
        self.peer_object_name.as_ref()
    }

    /// Sets the value of PeerObjectType
    pub fn set_peer_object_type(&mut self, value: ReplicaPeer_PeerObjectType) {
        self.peer_object_type = Some(value);
    }

    /// Gets the value of PeerObjectType
    pub fn get_peer_object_type(&self) -> Option<&ReplicaPeer_PeerObjectType> {
        self.peer_object_type.as_ref()
    }

    /// Sets the value of PeerProviderURI
    pub fn set_peer_provider_uri(&mut self, value: String) {
        self.peer_provider_uri = Some(value);
    }

    /// Gets the value of PeerProviderURI
    pub fn get_peer_provider_uri(&self) -> Option<&String> {
        self.peer_provider_uri.as_ref()
    }

    /// Sets the value of PeerSubsystemName
    pub fn set_peer_subsystem_name(&mut self, value: String) {
        self.peer_subsystem_name = Some(value);
    }

    /// Gets the value of PeerSubsystemName
    pub fn get_peer_subsystem_name(&self) -> Option<&String> {
        self.peer_subsystem_name.as_ref()
    }

    /// Sets the value of PeerUniqueId
    pub fn set_peer_unique_id(&mut self, value: String) {
        self.peer_unique_id = Some(value);
    }

    /// Gets the value of PeerUniqueId
    pub fn get_peer_unique_id(&self) -> Option<&String> {
        self.peer_unique_id.as_ref()
    }
}

