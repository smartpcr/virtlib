// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_VLANEndpoint struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_VLANEndpoint {
    #[serde(flatten)]
    pub base: CIM_ProtocolEndpoint,

/// The desired VLAN mode that is requested for use. (Note that the current mode is given by the OperationalEndpointMode property.) The following values are defined: 
/// - Access: Puts the endpoint/switch port into permanent nontrunking mode and negotiates to convert the link into a nontrunk link. The endpoint becomes a nontrunk interface. 
/// - Dynamic Auto: Makes the endpoint able to convert the link to a trunk link. The endpoint becomes a trunk interface if the neighboring interface is set to trunk or desirable mode. 
/// - Dynamic Desirable: Makes the endpoint actively attempt to convert the link to a trunk link. The endpoint becomes a trunk interface if the neighboring interface is set to trunk, desirable, or auto mode. The default switch-port mode for all Ethernet interfaces is 'dynamic desirable.' 
/// - Trunk: Puts the endpoint into permanent trunking mode and negotiates to convert the link into a trunk link. The endpoint becomes a trunk interface even if the neighboring interface is not a trunk interface. 
/// - Dot1Q Tunnel: Configures the interface as a tunnel (nontrunking) endpoint/port to be connected in an asymmetric link with an 802.1Q trunk port. 802.1Q tunneling is used to maintain customer VLAN integrity across a service provider network.
    #[serde(rename = "DesiredEndpointMode")]
    pub desired_endpoint_mode: Option<VLANEndpoint_DesiredEndpointMode>,

/// The type of VLAN encapsulation that is requested for use. (Note that the encapsulation currently in use is given by the OperationalVLANTrunkEncapsulation property.) Note that this property is only applicable when the endpoint is operating in a trunking mode (see the OperationalEndpointMode property for additional details). This property is either 'not applicable' (i.e., the endpoint will never be placed in a trunking mode), a particular type (802.1q or Cisco ISL), or 'negotiate' (i.e., the result of the negotiation between this interface and its neighbor). The value, 'Negotiate' is not allowed if the endpoint does not support negotiation. This capability is hardware and vendor dependent. Refer to the associated VLANEndpointCapabilities.doesTrunkEncapsulationNegotiation property to validate whether a particular endpoint (port) supports encapsulation negotiation.
    #[serde(rename = "DesiredVLANTrunkEncapsulation")]
    pub desired_vlantrunk_encapsulation: Option<VLANEndpoint_DesiredVLANTrunkEncapsulation>,

/// Indicates whether GARP VLAN Registration Protocol (GVRP) is enabled or disabled on the trunk endpoint/port. This property is 'not applicable' unless GVRP is supported by the endpoint. This is indicated in the Capabilities property, VLANEndpointCapabilities.Dot1QTagging. This property is applicable only when the endpoint is operating in trunking mode (determined by examining the SwitchEndpointMode property).
    #[serde(rename = "GVRPStatus")]
    pub gvrpstatus: Option<VLANEndpoint_GVRPStatus>,

/// The configuration mode for the VLAN endpoint. The following values are defined: /n - Unknown: If the endpoint is not VLAN aware. /n - Access: Puts the endpoint into permanent nontrunking mode and negotiates to convert the link into a nontrunk link. The endpoint becomes a nontrunk interface. 
/// - Dynamic Auto: Makes the endpoint able to convert the link to a trunk link. The endpoint becomes a trunk interface if the neighboring interface is set to trunk or desirable mode. 
/// - Dynamic Desirable: Makes the endpoint actively attempt to convert the link to a trunk link. The endpoint becomes a trunk interface if the neighboring interface is set to trunk, desirable, or auto mode. The default switch-port mode for all Ethernet interfaces is 'dynamic desirable.' 
/// - Trunk: Puts the endpoint into permanent trunking mode and negotiates to convert the link into a trunk link. The endpoint becomes a trunk interface even if the neighboring interface is not a trunk interface. 
/// - Dot1Q Tunnel: Configures the interface as a tunnel (nontrunking) endpoint/port to be connected in an asymmetric link with an 802.1Q trunk port. 802.1Q tunneling is used to maintain customer VLAN integrity across a service provider network.
    #[serde(rename = "OperationalEndpointMode")]
    pub operational_endpoint_mode: Option<VLANEndpoint_OperationalEndpointMode>,

