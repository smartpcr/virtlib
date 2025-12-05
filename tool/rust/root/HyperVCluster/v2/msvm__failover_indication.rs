// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_FailoverIndication struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_FailoverIndication {
    #[serde(flatten)]
    pub base: CIM_ProcessIndication,

/// 
    #[serde(rename = "FailoverType")]
    pub failover_type: Option<u16>,

/// 
    #[serde(rename = "HostedElement")]
    pub hosted_element: Option<String>,

/// 
    #[serde(rename = "HostedElementFormat")]
    pub hosted_element_format: Option<u16>,

/// 
    #[serde(rename = "HostingSystem")]
    pub hosting_system: Option<String>,

/// 
    #[serde(rename = "HostingSystemFormat")]
    pub hosting_system_format: Option<u16>,

/// 
    #[serde(rename = "OtherFailoverType")]
    pub other_failover_type: Option<String>,

/// 
    #[serde(rename = "OtherHostedElementFormat")]
    pub other_hosted_element_format: Option<String>,

/// 
    #[serde(rename = "OtherHostingSystemFormat")]
    pub other_hosting_system_format: Option<String>,

/// 
    #[serde(rename = "OtherPerceivedSeverity")]
    pub other_perceived_severity: Option<String>,

/// 
    #[serde(rename = "OtherPreviousHostingSystemFormat")]
    pub other_previous_hosting_system_format: Option<String>,

/// 
    #[serde(rename = "PreviousHostingSystem")]
    pub previous_hosting_system: Option<String>,

/// 
    #[serde(rename = "PreviousHostingSystemFormat")]
    pub previous_hosting_system_format: Option<u16>,
}

impl Msvm_FailoverIndication {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ProcessIndication::new(),
            failover_type: None,
            hosted_element: None,
            hosted_element_format: None,
            hosting_system: None,
            hosting_system_format: None,
            other_failover_type: None,
            other_hosted_element_format: None,
            other_hosting_system_format: None,
            other_perceived_severity: None,
            other_previous_hosting_system_format: None,
            previous_hosting_system: None,
            previous_hosting_system_format: None,
        }
    }


    /// Sets the value of FailoverType
    pub fn set_failover_type(&mut self, value: u16) {
        self.failover_type = Some(value);
    }

    /// Gets the value of FailoverType
    pub fn get_failover_type(&self) -> Option<&u16> {
        self.failover_type.as_ref()
    }

    /// Sets the value of HostedElement
    pub fn set_hosted_element(&mut self, value: String) {
        self.hosted_element = Some(value);
    }

    /// Gets the value of HostedElement
    pub fn get_hosted_element(&self) -> Option<&String> {
        self.hosted_element.as_ref()
    }

    /// Sets the value of HostedElementFormat
    pub fn set_hosted_element_format(&mut self, value: u16) {
        self.hosted_element_format = Some(value);
    }

    /// Gets the value of HostedElementFormat
    pub fn get_hosted_element_format(&self) -> Option<&u16> {
        self.hosted_element_format.as_ref()
    }

    /// Sets the value of HostingSystem
    pub fn set_hosting_system(&mut self, value: String) {
        self.hosting_system = Some(value);
    }

    /// Gets the value of HostingSystem
    pub fn get_hosting_system(&self) -> Option<&String> {
        self.hosting_system.as_ref()
    }

    /// Sets the value of HostingSystemFormat
    pub fn set_hosting_system_format(&mut self, value: u16) {
        self.hosting_system_format = Some(value);
    }

    /// Gets the value of HostingSystemFormat
    pub fn get_hosting_system_format(&self) -> Option<&u16> {
        self.hosting_system_format.as_ref()
    }

    /// Sets the value of OtherFailoverType
    pub fn set_other_failover_type(&mut self, value: String) {
        self.other_failover_type = Some(value);
    }

    /// Gets the value of OtherFailoverType
    pub fn get_other_failover_type(&self) -> Option<&String> {
        self.other_failover_type.as_ref()
    }

    /// Sets the value of OtherHostedElementFormat
    pub fn set_other_hosted_element_format(&mut self, value: String) {
        self.other_hosted_element_format = Some(value);
    }

    /// Gets the value of OtherHostedElementFormat
    pub fn get_other_hosted_element_format(&self) -> Option<&String> {
        self.other_hosted_element_format.as_ref()
    }

    /// Sets the value of OtherHostingSystemFormat
    pub fn set_other_hosting_system_format(&mut self, value: String) {
        self.other_hosting_system_format = Some(value);
    }

    /// Gets the value of OtherHostingSystemFormat
    pub fn get_other_hosting_system_format(&self) -> Option<&String> {
        self.other_hosting_system_format.as_ref()
    }

    /// Sets the value of OtherPerceivedSeverity
    pub fn set_other_perceived_severity(&mut self, value: String) {
        self.other_perceived_severity = Some(value);
    }

    /// Gets the value of OtherPerceivedSeverity
    pub fn get_other_perceived_severity(&self) -> Option<&String> {
        self.other_perceived_severity.as_ref()
    }

    /// Sets the value of OtherPreviousHostingSystemFormat
    pub fn set_other_previous_hosting_system_format(&mut self, value: String) {
        self.other_previous_hosting_system_format = Some(value);
    }

    /// Gets the value of OtherPreviousHostingSystemFormat
    pub fn get_other_previous_hosting_system_format(&self) -> Option<&String> {
        self.other_previous_hosting_system_format.as_ref()
    }

    /// Sets the value of PreviousHostingSystem
    pub fn set_previous_hosting_system(&mut self, value: String) {
        self.previous_hosting_system = Some(value);
    }

    /// Gets the value of PreviousHostingSystem
    pub fn get_previous_hosting_system(&self) -> Option<&String> {
        self.previous_hosting_system.as_ref()
    }

    /// Sets the value of PreviousHostingSystemFormat
    pub fn set_previous_hosting_system_format(&mut self, value: u16) {
        self.previous_hosting_system_format = Some(value);
    }

    /// Gets the value of PreviousHostingSystemFormat
    pub fn get_previous_hosting_system_format(&self) -> Option<&u16> {
        self.previous_hosting_system_format.as_ref()
    }
}

