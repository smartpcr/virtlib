// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Service struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Service {

/// The base addresses used by the service.
    #[serde(rename = "BaseAddresses")]
    pub base_addresses: Vec<String>,

/// The behaviors associated with this service.
    #[serde(rename = "Behaviors")]
    pub behaviors: Vec<Behavior>,

/// ServiceElement_BehaviorConfiguration
    #[serde(rename = "ConfigurationName")]
    pub configuration_name: Option<String>,

/// Instance name of the instance of the performance counters of the service. 
    #[serde(rename = "CounterInstanceName")]
    pub counter_instance_name: Option<String>,

/// Service name at the address.
    #[serde(rename = "DistinguishedName")]
    pub distinguished_name: Option<String>,

/// The instance contexts for the extensions of the service instance.
    #[serde(rename = "Extensions")]
    pub extensions: Vec<String>,

/// The service metadata settings.
    #[serde(rename = "Metadata")]
    pub metadata: Vec<String>,

/// The unique name of this service.
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// The namespace of the service.
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,

/// The time the service was opened.
    #[serde(rename = "Opened")]
    pub opened: Option<String>,

/// The channels that are outgoing from the service instance.
    #[serde(rename = "OutgoingChannels")]
    pub outgoing_channels: Vec<Channel>,

/// The process id of the process that hosts the service.
    #[serde(rename = "ProcessId")]
    pub process_id: Option<i32>,
}

impl Service {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base_addresses: Vec::new(),
            behaviors: Vec::new(),
            configuration_name: None,
            counter_instance_name: None,
            distinguished_name: None,
            extensions: Vec::new(),
            metadata: Vec::new(),
            name: None,
            namespace: None,
            opened: None,
            outgoing_channels: Vec::new(),
            process_id: None,
        }
    }


    /// Sets the value of BaseAddresses
    pub fn set_base_addresses(&mut self, value: Vec<String>) {
        self.base_addresses = value;
    }

    /// Gets the value of BaseAddresses
    pub fn get_base_addresses(&self) -> &Vec<String> {
        &self.base_addresses
    }

    /// Sets the value of Behaviors
    pub fn set_behaviors(&mut self, value: Vec<Behavior>) {
        self.behaviors = value;
    }

    /// Gets the value of Behaviors
    pub fn get_behaviors(&self) -> &Vec<Behavior> {
        &self.behaviors
    }

    /// Sets the value of ConfigurationName
    pub fn set_configuration_name(&mut self, value: String) {
        self.configuration_name = Some(value);
    }

    /// Gets the value of ConfigurationName
    pub fn get_configuration_name(&self) -> Option<&String> {
        self.configuration_name.as_ref()
    }

    /// Sets the value of CounterInstanceName
    pub fn set_counter_instance_name(&mut self, value: String) {
        self.counter_instance_name = Some(value);
    }

    /// Gets the value of CounterInstanceName
    pub fn get_counter_instance_name(&self) -> Option<&String> {
        self.counter_instance_name.as_ref()
    }

    /// Sets the value of DistinguishedName
    pub fn set_distinguished_name(&mut self, value: String) {
        self.distinguished_name = Some(value);
    }

    /// Gets the value of DistinguishedName
    pub fn get_distinguished_name(&self) -> Option<&String> {
        self.distinguished_name.as_ref()
    }

    /// Sets the value of Extensions
    pub fn set_extensions(&mut self, value: Vec<String>) {
        self.extensions = value;
    }

    /// Gets the value of Extensions
    pub fn get_extensions(&self) -> &Vec<String> {
        &self.extensions
    }

    /// Sets the value of Metadata
    pub fn set_metadata(&mut self, value: Vec<String>) {
        self.metadata = value;
    }

    /// Gets the value of Metadata
    pub fn get_metadata(&self) -> &Vec<String> {
        &self.metadata
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Namespace
    pub fn set_namespace(&mut self, value: String) {
        self.namespace = Some(value);
    }

    /// Gets the value of Namespace
    pub fn get_namespace(&self) -> Option<&String> {
        self.namespace.as_ref()
    }

    /// Sets the value of Opened
    pub fn set_opened(&mut self, value: String) {
        self.opened = Some(value);
    }

    /// Gets the value of Opened
    pub fn get_opened(&self) -> Option<&String> {
        self.opened.as_ref()
    }

    /// Sets the value of OutgoingChannels
    pub fn set_outgoing_channels(&mut self, value: Vec<Channel>) {
        self.outgoing_channels = value;
    }

    /// Gets the value of OutgoingChannels
    pub fn get_outgoing_channels(&self) -> &Vec<Channel> {
        &self.outgoing_channels
    }

    /// Sets the value of ProcessId
    pub fn set_process_id(&mut self, value: i32) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessId
    pub fn get_process_id(&self) -> Option<&i32> {
        self.process_id.as_ref()
    }
}

