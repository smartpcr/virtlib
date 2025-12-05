// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_NvspNicDropReasonsStats_HyperVVirtualNetworkAdapterDropReasons struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_NvspNicDropReasonsStats_HyperVVirtualNetworkAdapterDropReasons {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "IncomingBridgeReserved")]
    pub incoming_bridge_reserved: Option<u64>,

/// 
    #[serde(rename = "IncomingBusy")]
    pub incoming_busy: Option<u64>,

/// 
    #[serde(rename = "IncomingDhcpGuard")]
    pub incoming_dhcp_guard: Option<u64>,

/// 
    #[serde(rename = "IncomingDisconnected")]
    pub incoming_disconnected: Option<u64>,

/// 
    #[serde(rename = "IncomingFailedDestinationListUpdate")]
    pub incoming_failed_destination_list_update: Option<u64>,

/// 
    #[serde(rename = "IncomingFailedPacketFilter")]
    pub incoming_failed_packet_filter: Option<u64>,

/// 
    #[serde(rename = "IncomingFailedPvlanSetting")]
    pub incoming_failed_pvlan_setting: Option<u64>,

/// 
    #[serde(rename = "IncomingFailedSecurityPolicy")]
    pub incoming_failed_security_policy: Option<u64>,

/// 
    #[serde(rename = "IncomingFiltered")]
    pub incoming_filtered: Option<u64>,

/// 
    #[serde(rename = "IncomingFilteredIsolationUntagged")]
    pub incoming_filtered_isolation_untagged: Option<u64>,

/// 
    #[serde(rename = "IncomingFilteredVLAN")]
    pub incoming_filtered_vlan: Option<u64>,

/// 
    #[serde(rename = "IncomingInjectedIcmp")]
    pub incoming_injected_icmp: Option<u64>,

/// 
    #[serde(rename = "IncomingInvalidConfig")]
    pub incoming_invalid_config: Option<u64>,

/// 
    #[serde(rename = "IncomingInvalidData")]
    pub incoming_invalid_data: Option<u64>,

/// 
    #[serde(rename = "IncomingInvalidDestMac")]
    pub incoming_invalid_dest_mac: Option<u64>,

/// 
    #[serde(rename = "IncomingInvalidFirstNBTooSmall")]
    pub incoming_invalid_first_nbtoo_small: Option<u64>,

/// 
    #[serde(rename = "IncomingInvalidPacket")]
    pub incoming_invalid_packet: Option<u64>,

/// 
    #[serde(rename = "IncomingInvalidPDQueue")]
    pub incoming_invalid_pdqueue: Option<u64>,

/// 
    #[serde(rename = "IncomingInvalidSourceMac")]
    pub incoming_invalid_source_mac: Option<u64>,

/// 
    #[serde(rename = "IncomingInvalidVlanFormat")]
    pub incoming_invalid_vlan_format: Option<u64>,

/// 
    #[serde(rename = "IncomingIpsec")]
    pub incoming_ipsec: Option<u64>,

/// 
    #[serde(rename = "IncomingLowPowerPacketFilter")]
    pub incoming_low_power_packet_filter: Option<u64>,

/// 
    #[serde(rename = "IncomingMacSpoofing")]
    pub incoming_mac_spoofing: Option<u64>,

/// 
    #[serde(rename = "IncomingMTUMismatch")]
    pub incoming_mtumismatch: Option<u64>,

/// 
    #[serde(rename = "IncomingNativeFwdingReq")]
    pub incoming_native_fwding_req: Option<u64>,

/// 
    #[serde(rename = "IncomingNicDisabled")]
    pub incoming_nic_disabled: Option<u64>,

/// 
    #[serde(rename = "IncomingNotAccepted")]
    pub incoming_not_accepted: Option<u64>,

/// 
    #[serde(rename = "IncomingNotReady")]
    pub incoming_not_ready: Option<u64>,

/// 
    #[serde(rename = "IncomingQos")]
    pub incoming_qos: Option<u64>,

/// 
    #[serde(rename = "IncomingRequiredExtensionMissing")]
    pub incoming_required_extension_missing: Option<u64>,

/// 
    #[serde(rename = "IncomingResources")]
    pub incoming_resources: Option<u64>,

/// 
    #[serde(rename = "IncomingRouterGuard")]
    pub incoming_router_guard: Option<u64>,

/// 
    #[serde(rename = "IncomingStormLimit")]
    pub incoming_storm_limit: Option<u64>,

/// 
    #[serde(rename = "IncomingSwitchDataFlowDisabled")]
    pub incoming_switch_data_flow_disabled: Option<u64>,

/// 
    #[serde(rename = "IncomingUnauthorizedMAC")]
    pub incoming_unauthorized_mac: Option<u64>,

/// 
    #[serde(rename = "IncomingUnauthorizedVLAN")]
    pub incoming_unauthorized_vlan: Option<u64>,

/// 
    #[serde(rename = "IncomingUnknown")]
    pub incoming_unknown: Option<u64>,

/// 
    #[serde(rename = "IncomingVirtualSubnetId")]
    pub incoming_virtual_subnet_id: Option<u64>,

/// 
    #[serde(rename = "OutgoingBridgeReserved")]
    pub outgoing_bridge_reserved: Option<u64>,

/// 
    #[serde(rename = "OutgoingBusy")]
    pub outgoing_busy: Option<u64>,

