// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ServiceBehaviorAttribute struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceBehaviorAttribute {
    #[serde(flatten)]
    pub base: Behavior,

/// Specifies how to use address to filter a message, whether to use Exact match, Prefix match or match Any.
    #[serde(rename = "AddressFilterMode")]
    pub address_filter_mode: Option<String>,

/// Indicates whether to automatically close a session when a client closes an output session.
    #[serde(rename = "AutomaticSessionShutdown")]
    pub automatic_session_shutdown: Option<bool>,

/// Indicates whether a service supports one thread, multiple threads, or reentrant calls.
    #[serde(rename = "ConcurrencyMode")]
    pub concurrency_mode: Option<String>,

/// The name of the service configuration.
    #[serde(rename = "ConfigurationName")]
    pub configuration_name: Option<String>,

/// Specifies whether to process multiple messages concurrently at the dispatcher layer.
    #[serde(rename = "EnsureOrderedDispatch")]
    pub ensure_ordered_dispatch: Option<bool>,

/// Specifies whether to send unknown serlialization data onto the wire.
    #[serde(rename = "IgnoreExtensionDataObject")]
    pub ignore_extension_data_object: Option<bool>,

/// Specifies whether to include managed exception information in the detail of SOAP faults returned to the clients for debugging purposes.
    #[serde(rename = "IncludeExceptionDetailInFaults")]
    pub include_exception_detail_in_faults: Option<bool>,

/// Specifies when a new service object is created.
    #[serde(rename = "InstanceContextMode")]
    pub instance_context_mode: Option<String>,

/// The maximum number of items allowed in a serialized object.
    #[serde(rename = "MaxItemsInObjectGraph")]
    pub max_items_in_object_graph: Option<i32>,

/// The name attribute of the service in WSDL.
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// The target namespace of the service in WSDL.
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,

/// Specifies whether the service object is recycled when the current transaction completes.
    #[serde(rename = "ReleaseServiceInstanceOnTransactionComplete")]
    pub release_service_instance_on_transaction_complete: Option<bool>,

/// Specifies whether pending transactions are completed when the current session closes.
    #[serde(rename = "TransactionAutoCompleteOnSessionClose")]
    pub transaction_auto_complete_on_session_close: Option<bool>,

/// Specifies the transaction isolation level.
    #[serde(rename = "TransactionIsolationLevel")]
    pub transaction_isolation_level: Option<String>,

/// The period within which a transaction must complete.
    #[serde(rename = "TransactionTimeout")]
    pub transaction_timeout: Option<String>,

/// Specifies whether to use the current synchronization context to choose the thread execution.
    #[serde(rename = "UseSynchronizationContext")]
    pub use_synchronization_context: Option<bool>,

/// Specifies whether the system or the application enforces SOAP MustUnderstand header processing.
    #[serde(rename = "ValidateMustUnderstand")]
    pub validate_must_understand: Option<bool>,
}

