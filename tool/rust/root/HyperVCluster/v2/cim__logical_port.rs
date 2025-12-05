// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_LogicalPort struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_LogicalPort {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,

/// The maximum bandwidth of the Port in Bits per Second.
    #[serde(rename = "MaxSpeed")]
    pub max_speed: Option<u64>,

/// Describes the type of module, when PortType is set to 1 ("Other").
    #[serde(rename = "OtherPortType")]
    pub other_port_type: Option<String>,

/// PortType is defined to force consistent naming of the 'type' property in subclasses and to guarantee unique enum values for all instances of NetworkPort. When set to 1 ("Other"), related property OtherPortType contains a string description of the type of port. A range of values, DMTF_Reserved, has been defined that allows subclasses to override and define their specific types of ports.
    #[serde(rename = "PortType")]
    pub port_type: Option<LogicalPort_PortType>,

/// The requested bandwidth of the Port in Bits per Second. The actual bandwidth is reported in LogicalPort.Speed.
    #[serde(rename = "RequestedSpeed")]
    pub requested_speed: Option<u64>,

/// The bandwidth of the Port in Bits per Second.
    #[serde(rename = "Speed")]
    pub speed: Option<u64>,

/// In some circumstances, a LogicalPort might be identifiable as a front end or back end port. An example of this situation would be a storage array that might have back end ports to communicate with disk drives and front end ports to communicate with hosts. If there is no restriction on the use of the port, then the value should be set to 'not restricted'.
    #[serde(rename = "UsageRestriction")]
    pub usage_restriction: Option<LogicalPort_UsageRestriction>,
}

impl CIM_LogicalPort {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
            max_speed: None,
            other_port_type: None,
            port_type: None,
            requested_speed: None,
            speed: None,
            usage_restriction: None,
        }
    }


    /// Sets the value of MaxSpeed
    pub fn set_max_speed(&mut self, value: u64) {
        self.max_speed = Some(value);
    }

    /// Gets the value of MaxSpeed
    pub fn get_max_speed(&self) -> Option<&u64> {
        self.max_speed.as_ref()
    }

    /// Sets the value of OtherPortType
    pub fn set_other_port_type(&mut self, value: String) {
        self.other_port_type = Some(value);
    }

    /// Gets the value of OtherPortType
    pub fn get_other_port_type(&self) -> Option<&String> {
        self.other_port_type.as_ref()
    }

    /// Sets the value of PortType
    pub fn set_port_type(&mut self, value: LogicalPort_PortType) {
        self.port_type = Some(value);
    }

    /// Gets the value of PortType
    pub fn get_port_type(&self) -> Option<&LogicalPort_PortType> {
        self.port_type.as_ref()
    }

    /// Sets the value of RequestedSpeed
    pub fn set_requested_speed(&mut self, value: u64) {
        self.requested_speed = Some(value);
    }

    /// Gets the value of RequestedSpeed
    pub fn get_requested_speed(&self) -> Option<&u64> {
        self.requested_speed.as_ref()
    }

    /// Sets the value of Speed
    pub fn set_speed(&mut self, value: u64) {
        self.speed = Some(value);
    }

    /// Gets the value of Speed
    pub fn get_speed(&self) -> Option<&u64> {
        self.speed.as_ref()
    }

    /// Sets the value of UsageRestriction
    pub fn set_usage_restriction(&mut self, value: LogicalPort_UsageRestriction) {
        self.usage_restriction = Some(value);
    }

    /// Gets the value of UsageRestriction
    pub fn get_usage_restriction(&self) -> Option<&LogicalPort_UsageRestriction> {
        self.usage_restriction.as_ref()
    }
}