/// 
    #[serde(rename = "OutgoingDhcpGuard")]
    pub outgoing_dhcp_guard: Option<u64>,

/// 
    #[serde(rename = "OutgoingDisconnected")]
    pub outgoing_disconnected: Option<u64>,

/// 
    #[serde(rename = "OutgoingFailedDestinationListUpdate")]
    pub outgoing_failed_destination_list_update: Option<u64>,

/// 
    #[serde(rename = "OutgoingFailedPacketFilter")]
    pub outgoing_failed_packet_filter: Option<u64>,

/// 
    #[serde(rename = "OutgoingFailedPvlanSetting")]
    pub outgoing_failed_pvlan_setting: Option<u64>,

/// 
    #[serde(rename = "OutgoingFailedSecurityPolicy")]
    pub outgoing_failed_security_policy: Option<u64>,

/// 
    #[serde(rename = "OutgoingFiltered")]
    pub outgoing_filtered: Option<u64>,

/// 
    #[serde(rename = "OutgoingFilteredIsolationUntagged")]
    pub outgoing_filtered_isolation_untagged: Option<u64>,

/// 
    #[serde(rename = "OutgoingFilteredVLAN")]
    pub outgoing_filtered_vlan: Option<u64>,

/// 
    #[serde(rename = "OutgoingInjectedIcmp")]
    pub outgoing_injected_icmp: Option<u64>,

/// 
    #[serde(rename = "OutgoingInvalidConfig")]
    pub outgoing_invalid_config: Option<u64>,

/// 
    #[serde(rename = "OutgoingInvalidData")]
    pub outgoing_invalid_data: Option<u64>,

/// 
    #[serde(rename = "OutgoingInvalidDestMac")]
    pub outgoing_invalid_dest_mac: Option<u64>,

/// 
    #[serde(rename = "OutgoingInvalidFirstNBTooSmall")]
    pub outgoing_invalid_first_nbtoo_small: Option<u64>,

/// 
    #[serde(rename = "OutgoingInvalidPacket")]
    pub outgoing_invalid_packet: Option<u64>,

/// 
    #[serde(rename = "OutgoingInvalidPDQueue")]
    pub outgoing_invalid_pdqueue: Option<u64>,

/// 
    #[serde(rename = "OutgoingInvalidSourceMac")]
    pub outgoing_invalid_source_mac: Option<u64>,

/// 
    #[serde(rename = "OutgoingInvalidVlanFormat")]
    pub outgoing_invalid_vlan_format: Option<u64>,

/// 
    #[serde(rename = "OutgoingIpsec")]
    pub outgoing_ipsec: Option<u64>,

/// 
    #[serde(rename = "OutgoingLowPowerPacketFilter")]
    pub outgoing_low_power_packet_filter: Option<u64>,

/// 
    #[serde(rename = "OutgoingMacSpoofing")]
    pub outgoing_mac_spoofing: Option<u64>,

/// 
    #[serde(rename = "OutgoingMTUMismatch")]
    pub outgoing_mtumismatch: Option<u64>,

/// 
    #[serde(rename = "OutgoingNativeFwdingReq")]
    pub outgoing_native_fwding_req: Option<u64>,

/// 
    #[serde(rename = "OutgoingNicDisabled")]
    pub outgoing_nic_disabled: Option<u64>,

/// 
    #[serde(rename = "OutgoingNotAccepted")]
    pub outgoing_not_accepted: Option<u64>,

/// 
    #[serde(rename = "OutgoingNotReady")]
    pub outgoing_not_ready: Option<u64>,

/// 
    #[serde(rename = "OutgoingQos")]
    pub outgoing_qos: Option<u64>,

/// 
    #[serde(rename = "OutgoingRequiredExtensionMissing")]
    pub outgoing_required_extension_missing: Option<u64>,

/// 
    #[serde(rename = "OutgoingResources")]
    pub outgoing_resources: Option<u64>,

/// 
    #[serde(rename = "OutgoingRouterGuard")]
    pub outgoing_router_guard: Option<u64>,

/// 
    #[serde(rename = "OutgoingStormLimit")]
    pub outgoing_storm_limit: Option<u64>,

/// 
    #[serde(rename = "OutgoingSwitchDataFlowDisabled")]
    pub outgoing_switch_data_flow_disabled: Option<u64>,

/// 
    #[serde(rename = "OutgoingUnauthorizedMAC")]
    pub outgoing_unauthorized_mac: Option<u64>,

/// 
    #[serde(rename = "OutgoingUnauthorizedVLAN")]
    pub outgoing_unauthorized_vlan: Option<u64>,

/// 
    #[serde(rename = "OutgoingUnknown")]
    pub outgoing_unknown: Option<u64>,

/// 
    #[serde(rename = "OutgoingVirtualSubnetId")]
    pub outgoing_virtual_subnet_id: Option<u64>,
}

