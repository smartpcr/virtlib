// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ProtocolEndpoint struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ProtocolEndpoint {
    #[serde(flatten)]
    pub base: CIM_ServiceAccessPoint,

/// NameFormat contains the naming heuristic that is selected to ensure that the value of the Name property is unique. For example, you might choose to prepend the name of the port or interface with the Type of ProtocolEndpoint (for example, IPv4) of this instance followed by an underscore.
    #[serde(rename = "NameFormat")]
    pub name_format: Option<String>,

/// A string that describes the type of ProtocolEndpoint when the Type property of this class (or any of its subclasses) is set to 1 (Other). This property should be set to null when the Type property is any value other than 1.
    #[serde(rename = "OtherTypeDescription")]
    pub other_type_description: Option<String>,

/// ProtocolIFType is an enumeration that is synchronized with the IANA ifType MIB. The ifType MIB is maintained at the URL, http://www.iana.org/assignments/ianaiftype-mib. Also, additional values defined by the DMTF are included. The property is used to categorize and classify instances of the ProtocolEndpoint class. Note that if the ProtocolIFType is set to 1 (Other), then the type information should be provided in the OtherTypeDescription string property.
    #[serde(rename = "ProtocolIFType")]
    pub protocol_iftype: Option<ProtocolEndpoint_ProtocolIFType>,

/// Note: This property is deprecated in lieu of the ProtocolIFType enumeration. This deprecation was done to have better alignment between the IF-MIB of the IETF and this CIM class. 
/// Deprecated description: ProtocolType is an enumeration that provides information to categorize and classify different instances of this class. For most instances, information in this enumeration and the definition of the subclass overlap. However, there are several cases where a specific subclass of ProtocolEndpoint is not required (for example, there is no Fibre Channel subclass of ProtocolEndpoint). Therefore, this property is needed to define the type of Endpoint.
    #[serde(rename = "ProtocolType")]
    pub protocol_type: Option<ProtocolEndpoint_ProtocolType>,
}

impl CIM_ProtocolEndpoint {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ServiceAccessPoint::new(),
            name_format: None,
            other_type_description: None,
            protocol_iftype: None,
            protocol_type: None,
        }
    }


    /// Sets the value of NameFormat
    pub fn set_name_format(&mut self, value: String) {
        self.name_format = Some(value);
    }

    /// Gets the value of NameFormat
    pub fn get_name_format(&self) -> Option<&String> {
        self.name_format.as_ref()
    }

    /// Sets the value of OtherTypeDescription
    pub fn set_other_type_description(&mut self, value: String) {
        self.other_type_description = Some(value);
    }

    /// Gets the value of OtherTypeDescription
    pub fn get_other_type_description(&self) -> Option<&String> {
        self.other_type_description.as_ref()
    }

    /// Sets the value of ProtocolIFType
    pub fn set_protocol_iftype(&mut self, value: ProtocolEndpoint_ProtocolIFType) {
        self.protocol_iftype = Some(value);
    }

    /// Gets the value of ProtocolIFType
    pub fn get_protocol_iftype(&self) -> Option<&ProtocolEndpoint_ProtocolIFType> {
        self.protocol_iftype.as_ref()
    }

    /// Sets the value of ProtocolType
    pub fn set_protocol_type(&mut self, value: ProtocolEndpoint_ProtocolType) {
        self.protocol_type = Some(value);
    }

    /// Gets the value of ProtocolType
    pub fn get_protocol_type(&self) -> Option<&ProtocolEndpoint_ProtocolType> {
        self.protocol_type.as_ref()
    }
}

