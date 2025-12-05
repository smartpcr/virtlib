// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WorkflowServiceBehavior struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorkflowServiceBehavior {
    #[serde(flatten)]
    pub base: Behavior,

/// Specifies how to use address to filter a message, whether to use Exact match, Prefix match or match Any.
    #[serde(rename = "AddressFilterMode")]
    pub address_filter_mode: Option<String>,

/// The name of the service configuration.
    #[serde(rename = "ConfigurationName")]
    pub configuration_name: Option<String>,

/// Specifies whether to send unknown serlialization data onto the wire.
    #[serde(rename = "IgnoreExtensionDataObject")]
    pub ignore_extension_data_object: Option<bool>,

/// Specifies whether to include managed exception information in the detail of SOAP faults returned to the clients for debugging purposes.
    #[serde(rename = "IncludeExceptionDetailInFaults")]
    pub include_exception_detail_in_faults: Option<bool>,

/// The maximum number of items allowed in a serialized object.
    #[serde(rename = "MaxItemsInObjectGraph")]
    pub max_items_in_object_graph: Option<i32>,

/// The name attribute of the service in WSDL.
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// The target namespace of the service in WSDL.
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,

/// Specifies whether to use the current synchronization context to choose the thread execution.
    #[serde(rename = "UseSynchronizationContext")]
    pub use_synchronization_context: Option<bool>,

/// Specifies whether the system or the application enforces SOAP MustUnderstand header processing.
    #[serde(rename = "ValidateMustUnderstand")]
    pub validate_must_understand: Option<bool>,

/// Specifies the path to XOML defined Workflow.
    #[serde(rename = "WorkflowDefinitionPath")]
    pub workflow_definition_path: Option<String>,

/// Specifies the path to Rules file for a XOML defined Workflow.
    #[serde(rename = "WorkflowRulesPath")]
    pub workflow_rules_path: Option<String>,

/// Specifies the Type of Workflow.
    #[serde(rename = "WorkflowType")]
    pub workflow_type: Option<String>,
}

impl WorkflowServiceBehavior {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Behavior::new(),
            address_filter_mode: None,
            configuration_name: None,
            ignore_extension_data_object: None,
            include_exception_detail_in_faults: None,
            max_items_in_object_graph: None,
            name: None,
            namespace: None,
            use_synchronization_context: None,
            validate_must_understand: None,
            workflow_definition_path: None,
            workflow_rules_path: None,
            workflow_type: None,
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

    /// Sets the value of ConfigurationName
    pub fn set_configuration_name(&mut self, value: String) {
        self.configuration_name = Some(value);
    }

    /// Gets the value of ConfigurationName
    pub fn get_configuration_name(&self) -> Option<&String> {
        self.configuration_name.as_ref()
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

    /// Sets the value of WorkflowDefinitionPath
    pub fn set_workflow_definition_path(&mut self, value: String) {
        self.workflow_definition_path = Some(value);
    }

    /// Gets the value of WorkflowDefinitionPath
    pub fn get_workflow_definition_path(&self) -> Option<&String> {
        self.workflow_definition_path.as_ref()
    }

    /// Sets the value of WorkflowRulesPath
    pub fn set_workflow_rules_path(&mut self, value: String) {
        self.workflow_rules_path = Some(value);
    }

    /// Gets the value of WorkflowRulesPath
    pub fn get_workflow_rules_path(&self) -> Option<&String> {
        self.workflow_rules_path.as_ref()
    }

    /// Sets the value of WorkflowType
    pub fn set_workflow_type(&mut self, value: String) {
        self.workflow_type = Some(value);
    }

    /// Gets the value of WorkflowType
    pub fn get_workflow_type(&self) -> Option<&String> {
        self.workflow_type.as_ref()
    }
}

