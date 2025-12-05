// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Contract struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Contract {

/// The appdomain id of the appdomain that hosts the contract.
    #[serde(rename = "AppDomainId")]
    pub app_domain_id: Option<i32>,

/// The behaviors associated with this contract.
    #[serde(rename = "Behaviors")]
    pub behaviors: Vec<Behavior>,

/// The type of callback when the contract is a duplex contract.
    #[serde(rename = "CallbackContract")]
    pub callback_contract: Option<Contract>,

/// The name of the contract in WSDL.
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// The namespace of the <portType> element in WSDL.
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,

/// The operations of this contract.
    #[serde(rename = "Operations")]
    pub operations: Vec<Operation>,

/// The process Id of the process that hosts the contract.
    #[serde(rename = "ProcessId")]
    pub process_id: Option<i32>,

/// Indicates whether the contract requires the binding associated with this contract to use channel sessions.
    #[serde(rename = "SessionMode")]
    pub session_mode: Option<String>,

/// The type of the contract.
    #[serde(rename = "Type")]
    pub type: Option<String>,
}

impl Contract {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            app_domain_id: None,
            behaviors: Vec::new(),
            callback_contract: None,
            name: None,
            namespace: None,
            operations: Vec::new(),
            process_id: None,
            session_mode: None,
            type: None,
        }
    }


    /// Sets the value of AppDomainId
    pub fn set_app_domain_id(&mut self, value: i32) {
        self.app_domain_id = Some(value);
    }

    /// Gets the value of AppDomainId
    pub fn get_app_domain_id(&self) -> Option<&i32> {
        self.app_domain_id.as_ref()
    }

    /// Sets the value of Behaviors
    pub fn set_behaviors(&mut self, value: Vec<Behavior>) {
        self.behaviors = value;
    }

    /// Gets the value of Behaviors
    pub fn get_behaviors(&self) -> &Vec<Behavior> {
        &self.behaviors
    }

    /// Sets the value of CallbackContract
    pub fn set_callback_contract(&mut self, value: Contract) {
        self.callback_contract = Some(value);
    }

    /// Gets the value of CallbackContract
    pub fn get_callback_contract(&self) -> Option<&Contract> {
        self.callback_contract.as_ref()
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

    /// Sets the value of Operations
    pub fn set_operations(&mut self, value: Vec<Operation>) {
        self.operations = value;
    }

    /// Gets the value of Operations
    pub fn get_operations(&self) -> &Vec<Operation> {
        &self.operations
    }

    /// Sets the value of ProcessId
    pub fn set_process_id(&mut self, value: i32) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessId
    pub fn get_process_id(&self) -> Option<&i32> {
        self.process_id.as_ref()
    }

    /// Sets the value of SessionMode
    pub fn set_session_mode(&mut self, value: String) {
        self.session_mode = Some(value);
    }

    /// Gets the value of SessionMode
    pub fn get_session_mode(&self) -> Option<&String> {
        self.session_mode.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: String) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&String> {
        self.type.as_ref()
    }
}

