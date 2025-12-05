// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Service struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Service {
    #[serde(flatten)]
    pub base: CIM_EnabledLogicalElement,

/// CreationClassName indicates the name of the class or the subclass that is used in the creation of an instance. When used with the other key properties of this class, this property allows all instances of this class and its subclasses to be uniquely identified.
    #[serde(rename = "CreationClassName")]
    pub creation_class_name: Option<String>,

/// A string that provides information on how the primary owner of the Service can be reached (for example, phone number, e-mail address, and so on).
    #[serde(rename = "PrimaryOwnerContact")]
    pub primary_owner_contact: Option<String>,

/// The name of the primary owner for the service, if one is defined. The primary owner is the initial support contact for the Service.
    #[serde(rename = "PrimaryOwnerName")]
    pub primary_owner_name: Option<String>,

/// Started is a Boolean that indicates whether the Service has been started (TRUE), or stopped (FALSE).
    #[serde(rename = "Started")]
    pub started: Option<bool>,

/// Note: The use of this element is deprecated in lieu of the EnabledDefault property that is inherited from EnabledLogicalElement. The EnabledLogicalElement addresses the same semantics. The change to a uint16 data type was discussed when CIM V2.0 was defined. However, existing V1.0 implementations used the string property. To remain compatible with those implementations, StartMode was grandfathered into the schema. Use of the deprecated qualifier allows the maintenance of the existing property but also permits an improved, clarified definition using EnabledDefault. 
/// Deprecated description: StartMode is a string value that indicates whether the Service is automatically started by a System, an Operating System, and so on, or is started only upon request.
    #[serde(rename = "StartMode")]
    pub start_mode: Option<String>,

/// The CreationClassName of the scoping System.
    #[serde(rename = "SystemCreationClassName")]
    pub system_creation_class_name: Option<String>,

/// The Name of the scoping System.
    #[serde(rename = "SystemName")]
    pub system_name: Option<String>,
}

impl CIM_Service {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_EnabledLogicalElement::new(),
            creation_class_name: None,
            primary_owner_contact: None,
            primary_owner_name: None,
            started: None,
            start_mode: None,
            system_creation_class_name: None,
            system_name: None,
        }
    }


    /// Sets the value of CreationClassName
    pub fn set_creation_class_name(&mut self, value: String) {
        self.creation_class_name = Some(value);
    }

    /// Gets the value of CreationClassName
    pub fn get_creation_class_name(&self) -> Option<&String> {
        self.creation_class_name.as_ref()
    }

    /// Sets the value of PrimaryOwnerContact
    pub fn set_primary_owner_contact(&mut self, value: String) {
        self.primary_owner_contact = Some(value);
    }

    /// Gets the value of PrimaryOwnerContact
    pub fn get_primary_owner_contact(&self) -> Option<&String> {
        self.primary_owner_contact.as_ref()
    }

    /// Sets the value of PrimaryOwnerName
    pub fn set_primary_owner_name(&mut self, value: String) {
        self.primary_owner_name = Some(value);
    }

    /// Gets the value of PrimaryOwnerName
    pub fn get_primary_owner_name(&self) -> Option<&String> {
        self.primary_owner_name.as_ref()
    }

    /// Sets the value of Started
    pub fn set_started(&mut self, value: bool) {
        self.started = Some(value);
    }

    /// Gets the value of Started
    pub fn get_started(&self) -> Option<&bool> {
        self.started.as_ref()
    }

    /// Sets the value of StartMode
    pub fn set_start_mode(&mut self, value: String) {
        self.start_mode = Some(value);
    }

    /// Gets the value of StartMode
    pub fn get_start_mode(&self) -> Option<&String> {
        self.start_mode.as_ref()
    }

    /// Sets the value of SystemCreationClassName
    pub fn set_system_creation_class_name(&mut self, value: String) {
        self.system_creation_class_name = Some(value);
    }

    /// Gets the value of SystemCreationClassName
    pub fn get_system_creation_class_name(&self) -> Option<&String> {
        self.system_creation_class_name.as_ref()
    }

    /// Sets the value of SystemName
    pub fn set_system_name(&mut self, value: String) {
        self.system_name = Some(value);
    }

    /// Gets the value of SystemName
    pub fn get_system_name(&self) -> Option<&String> {
        self.system_name.as_ref()
    }

/// The StartService method places the Service in the started state. Note that the function of this method overlaps with the RequestedState property. RequestedState was added to the model to maintain a record (such as a persisted value) of the last state request. Invoking the StartService method should set the RequestedState property appropriately. The method returns an integer value of 0 if the Service was successfully started, 1 if the request is not supported, and any other number to indicate an error. In a subclass, the set of possible return codes could be specified using a ValueMap qualifier on the method. The strings to which the ValueMap contents are translated can also be specified in the subclass as a Values array qualifier. 
/// 
/// Note: The semantics of this method overlap with the RequestStateChange method that is inherited from EnabledLogicalElement. This method is maintained because it has been widely implemented, and its simple "start" semantics are convenient to use.

    /// * `return_value` -  (u32)
    pub fn start_service(&self) -> Result<(), WmiError> {
        self.invoke_method("StartService", &[])

    }


/// The StopService method places the Service in the stopped state. Note that the function of this method overlaps with the RequestedState property. RequestedState was added to the model to maintain a record (such as a persisted value) of the last state request. Invoking the StopService method should set the RequestedState property appropriately. The method returns an integer value of 0 if the Service was successfully stopped, 1 if the request is not supported, and any other number to indicate an error. In a subclass, the set of possible return codes could be specified using a ValueMap qualifier on the method. The strings to which the ValueMap contents are translated can also be specified in the subclass as a Values array qualifier. 
/// 
/// Note: The semantics of this method overlap with the RequestStateChange method that is inherited from EnabledLogicalElement. This method is maintained because it has been widely implemented, and its simple "stop" semantics are convenient to use.

    /// * `return_value` -  (u32)
    pub fn stop_service(&self) -> Result<(), WmiError> {
        self.invoke_method("StopService", &[])

    }

}