impl Win32_PerfFormattedData_NvspNicDropReasonsStats_HyperVVirtualNetworkAdapterDropReasons {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            incoming_bridge_reserved: None,
            incoming_busy: None,
            incoming_dhcp_guard: None,
            incoming_disconnected: None,
            incoming_failed_destination_list_update: None,
            incoming_failed_packet_filter: None,
            incoming_failed_pvlan_setting: None,
            incoming_failed_security_policy: None,
            incoming_filtered: None,
            incoming_filtered_isolation_untagged: None,
            incoming_filtered_vlan: None,
            incoming_injected_icmp: None,
            incoming_invalid_config: None,
            incoming_invalid_data: None,
            incoming_invalid_dest_mac: None,
            incoming_invalid_first_nbtoo_small: None,
            incoming_invalid_packet: None,
            incoming_invalid_pdqueue: None,
            incoming_invalid_source_mac: None,
            incoming_invalid_vlan_format: None,
            incoming_ipsec: None,
            incoming_low_power_packet_filter: None,
            incoming_mac_spoofing: None,
            incoming_mtumismatch: None,
            incoming_native_fwding_req: None,
            incoming_nic_disabled: None,
            incoming_not_accepted: None,
            incoming_not_ready: None,
            incoming_qos: None,
            incoming_required_extension_missing: None,
            incoming_resources: None,
            incoming_router_guard: None,
            incoming_storm_limit: None,
            incoming_switch_data_flow_disabled: None,
            incoming_unauthorized_mac: None,
            incoming_unauthorized_vlan: None,
            incoming_unknown: None,
            incoming_virtual_subnet_id: None,
            outgoing_bridge_reserved: None,
            outgoing_busy: None,
            outgoing_dhcp_guard: None,
            outgoing_disconnected: None,
            outgoing_failed_destination_list_update: None,
            outgoing_failed_packet_filter: None,
            outgoing_failed_pvlan_setting: None,
            outgoing_failed_security_policy: None,
            outgoing_filtered: None,
            outgoing_filtered_isolation_untagged: None,
            outgoing_filtered_vlan: None,
            outgoing_injected_icmp: None,
            outgoing_invalid_config: None,
            outgoing_invalid_data: None,
            outgoing_invalid_dest_mac: None,
            outgoing_invalid_first_nbtoo_small: None,
            outgoing_invalid_packet: None,
            outgoing_invalid_pdqueue: None,
            outgoing_invalid_source_mac: None,
            outgoing_invalid_vlan_format: None,
            outgoing_ipsec: None,
            outgoing_low_power_packet_filter: None,
            outgoing_mac_spoofing: None,
            outgoing_mtumismatch: None,
            outgoing_native_fwding_req: None,
            outgoing_nic_disabled: None,
            outgoing_not_accepted: None,
            outgoing_not_ready: None,
            outgoing_qos: None,
            outgoing_required_extension_missing: None,
            outgoing_resources: None,
            outgoing_router_guard: None,
            outgoing_storm_limit: None,
            outgoing_switch_data_flow_disabled: None,
            outgoing_unauthorized_mac: None,
            outgoing_unauthorized_vlan: None,
            outgoing_unknown: None,
            outgoing_virtual_subnet_id: None,
        }
    }


    /// Sets the value of IncomingBridgeReserved
    pub fn set_incoming_bridge_reserved(&mut self, value: u64) {
        self.incoming_bridge_reserved = Some(value);
    }

    /// Gets the value of IncomingBridgeReserved
    pub fn get_incoming_bridge_reserved(&self) -> Option<&u64> {
        self.incoming_bridge_reserved.as_ref()
    }

    /// Sets the value of IncomingBusy
    pub fn set_incoming_busy(&mut self, value: u64) {
        self.incoming_busy = Some(value);
    }

    /// Gets the value of IncomingBusy
    pub fn get_incoming_busy(&self) -> Option<&u64> {
        self.incoming_busy.as_ref()
    }

    /// Sets the value of IncomingDhcpGuard
    pub fn set_incoming_dhcp_guard(&mut self, value: u64) {
        self.incoming_dhcp_guard = Some(value);
    }

    /// Gets the value of IncomingDhcpGuard
    pub fn get_incoming_dhcp_guard(&self) -> Option<&u64> {
        self.incoming_dhcp_guard.as_ref()
    }

    /// Sets the value of IncomingDisconnected
    pub fn set_incoming_disconnected(&mut self, value: u64) {
        self.incoming_disconnected = Some(value);
    }

    /// Gets the value of IncomingDisconnected
    pub fn get_incoming_disconnected(&self) -> Option<&u64> {
        self.incoming_disconnected.as_ref()
    }

    /// Sets the value of IncomingFailedDestinationListUpdate
    pub fn set_incoming_failed_destination_list_update(&mut self, value: u64) {
        self.incoming_failed_destination_list_update = Some(value);
    }

    /// Gets the value of IncomingFailedDestinationListUpdate
    pub fn get_incoming_failed_destination_list_update(&self) -> Option<&u64> {
        self.incoming_failed_destination_list_update.as_ref()
    }

    /// Sets the value of IncomingFailedPacketFilter
    pub fn set_incoming_failed_packet_filter(&mut self, value: u64) {
        self.incoming_failed_packet_filter = Some(value);
    }

    /// Gets the value of IncomingFailedPacketFilter
    pub fn get_incoming_failed_packet_filter(&self) -> Option<&u64> {
        self.incoming_failed_packet_filter.as_ref()
    }

    /// Sets the value of IncomingFailedPvlanSetting
    pub fn set_incoming_failed_pvlan_setting(&mut self, value: u64) {
        self.incoming_failed_pvlan_setting = Some(value);
    }

    /// Gets the value of IncomingFailedPvlanSetting
    pub fn get_incoming_failed_pvlan_setting(&self) -> Option<&u64> {
        self.incoming_failed_pvlan_setting.as_ref()
    }

    /// Sets the value of IncomingFailedSecurityPolicy
    pub fn set_incoming_failed_security_policy(&mut self, value: u64) {
        self.incoming_failed_security_policy = Some(value);
    }

    /// Gets the value of IncomingFailedSecurityPolicy
    pub fn get_incoming_failed_security_policy(&self) -> Option<&u64> {
        self.incoming_failed_security_policy.as_ref()
    }

    /// Sets the value of IncomingFiltered
    pub fn set_incoming_filtered(&mut self, value: u64) {
        self.incoming_filtered = Some(value);
    }

    /// Gets the value of IncomingFiltered
    pub fn get_incoming_filtered(&self) -> Option<&u64> {
        self.incoming_filtered.as_ref()
    }

    /// Sets the value of IncomingFilteredIsolationUntagged
    pub fn set_incoming_filtered_isolation_untagged(&mut self, value: u64) {
        self.incoming_filtered_isolation_untagged = Some(value);
    }

    /// Gets the value of IncomingFilteredIsolationUntagged
    pub fn get_incoming_filtered_isolation_untagged(&self) -> Option<&u64> {
        self.incoming_filtered_isolation_untagged.as_ref()
    }

    /// Sets the value of IncomingFilteredVLAN
    pub fn set_incoming_filtered_vlan(&mut self, value: u64) {
        self.incoming_filtered_vlan = Some(value);
    }

    /// Gets the value of IncomingFilteredVLAN
    pub fn get_incoming_filtered_vlan(&self) -> Option<&u64> {
        self.incoming_filtered_vlan.as_ref()
    }

    /// Sets the value of IncomingInjectedIcmp
    pub fn set_incoming_injected_icmp(&mut self, value: u64) {
        self.incoming_injected_icmp = Some(value);
    }

    /// Gets the value of IncomingInjectedIcmp
    pub fn get_incoming_injected_icmp(&self) -> Option<&u64> {
        self.incoming_injected_icmp.as_ref()
    }

    /// Sets the value of IncomingInvalidConfig
    pub fn set_incoming_invalid_config(&mut self, value: u64) {
        self.incoming_invalid_config = Some(value);
    }

    /// Gets the value of IncomingInvalidConfig
    pub fn get_incoming_invalid_config(&self) -> Option<&u64> {
        self.incoming_invalid_config.as_ref()
    }

    /// Sets the value of IncomingInvalidData
    pub fn set_incoming_invalid_data(&mut self, value: u64) {
        self.incoming_invalid_data = Some(value);
    }

    /// Gets the value of IncomingInvalidData
    pub fn get_incoming_invalid_data(&self) -> Option<&u64> {
        self.incoming_invalid_data.as_ref()
    }

    /// Sets the value of IncomingInvalidDestMac
    pub fn set_incoming_invalid_dest_mac(&mut self, value: u64) {
        self.incoming_invalid_dest_mac = Some(value);
    }

    /// Gets the value of IncomingInvalidDestMac
    pub fn get_incoming_invalid_dest_mac(&self) -> Option<&u64> {
        self.incoming_invalid_dest_mac.as_ref()
    }

    /// Sets the value of IncomingInvalidFirstNBTooSmall
    pub fn set_incoming_invalid_first_nbtoo_small(&mut self, value: u64) {
        self.incoming_invalid_first_nbtoo_small = Some(value);
    }

    /// Gets the value of IncomingInvalidFirstNBTooSmall
    pub fn get_incoming_invalid_first_nbtoo_small(&self) -> Option<&u64> {
        self.incoming_invalid_first_nbtoo_small.as_ref()
    }

    /// Sets the value of IncomingInvalidPacket
    pub fn set_incoming_invalid_packet(&mut self, value: u64) {
        self.incoming_invalid_packet = Some(value);
    }

    /// Gets the value of IncomingInvalidPacket
    pub fn get_incoming_invalid_packet(&self) -> Option<&u64> {
        self.incoming_invalid_packet.as_ref()
    }

    /// Sets the value of IncomingInvalidPDQueue
    pub fn set_incoming_invalid_pdqueue(&mut self, value: u64) {
        self.incoming_invalid_pdqueue = Some(value);
    }

    /// Gets the value of IncomingInvalidPDQueue
    pub fn get_incoming_invalid_pdqueue(&self) -> Option<&u64> {
        self.incoming_invalid_pdqueue.as_ref()
    }

    /// Sets the value of IncomingInvalidSourceMac
    pub fn set_incoming_invalid_source_mac(&mut self, value: u64) {
        self.incoming_invalid_source_mac = Some(value);
    }

    /// Gets the value of IncomingInvalidSourceMac
    pub fn get_incoming_invalid_source_mac(&self) -> Option<&u64> {
        self.incoming_invalid_source_mac.as_ref()
    }

    /// Sets the value of IncomingInvalidVlanFormat
    pub fn set_incoming_invalid_vlan_format(&mut self, value: u64) {
        self.incoming_invalid_vlan_format = Some(value);
    }

    /// Gets the value of IncomingInvalidVlanFormat
    pub fn get_incoming_invalid_vlan_format(&self) -> Option<&u64> {
        self.incoming_invalid_vlan_format.as_ref()
    }

    /// Sets the value of IncomingIpsec
    pub fn set_incoming_ipsec(&mut self, value: u64) {
        self.incoming_ipsec = Some(value);
    }

    /// Gets the value of IncomingIpsec
    pub fn get_incoming_ipsec(&self) -> Option<&u64> {
        self.incoming_ipsec.as_ref()
    }

    /// Sets the value of IncomingLowPowerPacketFilter
    pub fn set_incoming_low_power_packet_filter(&mut self, value: u64) {
        self.incoming_low_power_packet_filter = Some(value);
    }

    /// Gets the value of IncomingLowPowerPacketFilter
    pub fn get_incoming_low_power_packet_filter(&self) -> Option<&u64> {
        self.incoming_low_power_packet_filter.as_ref()
    }

    /// Sets the value of IncomingMacSpoofing
    pub fn set_incoming_mac_spoofing(&mut self, value: u64) {
        self.incoming_mac_spoofing = Some(value);
    }

    /// Gets the value of IncomingMacSpoofing
    pub fn get_incoming_mac_spoofing(&self) -> Option<&u64> {
        self.incoming_mac_spoofing.as_ref()
    }

    /// Sets the value of IncomingMTUMismatch
    pub fn set_incoming_mtumismatch(&mut self, value: u64) {
        self.incoming_mtumismatch = Some(value);
    }

    /// Gets the value of IncomingMTUMismatch
    pub fn get_incoming_mtumismatch(&self) -> Option<&u64> {
        self.incoming_mtumismatch.as_ref()
    }

    /// Sets the value of IncomingNativeFwdingReq
    pub fn set_incoming_native_fwding_req(&mut self, value: u64) {
        self.incoming_native_fwding_req = Some(value);
    }

    /// Gets the value of IncomingNativeFwdingReq
    pub fn get_incoming_native_fwding_req(&self) -> Option<&u64> {
        self.incoming_native_fwding_req.as_ref()
    }

    /// Sets the value of IncomingNicDisabled
    pub fn set_incoming_nic_disabled(&mut self, value: u64) {
        self.incoming_nic_disabled = Some(value);
    }

    /// Gets the value of IncomingNicDisabled
    pub fn get_incoming_nic_disabled(&self) -> Option<&u64> {
        self.incoming_nic_disabled.as_ref()
    }

    /// Sets the value of IncomingNotAccepted
    pub fn set_incoming_not_accepted(&mut self, value: u64) {
        self.incoming_not_accepted = Some(value);
    }

    /// Gets the value of IncomingNotAccepted
    pub fn get_incoming_not_accepted(&self) -> Option<&u64> {
        self.incoming_not_accepted.as_ref()
    }

    /// Sets the value of IncomingNotReady
    pub fn set_incoming_not_ready(&mut self, value: u64) {
        self.incoming_not_ready = Some(value);
    }

    /// Gets the value of IncomingNotReady
    pub fn get_incoming_not_ready(&self) -> Option<&u64> {
        self.incoming_not_ready.as_ref()
    }

    /// Sets the value of IncomingQos
    pub fn set_incoming_qos(&mut self, value: u64) {
        self.incoming_qos = Some(value);
    }

    /// Gets the value of IncomingQos
    pub fn get_incoming_qos(&self) -> Option<&u64> {
        self.incoming_qos.as_ref()
    }

    /// Sets the value of IncomingRequiredExtensionMissing
    pub fn set_incoming_required_extension_missing(&mut self, value: u64) {
        self.incoming_required_extension_missing = Some(value);
    }

    /// Gets the value of IncomingRequiredExtensionMissing
    pub fn get_incoming_required_extension_missing(&self) -> Option<&u64> {
        self.incoming_required_extension_missing.as_ref()
    }

    /// Sets the value of IncomingResources
    pub fn set_incoming_resources(&mut self, value: u64) {
        self.incoming_resources = Some(value);
    }

    /// Gets the value of IncomingResources
    pub fn get_incoming_resources(&self) -> Option<&u64> {
        self.incoming_resources.as_ref()
    }

    /// Sets the value of IncomingRouterGuard
    pub fn set_incoming_router_guard(&mut self, value: u64) {
        self.incoming_router_guard = Some(value);
    }

    /// Gets the value of IncomingRouterGuard
    pub fn get_incoming_router_guard(&self) -> Option<&u64> {
        self.incoming_router_guard.as_ref()
    }

    /// Sets the value of IncomingStormLimit
    pub fn set_incoming_storm_limit(&mut self, value: u64) {
        self.incoming_storm_limit = Some(value);
    }

    /// Gets the value of IncomingStormLimit
    pub fn get_incoming_storm_limit(&self) -> Option<&u64> {
        self.incoming_storm_limit.as_ref()
    }

    /// Sets the value of IncomingSwitchDataFlowDisabled
    pub fn set_incoming_switch_data_flow_disabled(&mut self, value: u64) {
        self.incoming_switch_data_flow_disabled = Some(value);
    }

    /// Gets the value of IncomingSwitchDataFlowDisabled
    pub fn get_incoming_switch_data_flow_disabled(&self) -> Option<&u64> {
        self.incoming_switch_data_flow_disabled.as_ref()
    }

    /// Sets the value of IncomingUnauthorizedMAC
    pub fn set_incoming_unauthorized_mac(&mut self, value: u64) {
        self.incoming_unauthorized_mac = Some(value);
    }

    /// Gets the value of IncomingUnauthorizedMAC
    pub fn get_incoming_unauthorized_mac(&self) -> Option<&u64> {
        self.incoming_unauthorized_mac.as_ref()
    }

    /// Sets the value of IncomingUnauthorizedVLAN
    pub fn set_incoming_unauthorized_vlan(&mut self, value: u64) {
        self.incoming_unauthorized_vlan = Some(value);
    }

    /// Gets the value of IncomingUnauthorizedVLAN
    pub fn get_incoming_unauthorized_vlan(&self) -> Option<&u64> {
        self.incoming_unauthorized_vlan.as_ref()
    }

    /// Sets the value of IncomingUnknown
    pub fn set_incoming_unknown(&mut self, value: u64) {
        self.incoming_unknown = Some(value);
    }

    /// Gets the value of IncomingUnknown
    pub fn get_incoming_unknown(&self) -> Option<&u64> {
        self.incoming_unknown.as_ref()
    }

    /// Sets the value of IncomingVirtualSubnetId
    pub fn set_incoming_virtual_subnet_id(&mut self, value: u64) {
        self.incoming_virtual_subnet_id = Some(value);
    }

    /// Gets the value of IncomingVirtualSubnetId
    pub fn get_incoming_virtual_subnet_id(&self) -> Option<&u64> {
        self.incoming_virtual_subnet_id.as_ref()
    }

    /// Sets the value of OutgoingBridgeReserved
    pub fn set_outgoing_bridge_reserved(&mut self, value: u64) {
        self.outgoing_bridge_reserved = Some(value);
    }

    /// Gets the value of OutgoingBridgeReserved
    pub fn get_outgoing_bridge_reserved(&self) -> Option<&u64> {
        self.outgoing_bridge_reserved.as_ref()
    }

    /// Sets the value of OutgoingBusy
    pub fn set_outgoing_busy(&mut self, value: u64) {
        self.outgoing_busy = Some(value);
    }

    /// Gets the value of OutgoingBusy
    pub fn get_outgoing_busy(&self) -> Option<&u64> {
        self.outgoing_busy.as_ref()
    }

    /// Sets the value of OutgoingDhcpGuard
    pub fn set_outgoing_dhcp_guard(&mut self, value: u64) {
        self.outgoing_dhcp_guard = Some(value);
    }

    /// Gets the value of OutgoingDhcpGuard
    pub fn get_outgoing_dhcp_guard(&self) -> Option<&u64> {
        self.outgoing_dhcp_guard.as_ref()
    }

    /// Sets the value of OutgoingDisconnected
    pub fn set_outgoing_disconnected(&mut self, value: u64) {
        self.outgoing_disconnected = Some(value);
    }

    /// Gets the value of OutgoingDisconnected
    pub fn get_outgoing_disconnected(&self) -> Option<&u64> {
        self.outgoing_disconnected.as_ref()
    }

    /// Sets the value of OutgoingFailedDestinationListUpdate
    pub fn set_outgoing_failed_destination_list_update(&mut self, value: u64) {
        self.outgoing_failed_destination_list_update = Some(value);
    }

    /// Gets the value of OutgoingFailedDestinationListUpdate
    pub fn get_outgoing_failed_destination_list_update(&self) -> Option<&u64> {
        self.outgoing_failed_destination_list_update.as_ref()
    }

    /// Sets the value of OutgoingFailedPacketFilter
    pub fn set_outgoing_failed_packet_filter(&mut self, value: u64) {
        self.outgoing_failed_packet_filter = Some(value);
    }

    /// Gets the value of OutgoingFailedPacketFilter
    pub fn get_outgoing_failed_packet_filter(&self) -> Option<&u64> {
        self.outgoing_failed_packet_filter.as_ref()
    }

    /// Sets the value of OutgoingFailedPvlanSetting
    pub fn set_outgoing_failed_pvlan_setting(&mut self, value: u64) {
        self.outgoing_failed_pvlan_setting = Some(value);
    }

    /// Gets the value of OutgoingFailedPvlanSetting
    pub fn get_outgoing_failed_pvlan_setting(&self) -> Option<&u64> {
        self.outgoing_failed_pvlan_setting.as_ref()
    }

    /// Sets the value of OutgoingFailedSecurityPolicy
    pub fn set_outgoing_failed_security_policy(&mut self, value: u64) {
        self.outgoing_failed_security_policy = Some(value);
    }

    /// Gets the value of OutgoingFailedSecurityPolicy
    pub fn get_outgoing_failed_security_policy(&self) -> Option<&u64> {
        self.outgoing_failed_security_policy.as_ref()
    }

    /// Sets the value of OutgoingFiltered
    pub fn set_outgoing_filtered(&mut self, value: u64) {
        self.outgoing_filtered = Some(value);
    }

    /// Gets the value of OutgoingFiltered
    pub fn get_outgoing_filtered(&self) -> Option<&u64> {
        self.outgoing_filtered.as_ref()
    }

    /// Sets the value of OutgoingFilteredIsolationUntagged
    pub fn set_outgoing_filtered_isolation_untagged(&mut self, value: u64) {
        self.outgoing_filtered_isolation_untagged = Some(value);
    }

    /// Gets the value of OutgoingFilteredIsolationUntagged
    pub fn get_outgoing_filtered_isolation_untagged(&self) -> Option<&u64> {
        self.outgoing_filtered_isolation_untagged.as_ref()
    }

    /// Sets the value of OutgoingFilteredVLAN
    pub fn set_outgoing_filtered_vlan(&mut self, value: u64) {
        self.outgoing_filtered_vlan = Some(value);
    }

    /// Gets the value of OutgoingFilteredVLAN
    pub fn get_outgoing_filtered_vlan(&self) -> Option<&u64> {
        self.outgoing_filtered_vlan.as_ref()
    }

    /// Sets the value of OutgoingInjectedIcmp
    pub fn set_outgoing_injected_icmp(&mut self, value: u64) {
        self.outgoing_injected_icmp = Some(value);
    }

    /// Gets the value of OutgoingInjectedIcmp
    pub fn get_outgoing_injected_icmp(&self) -> Option<&u64> {
        self.outgoing_injected_icmp.as_ref()
    }

    /// Sets the value of OutgoingInvalidConfig
    pub fn set_outgoing_invalid_config(&mut self, value: u64) {
        self.outgoing_invalid_config = Some(value);
    }

    /// Gets the value of OutgoingInvalidConfig
    pub fn get_outgoing_invalid_config(&self) -> Option<&u64> {
        self.outgoing_invalid_config.as_ref()
    }

    /// Sets the value of OutgoingInvalidData
    pub fn set_outgoing_invalid_data(&mut self, value: u64) {
        self.outgoing_invalid_data = Some(value);
    }

    /// Gets the value of OutgoingInvalidData
    pub fn get_outgoing_invalid_data(&self) -> Option<&u64> {
        self.outgoing_invalid_data.as_ref()
    }

    /// Sets the value of OutgoingInvalidDestMac
    pub fn set_outgoing_invalid_dest_mac(&mut self, value: u64) {
        self.outgoing_invalid_dest_mac = Some(value);
    }

    /// Gets the value of OutgoingInvalidDestMac
    pub fn get_outgoing_invalid_dest_mac(&self) -> Option<&u64> {
        self.outgoing_invalid_dest_mac.as_ref()
    }

    /// Sets the value of OutgoingInvalidFirstNBTooSmall
    pub fn set_outgoing_invalid_first_nbtoo_small(&mut self, value: u64) {
        self.outgoing_invalid_first_nbtoo_small = Some(value);
    }

    /// Gets the value of OutgoingInvalidFirstNBTooSmall
    pub fn get_outgoing_invalid_first_nbtoo_small(&self) -> Option<&u64> {
        self.outgoing_invalid_first_nbtoo_small.as_ref()
    }

    /// Sets the value of OutgoingInvalidPacket
    pub fn set_outgoing_invalid_packet(&mut self, value: u64) {
        self.outgoing_invalid_packet = Some(value);
    }

    /// Gets the value of OutgoingInvalidPacket
    pub fn get_outgoing_invalid_packet(&self) -> Option<&u64> {
        self.outgoing_invalid_packet.as_ref()
    }

    /// Sets the value of OutgoingInvalidPDQueue
    pub fn set_outgoing_invalid_pdqueue(&mut self, value: u64) {
        self.outgoing_invalid_pdqueue = Some(value);
    }

    /// Gets the value of OutgoingInvalidPDQueue
    pub fn get_outgoing_invalid_pdqueue(&self) -> Option<&u64> {
        self.outgoing_invalid_pdqueue.as_ref()
    }

    /// Sets the value of OutgoingInvalidSourceMac
    pub fn set_outgoing_invalid_source_mac(&mut self, value: u64) {
        self.outgoing_invalid_source_mac = Some(value);
    }

    /// Gets the value of OutgoingInvalidSourceMac
    pub fn get_outgoing_invalid_source_mac(&self) -> Option<&u64> {
        self.outgoing_invalid_source_mac.as_ref()
    }

    /// Sets the value of OutgoingInvalidVlanFormat
    pub fn set_outgoing_invalid_vlan_format(&mut self, value: u64) {
        self.outgoing_invalid_vlan_format = Some(value);
    }

    /// Gets the value of OutgoingInvalidVlanFormat
    pub fn get_outgoing_invalid_vlan_format(&self) -> Option<&u64> {
        self.outgoing_invalid_vlan_format.as_ref()
    }

    /// Sets the value of OutgoingIpsec
    pub fn set_outgoing_ipsec(&mut self, value: u64) {
        self.outgoing_ipsec = Some(value);
    }

    /// Gets the value of OutgoingIpsec
    pub fn get_outgoing_ipsec(&self) -> Option<&u64> {
        self.outgoing_ipsec.as_ref()
    }

    /// Sets the value of OutgoingLowPowerPacketFilter
    pub fn set_outgoing_low_power_packet_filter(&mut self, value: u64) {
        self.outgoing_low_power_packet_filter = Some(value);
    }

    /// Gets the value of OutgoingLowPowerPacketFilter
    pub fn get_outgoing_low_power_packet_filter(&self) -> Option<&u64> {
        self.outgoing_low_power_packet_filter.as_ref()
    }

    /// Sets the value of OutgoingMacSpoofing
    pub fn set_outgoing_mac_spoofing(&mut self, value: u64) {
        self.outgoing_mac_spoofing = Some(value);
    }

    /// Gets the value of OutgoingMacSpoofing
    pub fn get_outgoing_mac_spoofing(&self) -> Option<&u64> {
        self.outgoing_mac_spoofing.as_ref()
    }

    /// Sets the value of OutgoingMTUMismatch
    pub fn set_outgoing_mtumismatch(&mut self, value: u64) {
        self.outgoing_mtumismatch = Some(value);
    }

    /// Gets the value of OutgoingMTUMismatch
    pub fn get_outgoing_mtumismatch(&self) -> Option<&u64> {
        self.outgoing_mtumismatch.as_ref()
    }

    /// Sets the value of OutgoingNativeFwdingReq
    pub fn set_outgoing_native_fwding_req(&mut self, value: u64) {
        self.outgoing_native_fwding_req = Some(value);
    }

    /// Gets the value of OutgoingNativeFwdingReq
    pub fn get_outgoing_native_fwding_req(&self) -> Option<&u64> {
        self.outgoing_native_fwding_req.as_ref()
    }

    /// Sets the value of OutgoingNicDisabled
    pub fn set_outgoing_nic_disabled(&mut self, value: u64) {
        self.outgoing_nic_disabled = Some(value);
    }

    /// Gets the value of OutgoingNicDisabled
    pub fn get_outgoing_nic_disabled(&self) -> Option<&u64> {
        self.outgoing_nic_disabled.as_ref()
    }

    /// Sets the value of OutgoingNotAccepted
    pub fn set_outgoing_not_accepted(&mut self, value: u64) {
        self.outgoing_not_accepted = Some(value);
    }

    /// Gets the value of OutgoingNotAccepted
    pub fn get_outgoing_not_accepted(&self) -> Option<&u64> {
        self.outgoing_not_accepted.as_ref()
    }

    /// Sets the value of OutgoingNotReady
    pub fn set_outgoing_not_ready(&mut self, value: u64) {
        self.outgoing_not_ready = Some(value);
    }

    /// Gets the value of OutgoingNotReady
    pub fn get_outgoing_not_ready(&self) -> Option<&u64> {
        self.outgoing_not_ready.as_ref()
    }

    /// Sets the value of OutgoingQos
    pub fn set_outgoing_qos(&mut self, value: u64) {
        self.outgoing_qos = Some(value);
    }

    /// Gets the value of OutgoingQos
    pub fn get_outgoing_qos(&self) -> Option<&u64> {
        self.outgoing_qos.as_ref()
    }

    /// Sets the value of OutgoingRequiredExtensionMissing
    pub fn set_outgoing_required_extension_missing(&mut self, value: u64) {
        self.outgoing_required_extension_missing = Some(value);
    }

    /// Gets the value of OutgoingRequiredExtensionMissing
    pub fn get_outgoing_required_extension_missing(&self) -> Option<&u64> {
        self.outgoing_required_extension_missing.as_ref()
    }

    /// Sets the value of OutgoingResources
    pub fn set_outgoing_resources(&mut self, value: u64) {
        self.outgoing_resources = Some(value);
    }

    /// Gets the value of OutgoingResources
    pub fn get_outgoing_resources(&self) -> Option<&u64> {
        self.outgoing_resources.as_ref()
    }

    /// Sets the value of OutgoingRouterGuard
    pub fn set_outgoing_router_guard(&mut self, value: u64) {
        self.outgoing_router_guard = Some(value);
    }

    /// Gets the value of OutgoingRouterGuard
    pub fn get_outgoing_router_guard(&self) -> Option<&u64> {
        self.outgoing_router_guard.as_ref()
    }

    /// Sets the value of OutgoingStormLimit
    pub fn set_outgoing_storm_limit(&mut self, value: u64) {
        self.outgoing_storm_limit = Some(value);
    }

    /// Gets the value of OutgoingStormLimit
    pub fn get_outgoing_storm_limit(&self) -> Option<&u64> {
        self.outgoing_storm_limit.as_ref()
    }

    /// Sets the value of OutgoingSwitchDataFlowDisabled
    pub fn set_outgoing_switch_data_flow_disabled(&mut self, value: u64) {
        self.outgoing_switch_data_flow_disabled = Some(value);
    }

    /// Gets the value of OutgoingSwitchDataFlowDisabled
    pub fn get_outgoing_switch_data_flow_disabled(&self) -> Option<&u64> {
        self.outgoing_switch_data_flow_disabled.as_ref()
    }

    /// Sets the value of OutgoingUnauthorizedMAC
    pub fn set_outgoing_unauthorized_mac(&mut self, value: u64) {
        self.outgoing_unauthorized_mac = Some(value);
    }

    /// Gets the value of OutgoingUnauthorizedMAC
    pub fn get_outgoing_unauthorized_mac(&self) -> Option<&u64> {
        self.outgoing_unauthorized_mac.as_ref()
    }

    /// Sets the value of OutgoingUnauthorizedVLAN
    pub fn set_outgoing_unauthorized_vlan(&mut self, value: u64) {
        self.outgoing_unauthorized_vlan = Some(value);
    }

    /// Gets the value of OutgoingUnauthorizedVLAN
    pub fn get_outgoing_unauthorized_vlan(&self) -> Option<&u64> {
        self.outgoing_unauthorized_vlan.as_ref()
    }

    /// Sets the value of OutgoingUnknown
    pub fn set_outgoing_unknown(&mut self, value: u64) {
        self.outgoing_unknown = Some(value);
    }

    /// Gets the value of OutgoingUnknown
    pub fn get_outgoing_unknown(&self) -> Option<&u64> {
        self.outgoing_unknown.as_ref()
    }

    /// Sets the value of OutgoingVirtualSubnetId
    pub fn set_outgoing_virtual_subnet_id(&mut self, value: u64) {
        self.outgoing_virtual_subnet_id = Some(value);
    }

    /// Gets the value of OutgoingVirtualSubnetId
    pub fn get_outgoing_virtual_subnet_id(&self) -> Option<&u64> {
        self.outgoing_virtual_subnet_id.as_ref()
    }
}