/// The type of VLAN encapsulation in use on a trunk endpoint/port. This property is either 'not applicable' (i.e., the endpoint is not operating in trunking mode), a particular type (802.1q or Cisco ISL), 'negotiating' (i.e., the endpoints are negotiating the encapsulation type). Note that this property is only applicable when the endpoint is operating in a trunking mode (see the OperationalEndpointMode property for additional details).
    #[serde(rename = "OperationalVLANTrunkEncapsulation")]
    pub operational_vlantrunk_encapsulation: Option<VLANEndpoint_OperationalVLANTrunkEncapsulation>,

/// A string describing the type of VLAN endpoint model that is supported by this VLANEndpoint, when the value of the mode property is set to 1 (i.e., "Other"). This property should be set to NULL when the mode property is any value other than 1.
    #[serde(rename = "OtherEndpointMode")]
    pub other_endpoint_mode: Option<String>,

/// A string describing the type of VLAN encapsulation that is supported by this VLANEndpoint, when the value of the encapsulation property is set to 1 (i.e., "Other"). This property should be set to NULL when the desired encapsulation property is any value other than 1.
    #[serde(rename = "OtherTrunkEncapsulation")]
    pub other_trunk_encapsulation: Option<String>,
}

impl CIM_VLANEndpoint {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ProtocolEndpoint::new(),
            desired_endpoint_mode: None,
            desired_vlantrunk_encapsulation: None,
            gvrpstatus: None,
            operational_endpoint_mode: None,
            operational_vlantrunk_encapsulation: None,
            other_endpoint_mode: None,
            other_trunk_encapsulation: None,
        }
    }


    /// Sets the value of DesiredEndpointMode
    pub fn set_desired_endpoint_mode(&mut self, value: VLANEndpoint_DesiredEndpointMode) {
        self.desired_endpoint_mode = Some(value);
    }

    /// Gets the value of DesiredEndpointMode
    pub fn get_desired_endpoint_mode(&self) -> Option<&VLANEndpoint_DesiredEndpointMode> {
        self.desired_endpoint_mode.as_ref()
    }

    /// Sets the value of DesiredVLANTrunkEncapsulation
    pub fn set_desired_vlantrunk_encapsulation(&mut self, value: VLANEndpoint_DesiredVLANTrunkEncapsulation) {
        self.desired_vlantrunk_encapsulation = Some(value);
    }

    /// Gets the value of DesiredVLANTrunkEncapsulation
    pub fn get_desired_vlantrunk_encapsulation(&self) -> Option<&VLANEndpoint_DesiredVLANTrunkEncapsulation> {
        self.desired_vlantrunk_encapsulation.as_ref()
    }

    /// Sets the value of GVRPStatus
    pub fn set_gvrpstatus(&mut self, value: VLANEndpoint_GVRPStatus) {
        self.gvrpstatus = Some(value);
    }

    /// Gets the value of GVRPStatus
    pub fn get_gvrpstatus(&self) -> Option<&VLANEndpoint_GVRPStatus> {
        self.gvrpstatus.as_ref()
    }

    /// Sets the value of OperationalEndpointMode
    pub fn set_operational_endpoint_mode(&mut self, value: VLANEndpoint_OperationalEndpointMode) {
        self.operational_endpoint_mode = Some(value);
    }

    /// Gets the value of OperationalEndpointMode
    pub fn get_operational_endpoint_mode(&self) -> Option<&VLANEndpoint_OperationalEndpointMode> {
        self.operational_endpoint_mode.as_ref()
    }

    /// Sets the value of OperationalVLANTrunkEncapsulation
    pub fn set_operational_vlantrunk_encapsulation(&mut self, value: VLANEndpoint_OperationalVLANTrunkEncapsulation) {
        self.operational_vlantrunk_encapsulation = Some(value);
    }

    /// Gets the value of OperationalVLANTrunkEncapsulation
    pub fn get_operational_vlantrunk_encapsulation(&self) -> Option<&VLANEndpoint_OperationalVLANTrunkEncapsulation> {
        self.operational_vlantrunk_encapsulation.as_ref()
    }

    /// Sets the value of OtherEndpointMode
    pub fn set_other_endpoint_mode(&mut self, value: String) {
        self.other_endpoint_mode = Some(value);
    }

    /// Gets the value of OtherEndpointMode
    pub fn get_other_endpoint_mode(&self) -> Option<&String> {
        self.other_endpoint_mode.as_ref()
    }

    /// Sets the value of OtherTrunkEncapsulation
    pub fn set_other_trunk_encapsulation(&mut self, value: String) {
        self.other_trunk_encapsulation = Some(value);
    }

    /// Gets the value of OtherTrunkEncapsulation
    pub fn get_other_trunk_encapsulation(&self) -> Option<&String> {
        self.other_trunk_encapsulation.as_ref()
    }
}

