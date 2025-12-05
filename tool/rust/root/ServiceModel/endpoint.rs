// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Endpoint struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Endpoint {

/// A Uri that contains the address of the endpoint.
    #[serde(rename = "Address")]
    pub address: Option<String>,

///  The collection of address headers attached to this endpoint.
    #[serde(rename = "AddressHeaders")]
    pub address_headers: Vec<String>,

/// The identity of the endpoint.
    #[serde(rename = "AddressIdentity")]
    pub address_identity: Option<String>,

/// The appdomain id of the appdomain that hosts the endpoint.
    #[serde(rename = "AppDomainId")]
    pub app_domain_id: Option<i32>,

/// The collection of behaviors implemented by this endpoint.
    #[serde(rename = "Behaviors")]
    pub behaviors: Vec<Behavior>,

/// The binding used by this endpoint.
    #[serde(rename = "Binding")]
    pub binding: Option<Binding>,

/// The contract this endpoint is exposing.
    #[serde(rename = "Contract")]
    pub contract: Option<Contract>,

/// A string that specifies which contract this endpoint is exposing.
    #[serde(rename = "ContractName")]
    pub contract_name: Option<String>,

/// The name of the instance of performance counters of the endpoint.
    #[serde(rename = "CounterInstanceName")]
    pub counter_instance_name: Option<String>,

/// The Uri the endpoint listens on.
    #[serde(rename = "ListenUri")]
    pub listen_uri: Option<String>,

/// The unique name of this endpoint.
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// The process Id of the process that hosts the endpoint.
    #[serde(rename = "ProcessId")]
    pub process_id: Option<i32>,
}

impl Endpoint {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            address: None,
            address_headers: Vec::new(),
            address_identity: None,
            app_domain_id: None,
            behaviors: Vec::new(),
            binding: None,
            contract: None,
            contract_name: None,
            counter_instance_name: None,
            listen_uri: None,
            name: None,
            process_id: None,
        }
    }


    /// Sets the value of Address
    pub fn set_address(&mut self, value: String) {
        self.address = Some(value);
    }

    /// Gets the value of Address
    pub fn get_address(&self) -> Option<&String> {
        self.address.as_ref()
    }

    /// Sets the value of AddressHeaders
    pub fn set_address_headers(&mut self, value: Vec<String>) {
        self.address_headers = value;
    }

    /// Gets the value of AddressHeaders
    pub fn get_address_headers(&self) -> &Vec<String> {
        &self.address_headers
    }

    /// Sets the value of AddressIdentity
    pub fn set_address_identity(&mut self, value: String) {
        self.address_identity = Some(value);
    }

    /// Gets the value of AddressIdentity
    pub fn get_address_identity(&self) -> Option<&String> {
        self.address_identity.as_ref()
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

    /// Sets the value of Binding
    pub fn set_binding(&mut self, value: Binding) {
        self.binding = Some(value);
    }

    /// Gets the value of Binding
    pub fn get_binding(&self) -> Option<&Binding> {
        self.binding.as_ref()
    }

    /// Sets the value of Contract
    pub fn set_contract(&mut self, value: Contract) {
        self.contract = Some(value);
    }

    /// Gets the value of Contract
    pub fn get_contract(&self) -> Option<&Contract> {
        self.contract.as_ref()
    }

    /// Sets the value of ContractName
    pub fn set_contract_name(&mut self, value: String) {
        self.contract_name = Some(value);
    }

    /// Gets the value of ContractName
    pub fn get_contract_name(&self) -> Option<&String> {
        self.contract_name.as_ref()
    }

    /// Sets the value of CounterInstanceName
    pub fn set_counter_instance_name(&mut self, value: String) {
        self.counter_instance_name = Some(value);
    }

    /// Gets the value of CounterInstanceName
    pub fn get_counter_instance_name(&self) -> Option<&String> {
        self.counter_instance_name.as_ref()
    }

    /// Sets the value of ListenUri
    pub fn set_listen_uri(&mut self, value: String) {
        self.listen_uri = Some(value);
    }

    /// Gets the value of ListenUri
    pub fn get_listen_uri(&self) -> Option<&String> {
        self.listen_uri.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of ProcessId
    pub fn set_process_id(&mut self, value: i32) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessId
    pub fn get_process_id(&self) -> Option<&i32> {
        self.process_id.as_ref()
    }

/// Retrieves operation performance counter instance name

    /// * `operation` -  (String)

    /// * `return_value` -  (String)
    pub fn get_operation_counter_instance_name(&self, operation: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Operation".to_string(), value: operation.into() });
        self.invoke_method("GetOperationCounterInstanceName", &args)

    }

}

