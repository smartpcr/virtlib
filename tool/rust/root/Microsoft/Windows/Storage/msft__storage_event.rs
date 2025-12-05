// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageEvent {

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "EventTime")]
    pub event_time: Option<String>,

/// 
    #[serde(rename = "PerceivedSeverity")]
    pub perceived_severity: Option<u16>,

/// 
    #[serde(rename = "SourceClassName")]
    pub source_class_name: Option<String>,

/// 
    #[serde(rename = "SourceInstance")]
    pub source_instance: Option<MSFT_StorageObject>,

/// 
    #[serde(rename = "SourceNamespace")]
    pub source_namespace: Option<String>,

/// 
    #[serde(rename = "SourceObjectId")]
    pub source_object_id: Option<String>,

/// 
    #[serde(rename = "SourceServer")]
    pub source_server: Option<String>,

/// 
    #[serde(rename = "StorageSubsystemObjectId")]
    pub storage_subsystem_object_id: Option<String>,
}

impl MSFT_StorageEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            description: None,
            event_time: None,
            perceived_severity: None,
            source_class_name: None,
            source_instance: None,
            source_namespace: None,
            source_object_id: None,
            source_server: None,
            storage_subsystem_object_id: None,
        }
    }


    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of EventTime
    pub fn set_event_time(&mut self, value: String) {
        self.event_time = Some(value);
    }

    /// Gets the value of EventTime
    pub fn get_event_time(&self) -> Option<&String> {
        self.event_time.as_ref()
    }

    /// Sets the value of PerceivedSeverity
    pub fn set_perceived_severity(&mut self, value: u16) {
        self.perceived_severity = Some(value);
    }

    /// Gets the value of PerceivedSeverity
    pub fn get_perceived_severity(&self) -> Option<&u16> {
        self.perceived_severity.as_ref()
    }

    /// Sets the value of SourceClassName
    pub fn set_source_class_name(&mut self, value: String) {
        self.source_class_name = Some(value);
    }

    /// Gets the value of SourceClassName
    pub fn get_source_class_name(&self) -> Option<&String> {
        self.source_class_name.as_ref()
    }

    /// Sets the value of SourceInstance
    pub fn set_source_instance(&mut self, value: MSFT_StorageObject) {
        self.source_instance = Some(value);
    }

    /// Gets the value of SourceInstance
    pub fn get_source_instance(&self) -> Option<&MSFT_StorageObject> {
        self.source_instance.as_ref()
    }

    /// Sets the value of SourceNamespace
    pub fn set_source_namespace(&mut self, value: String) {
        self.source_namespace = Some(value);
    }

    /// Gets the value of SourceNamespace
    pub fn get_source_namespace(&self) -> Option<&String> {
        self.source_namespace.as_ref()
    }

    /// Sets the value of SourceObjectId
    pub fn set_source_object_id(&mut self, value: String) {
        self.source_object_id = Some(value);
    }

    /// Gets the value of SourceObjectId
    pub fn get_source_object_id(&self) -> Option<&String> {
        self.source_object_id.as_ref()
    }

    /// Sets the value of SourceServer
    pub fn set_source_server(&mut self, value: String) {
        self.source_server = Some(value);
    }

    /// Gets the value of SourceServer
    pub fn get_source_server(&self) -> Option<&String> {
        self.source_server.as_ref()
    }

    /// Sets the value of StorageSubsystemObjectId
    pub fn set_storage_subsystem_object_id(&mut self, value: String) {
        self.storage_subsystem_object_id = Some(value);
    }

    /// Gets the value of StorageSubsystemObjectId
    pub fn get_storage_subsystem_object_id(&self) -> Option<&String> {
        self.storage_subsystem_object_id.as_ref()
    }
}

