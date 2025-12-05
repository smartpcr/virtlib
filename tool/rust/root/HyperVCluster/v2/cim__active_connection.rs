// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ActiveConnection struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ActiveConnection {
    #[serde(flatten)]
    pub base: CIM_SAPSAPDependency,

/// TRUE means that this connection is unidirectional; FALSE means that this connection is bidirectional. When the connection is unidirectional, the "speaker" should be defined as the Antecedent reference. In a bidirectional connection, the selection of which AccessPoint is the Antecedent or Dependent is immaterial.
    #[serde(rename = "IsUnidirectional")]
    pub is_unidirectional: Option<bool>,

/// Note: The use of this element is deprecated because it is incorrectly placed on the association. Unicast, broadcast, or other traffic types are not a function of the connection between the referenced endpoints, but rather are a function of the addressing, protocol and basic functionality of the endpoints. 
/// Deprecated description: A string that describes the type of traffic that is being carried over this instance when its Type property is set, for example, to 1 (Other).
    #[serde(rename = "OtherTrafficDescription")]
    pub other_traffic_description: Option<String>,

/// Note: The use of this element is deprecated because it is incorrectly placed on the association. Unicast, broadcast, or other traffic types are not a function of the connection between the referenced endpoints, but rather are a function of the addressing, protocol and basic functionality of the endpoints. 
/// Deprecated description: The type of traffic that is carried over this connection.
    #[serde(rename = "TrafficType")]
    pub traffic_type: Option<ActiveConnection_TrafficType>,
}

impl CIM_ActiveConnection {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SAPSAPDependency::new(),
            is_unidirectional: None,
            other_traffic_description: None,
            traffic_type: None,
        }
    }


    /// Sets the value of IsUnidirectional
    pub fn set_is_unidirectional(&mut self, value: bool) {
        self.is_unidirectional = Some(value);
    }

    /// Gets the value of IsUnidirectional
    pub fn get_is_unidirectional(&self) -> Option<&bool> {
        self.is_unidirectional.as_ref()
    }

    /// Sets the value of OtherTrafficDescription
    pub fn set_other_traffic_description(&mut self, value: String) {
        self.other_traffic_description = Some(value);
    }

    /// Gets the value of OtherTrafficDescription
    pub fn get_other_traffic_description(&self) -> Option<&String> {
        self.other_traffic_description.as_ref()
    }

    /// Sets the value of TrafficType
    pub fn set_traffic_type(&mut self, value: ActiveConnection_TrafficType) {
        self.traffic_type = Some(value);
    }

    /// Gets the value of TrafficType
    pub fn get_traffic_type(&self) -> Option<&ActiveConnection_TrafficType> {
        self.traffic_type.as_ref()
    }
}