impl ServiceBehaviorAttribute {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Behavior::new(),
            address_filter_mode: None,
            automatic_session_shutdown: None,
            concurrency_mode: None,
            configuration_name: None,
            ensure_ordered_dispatch: None,
            ignore_extension_data_object: None,
            include_exception_detail_in_faults: None,
            instance_context_mode: None,
            max_items_in_object_graph: None,
            name: None,
            namespace: None,
            release_service_instance_on_transaction_complete: None,
            transaction_auto_complete_on_session_close: None,
            transaction_isolation_level: None,
            transaction_timeout: None,
            use_synchronization_context: None,
            validate_must_understand: None,
        }
    }


    /// Sets the value of AddressFilterMode
    pub fn set_address_filter_mode(&mut self, value: String) {
        self.address_filter_mode = Some(value);
    }

    /// Gets the value of AddressFilterMode
    pub fn get_address_filter_mode(&self) -> Option<&String> {
        self.address_filter_mode.as_ref()
    }

    /// Sets the value of AutomaticSessionShutdown
    pub fn set_automatic_session_shutdown(&mut self, value: bool) {
        self.automatic_session_shutdown = Some(value);
    }

    /// Gets the value of AutomaticSessionShutdown
    pub fn get_automatic_session_shutdown(&self) -> Option<&bool> {
        self.automatic_session_shutdown.as_ref()
    }

    /// Sets the value of ConcurrencyMode
    pub fn set_concurrency_mode(&mut self, value: String) {
        self.concurrency_mode = Some(value);
    }

    /// Gets the value of ConcurrencyMode
    pub fn get_concurrency_mode(&self) -> Option<&String> {
        self.concurrency_mode.as_ref()
    }

    /// Sets the value of ConfigurationName
    pub fn set_configuration_name(&mut self, value: String) {
        self.configuration_name = Some(value);
    }

    /// Gets the value of ConfigurationName
    pub fn get_configuration_name(&self) -> Option<&String> {
        self.configuration_name.as_ref()
    }

    /// Sets the value of EnsureOrderedDispatch
    pub fn set_ensure_ordered_dispatch(&mut self, value: bool) {
        self.ensure_ordered_dispatch = Some(value);
    }

    /// Gets the value of EnsureOrderedDispatch
    pub fn get_ensure_ordered_dispatch(&self) -> Option<&bool> {
        self.ensure_ordered_dispatch.as_ref()
    }

    /// Sets the value of IgnoreExtensionDataObject
    pub fn set_ignore_extension_data_object(&mut self, value: bool) {
        self.ignore_extension_data_object = Some(value);
    }

    /// Gets the value of IgnoreExtensionDataObject
    pub fn get_ignore_extension_data_object(&self) -> Option<&bool> {
        self.ignore_extension_data_object.as_ref()
    }

    /// Sets the value of IncludeExceptionDetailInFaults
    pub fn set_include_exception_detail_in_faults(&mut self, value: bool) {
        self.include_exception_detail_in_faults = Some(value);
    }

    /// Gets the value of IncludeExceptionDetailInFaults
    pub fn get_include_exception_detail_in_faults(&self) -> Option<&bool> {
        self.include_exception_detail_in_faults.as_ref()
    }

    /// Sets the value of InstanceContextMode
    pub fn set_instance_context_mode(&mut self, value: String) {
        self.instance_context_mode = Some(value);
    }

    /// Gets the value of InstanceContextMode
    pub fn get_instance_context_mode(&self) -> Option<&String> {
        self.instance_context_mode.as_ref()
    }

    /// Sets the value of MaxItemsInObjectGraph
    pub fn set_max_items_in_object_graph(&mut self, value: i32) {
        self.max_items_in_object_graph = Some(value);
    }

    /// Gets the value of MaxItemsInObjectGraph
    pub fn get_max_items_in_object_graph(&self) -> Option<&i32> {
        self.max_items_in_object_graph.as_ref()
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

    /// Sets the value of ReleaseServiceInstanceOnTransactionComplete
    pub fn set_release_service_instance_on_transaction_complete(&mut self, value: bool) {
        self.release_service_instance_on_transaction_complete = Some(value);
    }

    /// Gets the value of ReleaseServiceInstanceOnTransactionComplete
    pub fn get_release_service_instance_on_transaction_complete(&self) -> Option<&bool> {
        self.release_service_instance_on_transaction_complete.as_ref()
    }

    /// Sets the value of TransactionAutoCompleteOnSessionClose
    pub fn set_transaction_auto_complete_on_session_close(&mut self, value: bool) {
        self.transaction_auto_complete_on_session_close = Some(value);
    }

    /// Gets the value of TransactionAutoCompleteOnSessionClose
    pub fn get_transaction_auto_complete_on_session_close(&self) -> Option<&bool> {
        self.transaction_auto_complete_on_session_close.as_ref()
    }

    /// Sets the value of TransactionIsolationLevel
    pub fn set_transaction_isolation_level(&mut self, value: String) {
        self.transaction_isolation_level = Some(value);
    }

    /// Gets the value of TransactionIsolationLevel
    pub fn get_transaction_isolation_level(&self) -> Option<&String> {
        self.transaction_isolation_level.as_ref()
    }

    /// Sets the value of TransactionTimeout
    pub fn set_transaction_timeout(&mut self, value: String) {
        self.transaction_timeout = Some(value);
    }

    /// Gets the value of TransactionTimeout
    pub fn get_transaction_timeout(&self) -> Option<&String> {
        self.transaction_timeout.as_ref()
    }

    /// Sets the value of UseSynchronizationContext
    pub fn set_use_synchronization_context(&mut self, value: bool) {
        self.use_synchronization_context = Some(value);
    }

    /// Gets the value of UseSynchronizationContext
    pub fn get_use_synchronization_context(&self) -> Option<&bool> {
        self.use_synchronization_context.as_ref()
    }

    /// Sets the value of ValidateMustUnderstand
    pub fn set_validate_must_understand(&mut self, value: bool) {
        self.validate_must_understand = Some(value);
    }

    /// Gets the value of ValidateMustUnderstand
    pub fn get_validate_must_understand(&self) -> Option<&bool> {
        self.validate_must_understand.as_ref()
    }
}

