// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CallbackBehavior struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CallbackBehavior {
    #[serde(flatten)]
    pub base: Behavior,

/// When true the session is automatically closed when a service closes a duplex session.
    #[serde(rename = "AutomaticSessionShutdown")]
    pub automatic_session_shutdown: Option<bool>,

/// Specifies whether the service supports one thread, multiple threads, or reentrant calls.
    #[serde(rename = "ConcurrencyMode")]
    pub concurrency_mode: Option<String>,

/// A value that specifies whether to send unknown serialization data onto the wire.
    #[serde(rename = "IgnoreExtensionDataObject")]
    pub ignore_extension_data_object: Option<bool>,

/// When enabled details about exceptions on the callback are attached to the faults returned to the service.
    #[serde(rename = "IncludeExceptionDetailInFaults")]
    pub include_exception_detail_in_faults: Option<bool>,

/// The maximum number of items allowed in a serialized object.
    #[serde(rename = "MaxItemsInObjectGraph")]
    pub max_items_in_object_graph: Option<bool>,

/// Specifies whether to use the current synchronization context to choose the thread of execution.
    #[serde(rename = "UseSynchronizationContext")]
    pub use_synchronization_context: Option<bool>,

/// Specifies whether the system or the application enforces SOAP MustUnderstand header processing.
    #[serde(rename = "ValidateMustUnderstand")]
    pub validate_must_understand: Option<bool>,
}

impl CallbackBehavior {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Behavior::new(),
            automatic_session_shutdown: None,
            concurrency_mode: None,
            ignore_extension_data_object: None,
            include_exception_detail_in_faults: None,
            max_items_in_object_graph: None,
            use_synchronization_context: None,
            validate_must_understand: None,
        }
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

    /// Sets the value of MaxItemsInObjectGraph
    pub fn set_max_items_in_object_graph(&mut self, value: bool) {
        self.max_items_in_object_graph = Some(value);
    }

    /// Gets the value of MaxItemsInObjectGraph
    pub fn get_max_items_in_object_graph(&self) -> Option<&bool> {
        self.max_items_in_object_graph.as_ref()
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

