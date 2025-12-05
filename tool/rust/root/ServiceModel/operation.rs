// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Operation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Operation {

/// The WS-Adressing action of the request message.
    #[serde(rename = "Action")]
    pub action: Option<String>,

/// Indicates that an operation is implemented asynchronously using a Begin<> and End<> method pair in a service contract.
    #[serde(rename = "AsyncPattern")]
    pub async_pattern: Option<bool>,

/// The bhaviors associated with this operation.
    #[serde(rename = "Behaviors")]
    pub behaviors: Vec<Behavior>,

/// True when the operation is a callback operation.
    #[serde(rename = "IsCallback")]
    pub is_callback: Option<bool>,

/// Indicates whether the method implements an operation that can initiate a session on the server.
    #[serde(rename = "IsInitiating")]
    pub is_initiating: Option<bool>,

/// Indicates whether an operation returns a reply message.
    #[serde(rename = "IsOneWay")]
    pub is_one_way: Option<bool>,

/// Indicates whether an operation returns a reply message.
    #[serde(rename = "IsTerminating")]
    pub is_terminating: Option<bool>,

/// The method signature of the operation.
    #[serde(rename = "MethodSignature")]
    pub method_signature: Option<String>,

/// The name of the operation.
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// The types of the parameters of the operation.
    #[serde(rename = "ParameterTypes")]
    pub parameter_types: Vec<String>,

/// The value of the SOAP action for the reply message of the operation.
    #[serde(rename = "ReplyAction")]
    pub reply_action: Option<String>,

/// The return type of the operation.
    #[serde(rename = "ReturnType")]
    pub return_type: Option<String>,
}

impl Operation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            action: None,
            async_pattern: None,
            behaviors: Vec::new(),
            is_callback: None,
            is_initiating: None,
            is_one_way: None,
            is_terminating: None,
            method_signature: None,
            name: None,
            parameter_types: Vec::new(),
            reply_action: None,
            return_type: None,
        }
    }


    /// Sets the value of Action
    pub fn set_action(&mut self, value: String) {
        self.action = Some(value);
    }

    /// Gets the value of Action
    pub fn get_action(&self) -> Option<&String> {
        self.action.as_ref()
    }

    /// Sets the value of AsyncPattern
    pub fn set_async_pattern(&mut self, value: bool) {
        self.async_pattern = Some(value);
    }

    /// Gets the value of AsyncPattern
    pub fn get_async_pattern(&self) -> Option<&bool> {
        self.async_pattern.as_ref()
    }

    /// Sets the value of Behaviors
    pub fn set_behaviors(&mut self, value: Vec<Behavior>) {
        self.behaviors = value;
    }

    /// Gets the value of Behaviors
    pub fn get_behaviors(&self) -> &Vec<Behavior> {
        &self.behaviors
    }

    /// Sets the value of IsCallback
    pub fn set_is_callback(&mut self, value: bool) {
        self.is_callback = Some(value);
    }

    /// Gets the value of IsCallback
    pub fn get_is_callback(&self) -> Option<&bool> {
        self.is_callback.as_ref()
    }

    /// Sets the value of IsInitiating
    pub fn set_is_initiating(&mut self, value: bool) {
        self.is_initiating = Some(value);
    }

    /// Gets the value of IsInitiating
    pub fn get_is_initiating(&self) -> Option<&bool> {
        self.is_initiating.as_ref()
    }

    /// Sets the value of IsOneWay
    pub fn set_is_one_way(&mut self, value: bool) {
        self.is_one_way = Some(value);
    }

    /// Gets the value of IsOneWay
    pub fn get_is_one_way(&self) -> Option<&bool> {
        self.is_one_way.as_ref()
    }

    /// Sets the value of IsTerminating
    pub fn set_is_terminating(&mut self, value: bool) {
        self.is_terminating = Some(value);
    }

    /// Gets the value of IsTerminating
    pub fn get_is_terminating(&self) -> Option<&bool> {
        self.is_terminating.as_ref()
    }

    /// Sets the value of MethodSignature
    pub fn set_method_signature(&mut self, value: String) {
        self.method_signature = Some(value);
    }

    /// Gets the value of MethodSignature
    pub fn get_method_signature(&self) -> Option<&String> {
        self.method_signature.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of ParameterTypes
    pub fn set_parameter_types(&mut self, value: Vec<String>) {
        self.parameter_types = value;
    }

    /// Gets the value of ParameterTypes
    pub fn get_parameter_types(&self) -> &Vec<String> {
        &self.parameter_types
    }

    /// Sets the value of ReplyAction
    pub fn set_reply_action(&mut self, value: String) {
        self.reply_action = Some(value);
    }

    /// Gets the value of ReplyAction
    pub fn get_reply_action(&self) -> Option<&String> {
        self.reply_action.as_ref()
    }

    /// Sets the value of ReturnType
    pub fn set_return_type(&mut self, value: String) {
        self.return_type = Some(value);
    }

    /// Gets the value of ReturnType
    pub fn get_return_type(&self) -> Option<&String> {
        self.return_type.as_ref()
    }
}

