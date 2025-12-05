// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_WirelessDisplay02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_WirelessDisplay02 {

/// 
    #[serde(rename = "AllowMdnsAdvertisement")]
    pub allow_mdns_advertisement: Option<i32>,

/// 
    #[serde(rename = "AllowMdnsDiscovery")]
    pub allow_mdns_discovery: Option<i32>,

/// 
    #[serde(rename = "AllowMovementDetectionOnInfrastructure")]
    pub allow_movement_detection_on_infrastructure: Option<i32>,

/// 
    #[serde(rename = "AllowPCReceiverToBeTCPServer")]
    pub allow_pcreceiver_to_be_tcpserver: Option<i32>,

/// 
    #[serde(rename = "AllowPCSenderToBeTCPClient")]
    pub allow_pcsender_to_be_tcpclient: Option<i32>,

/// 
    #[serde(rename = "AllowProjectionFromPC")]
    pub allow_projection_from_pc: Option<i32>,

/// 
    #[serde(rename = "AllowProjectionFromPCOverInfrastructure")]
    pub allow_projection_from_pcover_infrastructure: Option<i32>,

/// 
    #[serde(rename = "AllowProjectionToPC")]
    pub allow_projection_to_pc: Option<i32>,

/// 
    #[serde(rename = "AllowProjectionToPCOverInfrastructure")]
    pub allow_projection_to_pcover_infrastructure: Option<i32>,

/// 
    #[serde(rename = "AllowUserInputFromWirelessDisplayReceiver")]
    pub allow_user_input_from_wireless_display_receiver: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "RequirePinForPairing")]
    pub require_pin_for_pairing: Option<i32>,
}

impl MDM_Policy_Config01_WirelessDisplay02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_mdns_advertisement: None,
            allow_mdns_discovery: None,
            allow_movement_detection_on_infrastructure: None,
            allow_pcreceiver_to_be_tcpserver: None,
            allow_pcsender_to_be_tcpclient: None,
            allow_projection_from_pc: None,
            allow_projection_from_pcover_infrastructure: None,
            allow_projection_to_pc: None,
            allow_projection_to_pcover_infrastructure: None,
            allow_user_input_from_wireless_display_receiver: None,
            instance_id: None,
            parent_id: None,
            require_pin_for_pairing: None,
        }
    }


    /// Sets the value of AllowMdnsAdvertisement
    pub fn set_allow_mdns_advertisement(&mut self, value: i32) {
        self.allow_mdns_advertisement = Some(value);
    }

    /// Gets the value of AllowMdnsAdvertisement
    pub fn get_allow_mdns_advertisement(&self) -> Option<&i32> {
        self.allow_mdns_advertisement.as_ref()
    }

    /// Sets the value of AllowMdnsDiscovery
    pub fn set_allow_mdns_discovery(&mut self, value: i32) {
        self.allow_mdns_discovery = Some(value);
    }

    /// Gets the value of AllowMdnsDiscovery
    pub fn get_allow_mdns_discovery(&self) -> Option<&i32> {
        self.allow_mdns_discovery.as_ref()
    }

    /// Sets the value of AllowMovementDetectionOnInfrastructure
    pub fn set_allow_movement_detection_on_infrastructure(&mut self, value: i32) {
        self.allow_movement_detection_on_infrastructure = Some(value);
    }

    /// Gets the value of AllowMovementDetectionOnInfrastructure
    pub fn get_allow_movement_detection_on_infrastructure(&self) -> Option<&i32> {
        self.allow_movement_detection_on_infrastructure.as_ref()
    }

    /// Sets the value of AllowPCReceiverToBeTCPServer
    pub fn set_allow_pcreceiver_to_be_tcpserver(&mut self, value: i32) {
        self.allow_pcreceiver_to_be_tcpserver = Some(value);
    }

    /// Gets the value of AllowPCReceiverToBeTCPServer
    pub fn get_allow_pcreceiver_to_be_tcpserver(&self) -> Option<&i32> {
        self.allow_pcreceiver_to_be_tcpserver.as_ref()
    }

    /// Sets the value of AllowPCSenderToBeTCPClient
    pub fn set_allow_pcsender_to_be_tcpclient(&mut self, value: i32) {
        self.allow_pcsender_to_be_tcpclient = Some(value);
    }

    /// Gets the value of AllowPCSenderToBeTCPClient
    pub fn get_allow_pcsender_to_be_tcpclient(&self) -> Option<&i32> {
        self.allow_pcsender_to_be_tcpclient.as_ref()
    }

    /// Sets the value of AllowProjectionFromPC
    pub fn set_allow_projection_from_pc(&mut self, value: i32) {
        self.allow_projection_from_pc = Some(value);
    }

    /// Gets the value of AllowProjectionFromPC
    pub fn get_allow_projection_from_pc(&self) -> Option<&i32> {
        self.allow_projection_from_pc.as_ref()
    }

    /// Sets the value of AllowProjectionFromPCOverInfrastructure
    pub fn set_allow_projection_from_pcover_infrastructure(&mut self, value: i32) {
        self.allow_projection_from_pcover_infrastructure = Some(value);
    }

    /// Gets the value of AllowProjectionFromPCOverInfrastructure
    pub fn get_allow_projection_from_pcover_infrastructure(&self) -> Option<&i32> {
        self.allow_projection_from_pcover_infrastructure.as_ref()
    }

    /// Sets the value of AllowProjectionToPC
    pub fn set_allow_projection_to_pc(&mut self, value: i32) {
        self.allow_projection_to_pc = Some(value);
    }

    /// Gets the value of AllowProjectionToPC
    pub fn get_allow_projection_to_pc(&self) -> Option<&i32> {
        self.allow_projection_to_pc.as_ref()
    }

    /// Sets the value of AllowProjectionToPCOverInfrastructure
    pub fn set_allow_projection_to_pcover_infrastructure(&mut self, value: i32) {
        self.allow_projection_to_pcover_infrastructure = Some(value);
    }

    /// Gets the value of AllowProjectionToPCOverInfrastructure
    pub fn get_allow_projection_to_pcover_infrastructure(&self) -> Option<&i32> {
        self.allow_projection_to_pcover_infrastructure.as_ref()
    }

    /// Sets the value of AllowUserInputFromWirelessDisplayReceiver
    pub fn set_allow_user_input_from_wireless_display_receiver(&mut self, value: i32) {
        self.allow_user_input_from_wireless_display_receiver = Some(value);
    }

    /// Gets the value of AllowUserInputFromWirelessDisplayReceiver
    pub fn get_allow_user_input_from_wireless_display_receiver(&self) -> Option<&i32> {
        self.allow_user_input_from_wireless_display_receiver.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of RequirePinForPairing
    pub fn set_require_pin_for_pairing(&mut self, value: i32) {
        self.require_pin_for_pairing = Some(value);
    }

    /// Gets the value of RequirePinForPairing
    pub fn get_require_pin_for_pairing(&self) -> Option<&i32> {
        self.require_pin_for_pairing.as_ref()
    }
}

