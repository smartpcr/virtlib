// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_NetworkAdapterConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_NetworkAdapterConfiguration {
    #[serde(flatten)]
    pub base: CIM_Setting,

/// 
    #[serde(rename = "ArpAlwaysSourceRoute")]
    pub arp_always_source_route: Option<bool>,

/// 
    #[serde(rename = "ArpUseEtherSNAP")]
    pub arp_use_ether_snap: Option<bool>,

/// 
    #[serde(rename = "DatabasePath")]
    pub database_path: Option<String>,

/// 
    #[serde(rename = "DeadGWDetectEnabled")]
    pub dead_gwdetect_enabled: Option<bool>,

/// 
    #[serde(rename = "DefaultIPGateway")]
    pub default_ipgateway: Vec<String>,

/// 
    #[serde(rename = "DefaultTOS")]
    pub default_tos: Option<u8>,

/// 
    #[serde(rename = "DefaultTTL")]
    pub default_ttl: Option<u8>,

/// 
    #[serde(rename = "DHCPEnabled")]
    pub dhcpenabled: Option<bool>,

/// 
    #[serde(rename = "DHCPLeaseExpires")]
    pub dhcplease_expires: Option<String>,

/// 
    #[serde(rename = "DHCPLeaseObtained")]
    pub dhcplease_obtained: Option<String>,

/// 
    #[serde(rename = "DHCPServer")]
    pub dhcpserver: Option<String>,

/// 
    #[serde(rename = "DNSDomain")]
    pub dnsdomain: Option<String>,

/// 
    #[serde(rename = "DNSDomainSuffixSearchOrder")]
    pub dnsdomain_suffix_search_order: Vec<String>,

/// 
    #[serde(rename = "DNSEnabledForWINSResolution")]
    pub dnsenabled_for_winsresolution: Option<bool>,

/// 
    #[serde(rename = "DNSHostName")]
    pub dnshost_name: Option<String>,

/// 
    #[serde(rename = "DNSServerSearchOrder")]
    pub dnsserver_search_order: Vec<String>,

/// 
    #[serde(rename = "DomainDNSRegistrationEnabled")]
    pub domain_dnsregistration_enabled: Option<bool>,

/// 
    #[serde(rename = "ForwardBufferMemory")]
    pub forward_buffer_memory: Option<u32>,

/// 
    #[serde(rename = "FullDNSRegistrationEnabled")]
    pub full_dnsregistration_enabled: Option<bool>,

/// 
    #[serde(rename = "GatewayCostMetric")]
    pub gateway_cost_metric: Vec<u16>,

/// 
    #[serde(rename = "IGMPLevel")]
    pub igmplevel: Option<u8>,

/// 
    #[serde(rename = "Index")]
    pub index: Option<u32>,

/// 
    #[serde(rename = "InterfaceIndex")]
    pub interface_index: Option<u32>,

/// 
    #[serde(rename = "IPAddress")]
    pub ipaddress: Vec<String>,

/// 
    #[serde(rename = "IPConnectionMetric")]
    pub ipconnection_metric: Option<u32>,

/// 
    #[serde(rename = "IPEnabled")]
    pub ipenabled: Option<bool>,

/// 
    #[serde(rename = "IPFilterSecurityEnabled")]
    pub ipfilter_security_enabled: Option<bool>,

/// 
    #[serde(rename = "IPPortSecurityEnabled")]
    pub ipport_security_enabled: Option<bool>,

/// 
    #[serde(rename = "IPSecPermitIPProtocols")]
    pub ipsec_permit_ipprotocols: Vec<String>,

/// 
    #[serde(rename = "IPSecPermitTCPPorts")]
    pub ipsec_permit_tcpports: Vec<String>,

/// 
    #[serde(rename = "IPSecPermitUDPPorts")]
    pub ipsec_permit_udpports: Vec<String>,

/// 
    #[serde(rename = "IPSubnet")]
    pub ipsubnet: Vec<String>,

/// 
    #[serde(rename = "IPUseZeroBroadcast")]
    pub ipuse_zero_broadcast: Option<bool>,

/// 
    #[serde(rename = "IPXAddress")]
    pub ipxaddress: Option<String>,

/// 
    #[serde(rename = "IPXEnabled")]
    pub ipxenabled: Option<bool>,

/// 
    #[serde(rename = "IPXFrameType")]
    pub ipxframe_type: Vec<u32>,

/// 
    #[serde(rename = "IPXMediaType")]
    pub ipxmedia_type: Option<u32>,

/// 
    #[serde(rename = "IPXNetworkNumber")]
    pub ipxnetwork_number: Vec<String>,

/// 
    #[serde(rename = "IPXVirtualNetNumber")]
    pub ipxvirtual_net_number: Option<String>,

/// 
    #[serde(rename = "KeepAliveInterval")]
    pub keep_alive_interval: Option<u32>,

/// 
    #[serde(rename = "KeepAliveTime")]
    pub keep_alive_time: Option<u32>,

/// 
    #[serde(rename = "MACAddress")]
    pub macaddress: Option<String>,

/// 
    #[serde(rename = "MTU")]
    pub mtu: Option<u32>,

/// 
    #[serde(rename = "NumForwardPackets")]
    pub num_forward_packets: Option<u32>,

/// 
    #[serde(rename = "PMTUBHDetectEnabled")]
    pub pmtubhdetect_enabled: Option<bool>,

/// 
    #[serde(rename = "PMTUDiscoveryEnabled")]
    pub pmtudiscovery_enabled: Option<bool>,

/// 
    #[serde(rename = "ServiceName")]
    pub service_name: Option<String>,

/// 
    #[serde(rename = "TcpipNetbiosOptions")]
    pub tcpip_netbios_options: Option<u32>,

/// 
    #[serde(rename = "TcpMaxConnectRetransmissions")]
    pub tcp_max_connect_retransmissions: Option<u32>,

/// 
    #[serde(rename = "TcpMaxDataRetransmissions")]
    pub tcp_max_data_retransmissions: Option<u32>,

/// 
    #[serde(rename = "TcpNumConnections")]
    pub tcp_num_connections: Option<u32>,

/// 
    #[serde(rename = "TcpUseRFC1122UrgentPointer")]
    pub tcp_use_rfc1122_urgent_pointer: Option<bool>,

/// 
    #[serde(rename = "TcpWindowSize")]
    pub tcp_window_size: Option<u16>,

/// 
    #[serde(rename = "WINSEnableLMHostsLookup")]
    pub winsenable_lmhosts_lookup: Option<bool>,

/// 
    #[serde(rename = "WINSHostLookupFile")]
    pub winshost_lookup_file: Option<String>,

/// 
    #[serde(rename = "WINSPrimaryServer")]
    pub winsprimary_server: Option<String>,

/// 
    #[serde(rename = "WINSScopeID")]
    pub winsscope_id: Option<String>,

/// 
    #[serde(rename = "WINSSecondaryServer")]
    pub winssecondary_server: Option<String>,
}

impl Win32_NetworkAdapterConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Setting::new(),
            arp_always_source_route: None,
            arp_use_ether_snap: None,
            database_path: None,
            dead_gwdetect_enabled: None,
            default_ipgateway: Vec::new(),
            default_tos: None,
            default_ttl: None,
            dhcpenabled: None,
            dhcplease_expires: None,
            dhcplease_obtained: None,
            dhcpserver: None,
            dnsdomain: None,
            dnsdomain_suffix_search_order: Vec::new(),
            dnsenabled_for_winsresolution: None,
            dnshost_name: None,
            dnsserver_search_order: Vec::new(),
            domain_dnsregistration_enabled: None,
            forward_buffer_memory: None,
            full_dnsregistration_enabled: None,
            gateway_cost_metric: Vec::new(),
            igmplevel: None,
            index: None,
            interface_index: None,
            ipaddress: Vec::new(),
            ipconnection_metric: None,
            ipenabled: None,
            ipfilter_security_enabled: None,
            ipport_security_enabled: None,
            ipsec_permit_ipprotocols: Vec::new(),
            ipsec_permit_tcpports: Vec::new(),
            ipsec_permit_udpports: Vec::new(),
            ipsubnet: Vec::new(),
            ipuse_zero_broadcast: None,
            ipxaddress: None,
            ipxenabled: None,
            ipxframe_type: Vec::new(),
            ipxmedia_type: None,
            ipxnetwork_number: Vec::new(),
            ipxvirtual_net_number: None,
            keep_alive_interval: None,
            keep_alive_time: None,
            macaddress: None,
            mtu: None,
            num_forward_packets: None,
            pmtubhdetect_enabled: None,
            pmtudiscovery_enabled: None,
            service_name: None,
            tcpip_netbios_options: None,
            tcp_max_connect_retransmissions: None,
            tcp_max_data_retransmissions: None,
            tcp_num_connections: None,
            tcp_use_rfc1122_urgent_pointer: None,
            tcp_window_size: None,
            winsenable_lmhosts_lookup: None,
            winshost_lookup_file: None,
            winsprimary_server: None,
            winsscope_id: None,
            winssecondary_server: None,
        }
    }


    /// Sets the value of ArpAlwaysSourceRoute
    pub fn set_arp_always_source_route(&mut self, value: bool) {
        self.arp_always_source_route = Some(value);
    }

    /// Gets the value of ArpAlwaysSourceRoute
    pub fn get_arp_always_source_route(&self) -> Option<&bool> {
        self.arp_always_source_route.as_ref()
    }

    /// Sets the value of ArpUseEtherSNAP
    pub fn set_arp_use_ether_snap(&mut self, value: bool) {
        self.arp_use_ether_snap = Some(value);
    }

    /// Gets the value of ArpUseEtherSNAP
    pub fn get_arp_use_ether_snap(&self) -> Option<&bool> {
        self.arp_use_ether_snap.as_ref()
    }

    /// Sets the value of DatabasePath
    pub fn set_database_path(&mut self, value: String) {
        self.database_path = Some(value);
    }

    /// Gets the value of DatabasePath
    pub fn get_database_path(&self) -> Option<&String> {
        self.database_path.as_ref()
    }

    /// Sets the value of DeadGWDetectEnabled
    pub fn set_dead_gwdetect_enabled(&mut self, value: bool) {
        self.dead_gwdetect_enabled = Some(value);
    }

    /// Gets the value of DeadGWDetectEnabled
    pub fn get_dead_gwdetect_enabled(&self) -> Option<&bool> {
        self.dead_gwdetect_enabled.as_ref()
    }

    /// Sets the value of DefaultIPGateway
    pub fn set_default_ipgateway(&mut self, value: Vec<String>) {
        self.default_ipgateway = value;
    }

    /// Gets the value of DefaultIPGateway
    pub fn get_default_ipgateway(&self) -> &Vec<String> {
        &self.default_ipgateway
    }

    /// Sets the value of DefaultTOS
    pub fn set_default_tos(&mut self, value: u8) {
        self.default_tos = Some(value);
    }

    /// Gets the value of DefaultTOS
    pub fn get_default_tos(&self) -> Option<&u8> {
        self.default_tos.as_ref()
    }

    /// Sets the value of DefaultTTL
    pub fn set_default_ttl(&mut self, value: u8) {
        self.default_ttl = Some(value);
    }

    /// Gets the value of DefaultTTL
    pub fn get_default_ttl(&self) -> Option<&u8> {
        self.default_ttl.as_ref()
    }

    /// Sets the value of DHCPEnabled
    pub fn set_dhcpenabled(&mut self, value: bool) {
        self.dhcpenabled = Some(value);
    }

    /// Gets the value of DHCPEnabled
    pub fn get_dhcpenabled(&self) -> Option<&bool> {
        self.dhcpenabled.as_ref()
    }

    /// Sets the value of DHCPLeaseExpires
    pub fn set_dhcplease_expires(&mut self, value: String) {
        self.dhcplease_expires = Some(value);
    }

    /// Gets the value of DHCPLeaseExpires
    pub fn get_dhcplease_expires(&self) -> Option<&String> {
        self.dhcplease_expires.as_ref()
    }

    /// Sets the value of DHCPLeaseObtained
    pub fn set_dhcplease_obtained(&mut self, value: String) {
        self.dhcplease_obtained = Some(value);
    }

    /// Gets the value of DHCPLeaseObtained
    pub fn get_dhcplease_obtained(&self) -> Option<&String> {
        self.dhcplease_obtained.as_ref()
    }

    /// Sets the value of DHCPServer
    pub fn set_dhcpserver(&mut self, value: String) {
        self.dhcpserver = Some(value);
    }

    /// Gets the value of DHCPServer
    pub fn get_dhcpserver(&self) -> Option<&String> {
        self.dhcpserver.as_ref()
    }

    /// Sets the value of DNSDomain
    pub fn set_dnsdomain(&mut self, value: String) {
        self.dnsdomain = Some(value);
    }

    /// Gets the value of DNSDomain
    pub fn get_dnsdomain(&self) -> Option<&String> {
        self.dnsdomain.as_ref()
    }

    /// Sets the value of DNSDomainSuffixSearchOrder
    pub fn set_dnsdomain_suffix_search_order(&mut self, value: Vec<String>) {
        self.dnsdomain_suffix_search_order = value;
    }

    /// Gets the value of DNSDomainSuffixSearchOrder
    pub fn get_dnsdomain_suffix_search_order(&self) -> &Vec<String> {
        &self.dnsdomain_suffix_search_order
    }

    /// Sets the value of DNSEnabledForWINSResolution
    pub fn set_dnsenabled_for_winsresolution(&mut self, value: bool) {
        self.dnsenabled_for_winsresolution = Some(value);
    }

    /// Gets the value of DNSEnabledForWINSResolution
    pub fn get_dnsenabled_for_winsresolution(&self) -> Option<&bool> {
        self.dnsenabled_for_winsresolution.as_ref()
    }

    /// Sets the value of DNSHostName
    pub fn set_dnshost_name(&mut self, value: String) {
        self.dnshost_name = Some(value);
    }

    /// Gets the value of DNSHostName
    pub fn get_dnshost_name(&self) -> Option<&String> {
        self.dnshost_name.as_ref()
    }

    /// Sets the value of DNSServerSearchOrder
    pub fn set_dnsserver_search_order(&mut self, value: Vec<String>) {
        self.dnsserver_search_order = value;
    }

    /// Gets the value of DNSServerSearchOrder
    pub fn get_dnsserver_search_order(&self) -> &Vec<String> {
        &self.dnsserver_search_order
    }

    /// Sets the value of DomainDNSRegistrationEnabled
    pub fn set_domain_dnsregistration_enabled(&mut self, value: bool) {
        self.domain_dnsregistration_enabled = Some(value);
    }

    /// Gets the value of DomainDNSRegistrationEnabled
    pub fn get_domain_dnsregistration_enabled(&self) -> Option<&bool> {
        self.domain_dnsregistration_enabled.as_ref()
    }

    /// Sets the value of ForwardBufferMemory
    pub fn set_forward_buffer_memory(&mut self, value: u32) {
        self.forward_buffer_memory = Some(value);
    }

    /// Gets the value of ForwardBufferMemory
    pub fn get_forward_buffer_memory(&self) -> Option<&u32> {
        self.forward_buffer_memory.as_ref()
    }

    /// Sets the value of FullDNSRegistrationEnabled
    pub fn set_full_dnsregistration_enabled(&mut self, value: bool) {
        self.full_dnsregistration_enabled = Some(value);
    }

    /// Gets the value of FullDNSRegistrationEnabled
    pub fn get_full_dnsregistration_enabled(&self) -> Option<&bool> {
        self.full_dnsregistration_enabled.as_ref()
    }

    /// Sets the value of GatewayCostMetric
    pub fn set_gateway_cost_metric(&mut self, value: Vec<u16>) {
        self.gateway_cost_metric = value;
    }

    /// Gets the value of GatewayCostMetric
    pub fn get_gateway_cost_metric(&self) -> &Vec<u16> {
        &self.gateway_cost_metric
    }

    /// Sets the value of IGMPLevel
    pub fn set_igmplevel(&mut self, value: u8) {
        self.igmplevel = Some(value);
    }

    /// Gets the value of IGMPLevel
    pub fn get_igmplevel(&self) -> Option<&u8> {
        self.igmplevel.as_ref()
    }

    /// Sets the value of Index
    pub fn set_index(&mut self, value: u32) {
        self.index = Some(value);
    }

    /// Gets the value of Index
    pub fn get_index(&self) -> Option<&u32> {
        self.index.as_ref()
    }

    /// Sets the value of InterfaceIndex
    pub fn set_interface_index(&mut self, value: u32) {
        self.interface_index = Some(value);
    }

    /// Gets the value of InterfaceIndex
    pub fn get_interface_index(&self) -> Option<&u32> {
        self.interface_index.as_ref()
    }

    /// Sets the value of IPAddress
    pub fn set_ipaddress(&mut self, value: Vec<String>) {
        self.ipaddress = value;
    }

    /// Gets the value of IPAddress
    pub fn get_ipaddress(&self) -> &Vec<String> {
        &self.ipaddress
    }

    /// Sets the value of IPConnectionMetric
    pub fn set_ipconnection_metric(&mut self, value: u32) {
        self.ipconnection_metric = Some(value);
    }

    /// Gets the value of IPConnectionMetric
    pub fn get_ipconnection_metric(&self) -> Option<&u32> {
        self.ipconnection_metric.as_ref()
    }

    /// Sets the value of IPEnabled
    pub fn set_ipenabled(&mut self, value: bool) {
        self.ipenabled = Some(value);
    }

    /// Gets the value of IPEnabled
    pub fn get_ipenabled(&self) -> Option<&bool> {
        self.ipenabled.as_ref()
    }

    /// Sets the value of IPFilterSecurityEnabled
    pub fn set_ipfilter_security_enabled(&mut self, value: bool) {
        self.ipfilter_security_enabled = Some(value);
    }

    /// Gets the value of IPFilterSecurityEnabled
    pub fn get_ipfilter_security_enabled(&self) -> Option<&bool> {
        self.ipfilter_security_enabled.as_ref()
    }

    /// Sets the value of IPPortSecurityEnabled
    pub fn set_ipport_security_enabled(&mut self, value: bool) {
        self.ipport_security_enabled = Some(value);
    }

    /// Gets the value of IPPortSecurityEnabled
    pub fn get_ipport_security_enabled(&self) -> Option<&bool> {
        self.ipport_security_enabled.as_ref()
    }

    /// Sets the value of IPSecPermitIPProtocols
    pub fn set_ipsec_permit_ipprotocols(&mut self, value: Vec<String>) {
        self.ipsec_permit_ipprotocols = value;
    }

    /// Gets the value of IPSecPermitIPProtocols
    pub fn get_ipsec_permit_ipprotocols(&self) -> &Vec<String> {
        &self.ipsec_permit_ipprotocols
    }

    /// Sets the value of IPSecPermitTCPPorts
    pub fn set_ipsec_permit_tcpports(&mut self, value: Vec<String>) {
        self.ipsec_permit_tcpports = value;
    }

    /// Gets the value of IPSecPermitTCPPorts
    pub fn get_ipsec_permit_tcpports(&self) -> &Vec<String> {
        &self.ipsec_permit_tcpports
    }

    /// Sets the value of IPSecPermitUDPPorts
    pub fn set_ipsec_permit_udpports(&mut self, value: Vec<String>) {
        self.ipsec_permit_udpports = value;
    }

    /// Gets the value of IPSecPermitUDPPorts
    pub fn get_ipsec_permit_udpports(&self) -> &Vec<String> {
        &self.ipsec_permit_udpports
    }

    /// Sets the value of IPSubnet
    pub fn set_ipsubnet(&mut self, value: Vec<String>) {
        self.ipsubnet = value;
    }

    /// Gets the value of IPSubnet
    pub fn get_ipsubnet(&self) -> &Vec<String> {
        &self.ipsubnet
    }

    /// Sets the value of IPUseZeroBroadcast
    pub fn set_ipuse_zero_broadcast(&mut self, value: bool) {
        self.ipuse_zero_broadcast = Some(value);
    }

    /// Gets the value of IPUseZeroBroadcast
    pub fn get_ipuse_zero_broadcast(&self) -> Option<&bool> {
        self.ipuse_zero_broadcast.as_ref()
    }

    /// Sets the value of IPXAddress
    pub fn set_ipxaddress(&mut self, value: String) {
        self.ipxaddress = Some(value);
    }

    /// Gets the value of IPXAddress
    pub fn get_ipxaddress(&self) -> Option<&String> {
        self.ipxaddress.as_ref()
    }

    /// Sets the value of IPXEnabled
    pub fn set_ipxenabled(&mut self, value: bool) {
        self.ipxenabled = Some(value);
    }

    /// Gets the value of IPXEnabled
    pub fn get_ipxenabled(&self) -> Option<&bool> {
        self.ipxenabled.as_ref()
    }

    /// Sets the value of IPXFrameType
    pub fn set_ipxframe_type(&mut self, value: Vec<u32>) {
        self.ipxframe_type = value;
    }

    /// Gets the value of IPXFrameType
    pub fn get_ipxframe_type(&self) -> &Vec<u32> {
        &self.ipxframe_type
    }

    /// Sets the value of IPXMediaType
    pub fn set_ipxmedia_type(&mut self, value: u32) {
        self.ipxmedia_type = Some(value);
    }

    /// Gets the value of IPXMediaType
    pub fn get_ipxmedia_type(&self) -> Option<&u32> {
        self.ipxmedia_type.as_ref()
    }

    /// Sets the value of IPXNetworkNumber
    pub fn set_ipxnetwork_number(&mut self, value: Vec<String>) {
        self.ipxnetwork_number = value;
    }

    /// Gets the value of IPXNetworkNumber
    pub fn get_ipxnetwork_number(&self) -> &Vec<String> {
        &self.ipxnetwork_number
    }

    /// Sets the value of IPXVirtualNetNumber
    pub fn set_ipxvirtual_net_number(&mut self, value: String) {
        self.ipxvirtual_net_number = Some(value);
    }

    /// Gets the value of IPXVirtualNetNumber
    pub fn get_ipxvirtual_net_number(&self) -> Option<&String> {
        self.ipxvirtual_net_number.as_ref()
    }

    /// Sets the value of KeepAliveInterval
    pub fn set_keep_alive_interval(&mut self, value: u32) {
        self.keep_alive_interval = Some(value);
    }

    /// Gets the value of KeepAliveInterval
    pub fn get_keep_alive_interval(&self) -> Option<&u32> {
        self.keep_alive_interval.as_ref()
    }

    /// Sets the value of KeepAliveTime
    pub fn set_keep_alive_time(&mut self, value: u32) {
        self.keep_alive_time = Some(value);
    }

    /// Gets the value of KeepAliveTime
    pub fn get_keep_alive_time(&self) -> Option<&u32> {
        self.keep_alive_time.as_ref()
    }

    /// Sets the value of MACAddress
    pub fn set_macaddress(&mut self, value: String) {
        self.macaddress = Some(value);
    }

    /// Gets the value of MACAddress
    pub fn get_macaddress(&self) -> Option<&String> {
        self.macaddress.as_ref()
    }

    /// Sets the value of MTU
    pub fn set_mtu(&mut self, value: u32) {
        self.mtu = Some(value);
    }

    /// Gets the value of MTU
    pub fn get_mtu(&self) -> Option<&u32> {
        self.mtu.as_ref()
    }

    /// Sets the value of NumForwardPackets
    pub fn set_num_forward_packets(&mut self, value: u32) {
        self.num_forward_packets = Some(value);
    }

    /// Gets the value of NumForwardPackets
    pub fn get_num_forward_packets(&self) -> Option<&u32> {
        self.num_forward_packets.as_ref()
    }

    /// Sets the value of PMTUBHDetectEnabled
    pub fn set_pmtubhdetect_enabled(&mut self, value: bool) {
        self.pmtubhdetect_enabled = Some(value);
    }

    /// Gets the value of PMTUBHDetectEnabled
    pub fn get_pmtubhdetect_enabled(&self) -> Option<&bool> {
        self.pmtubhdetect_enabled.as_ref()
    }

    /// Sets the value of PMTUDiscoveryEnabled
    pub fn set_pmtudiscovery_enabled(&mut self, value: bool) {
        self.pmtudiscovery_enabled = Some(value);
    }

    /// Gets the value of PMTUDiscoveryEnabled
    pub fn get_pmtudiscovery_enabled(&self) -> Option<&bool> {
        self.pmtudiscovery_enabled.as_ref()
    }

    /// Sets the value of ServiceName
    pub fn set_service_name(&mut self, value: String) {
        self.service_name = Some(value);
    }

    /// Gets the value of ServiceName
    pub fn get_service_name(&self) -> Option<&String> {
        self.service_name.as_ref()
    }

    /// Sets the value of TcpipNetbiosOptions
    pub fn set_tcpip_netbios_options(&mut self, value: u32) {
        self.tcpip_netbios_options = Some(value);
    }

    /// Gets the value of TcpipNetbiosOptions
    pub fn get_tcpip_netbios_options(&self) -> Option<&u32> {
        self.tcpip_netbios_options.as_ref()
    }

    /// Sets the value of TcpMaxConnectRetransmissions
    pub fn set_tcp_max_connect_retransmissions(&mut self, value: u32) {
        self.tcp_max_connect_retransmissions = Some(value);
    }

    /// Gets the value of TcpMaxConnectRetransmissions
    pub fn get_tcp_max_connect_retransmissions(&self) -> Option<&u32> {
        self.tcp_max_connect_retransmissions.as_ref()
    }

    /// Sets the value of TcpMaxDataRetransmissions
    pub fn set_tcp_max_data_retransmissions(&mut self, value: u32) {
        self.tcp_max_data_retransmissions = Some(value);
    }

    /// Gets the value of TcpMaxDataRetransmissions
    pub fn get_tcp_max_data_retransmissions(&self) -> Option<&u32> {
        self.tcp_max_data_retransmissions.as_ref()
    }

    /// Sets the value of TcpNumConnections
    pub fn set_tcp_num_connections(&mut self, value: u32) {
        self.tcp_num_connections = Some(value);
    }

    /// Gets the value of TcpNumConnections
    pub fn get_tcp_num_connections(&self) -> Option<&u32> {
        self.tcp_num_connections.as_ref()
    }

    /// Sets the value of TcpUseRFC1122UrgentPointer
    pub fn set_tcp_use_rfc1122_urgent_pointer(&mut self, value: bool) {
        self.tcp_use_rfc1122_urgent_pointer = Some(value);
    }

    /// Gets the value of TcpUseRFC1122UrgentPointer
    pub fn get_tcp_use_rfc1122_urgent_pointer(&self) -> Option<&bool> {
        self.tcp_use_rfc1122_urgent_pointer.as_ref()
    }

    /// Sets the value of TcpWindowSize
    pub fn set_tcp_window_size(&mut self, value: u16) {
        self.tcp_window_size = Some(value);
    }

    /// Gets the value of TcpWindowSize
    pub fn get_tcp_window_size(&self) -> Option<&u16> {
        self.tcp_window_size.as_ref()
    }

    /// Sets the value of WINSEnableLMHostsLookup
    pub fn set_winsenable_lmhosts_lookup(&mut self, value: bool) {
        self.winsenable_lmhosts_lookup = Some(value);
    }

    /// Gets the value of WINSEnableLMHostsLookup
    pub fn get_winsenable_lmhosts_lookup(&self) -> Option<&bool> {
        self.winsenable_lmhosts_lookup.as_ref()
    }

    /// Sets the value of WINSHostLookupFile
    pub fn set_winshost_lookup_file(&mut self, value: String) {
        self.winshost_lookup_file = Some(value);
    }

    /// Gets the value of WINSHostLookupFile
    pub fn get_winshost_lookup_file(&self) -> Option<&String> {
        self.winshost_lookup_file.as_ref()
    }

    /// Sets the value of WINSPrimaryServer
    pub fn set_winsprimary_server(&mut self, value: String) {
        self.winsprimary_server = Some(value);
    }

    /// Gets the value of WINSPrimaryServer
    pub fn get_winsprimary_server(&self) -> Option<&String> {
        self.winsprimary_server.as_ref()
    }

    /// Sets the value of WINSScopeID
    pub fn set_winsscope_id(&mut self, value: String) {
        self.winsscope_id = Some(value);
    }

    /// Gets the value of WINSScopeID
    pub fn get_winsscope_id(&self) -> Option<&String> {
        self.winsscope_id.as_ref()
    }

    /// Sets the value of WINSSecondaryServer
    pub fn set_winssecondary_server(&mut self, value: String) {
        self.winssecondary_server = Some(value);
    }

    /// Gets the value of WINSSecondaryServer
    pub fn get_winssecondary_server(&self) -> Option<&String> {
        self.winssecondary_server.as_ref()
    }

/// 

    /// * `return_value` -  (u32)
    pub fn enable_dhcp(&self) -> Result<(), WmiError> {
        self.invoke_method("EnableDHCP", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn renew_dhcplease(&self) -> Result<(), WmiError> {
        self.invoke_method("RenewDHCPLease", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn renew_dhcplease_all(&self) -> Result<(), WmiError> {
        self.invoke_method("RenewDHCPLeaseAll", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn release_dhcplease(&self) -> Result<(), WmiError> {
        self.invoke_method("ReleaseDHCPLease", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn release_dhcplease_all(&self) -> Result<(), WmiError> {
        self.invoke_method("ReleaseDHCPLeaseAll", &[])

    }


/// 

    /// * `ipaddress` -  (String[])
    /// * `subnet_mask` -  (String[])

    /// * `return_value` -  (u32)
    pub fn enable_static(&self, ipaddress: &Vec<String>, subnet_mask: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "IPAddress".to_string(), value: ipaddress.into() });
        args.push(MethodParameter { name: "SubnetMask".to_string(), value: subnet_mask.into() });
        self.invoke_method("EnableStatic", &args)

    }


/// 

    /// * `default_ipgateway` -  (String[])
    /// * `gateway_cost_metric` -  (u16[])

    /// * `return_value` -  (u32)
    pub fn set_gateways(&self, default_ipgateway: &Vec<String>, gateway_cost_metric: &Option<Vec<u16>>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DefaultIPGateway".to_string(), value: default_ipgateway.into() });
        if let Some(val) = gateway_cost_metric {
            args.push(MethodParameter { name: "GatewayCostMetric".to_string(), value: val.into() });
        }
        self.invoke_method("SetGateways", &args)

    }


/// 

    /// * `dnsdomain` -  (String)
    /// * `dnsdomain_suffix_search_order` -  (String[])
    /// * `dnshost_name` -  (String)
    /// * `dnsserver_search_order` -  (String[])

    /// * `return_value` -  (u32)
    pub fn enable_dns(&self, dnshost_name: &Option<String>, dnsdomain: &Option<String>, dnsserver_search_order: &Option<Vec<String>>, dnsdomain_suffix_search_order: &Option<Vec<String>>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = dnshost_name {
            args.push(MethodParameter { name: "DNSHostName".to_string(), value: val.into() });
        }
        if let Some(val) = dnsdomain {
            args.push(MethodParameter { name: "DNSDomain".to_string(), value: val.into() });
        }
        if let Some(val) = dnsserver_search_order {
            args.push(MethodParameter { name: "DNSServerSearchOrder".to_string(), value: val.into() });
        }
        if let Some(val) = dnsdomain_suffix_search_order {
            args.push(MethodParameter { name: "DNSDomainSuffixSearchOrder".to_string(), value: val.into() });
        }
        self.invoke_method("EnableDNS", &args)

    }


/// 

    /// * `dnsdomain` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_dnsdomain(&self, dnsdomain: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DNSDomain".to_string(), value: dnsdomain.into() });
        self.invoke_method("SetDNSDomain", &args)

    }


/// 

    /// * `dnsserver_search_order` -  (String[])

    /// * `return_value` -  (u32)
    pub fn set_dnsserver_search_order(&self, dnsserver_search_order: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DNSServerSearchOrder".to_string(), value: dnsserver_search_order.into() });
        self.invoke_method("SetDNSServerSearchOrder", &args)

    }


/// 

    /// * `dnsdomain_suffix_search_order` -  (String[])

    /// * `return_value` -  (u32)
    pub fn set_dnssuffix_search_order(&self, dnsdomain_suffix_search_order: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DNSDomainSuffixSearchOrder".to_string(), value: dnsdomain_suffix_search_order.into() });
        self.invoke_method("SetDNSSuffixSearchOrder", &args)

    }


/// 

    /// * `domain_dnsregistration_enabled` -  (bool)
    /// * `full_dnsregistration_enabled` -  (bool)

    /// * `return_value` -  (u32)
    pub fn set_dynamic_dnsregistration(&self, full_dnsregistration_enabled: bool, domain_dnsregistration_enabled: Option<bool>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FullDNSRegistrationEnabled".to_string(), value: full_dnsregistration_enabled.into() });
        if let Some(val) = domain_dnsregistration_enabled {
            args.push(MethodParameter { name: "DomainDNSRegistrationEnabled".to_string(), value: val.into() });
        }
        self.invoke_method("SetDynamicDNSRegistration", &args)

    }


/// 

    /// * `ipconnection_metric` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_ipconnection_metric(&self, ipconnection_metric: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "IPConnectionMetric".to_string(), value: ipconnection_metric.into() });
        self.invoke_method("SetIPConnectionMetric", &args)

    }


/// 

    /// * `winsprimary_server` -  (String)
    /// * `winssecondary_server` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_winsserver(&self, winsprimary_server: &String, winssecondary_server: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "WINSPrimaryServer".to_string(), value: winsprimary_server.into() });
        args.push(MethodParameter { name: "WINSSecondaryServer".to_string(), value: winssecondary_server.into() });
        self.invoke_method("SetWINSServer", &args)

    }


/// 

    /// * `dnsenabled_for_winsresolution` -  (bool)
    /// * `winsenable_lmhosts_lookup` -  (bool)
    /// * `winshost_lookup_file` -  (String)
    /// * `winsscope_id` -  (String)

    /// * `return_value` -  (u32)
    pub fn enable_wins(&self, dnsenabled_for_winsresolution: bool, winsenable_lmhosts_lookup: bool, winshost_lookup_file: &Option<String>, winsscope_id: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DNSEnabledForWINSResolution".to_string(), value: dnsenabled_for_winsresolution.into() });
        args.push(MethodParameter { name: "WINSEnableLMHostsLookup".to_string(), value: winsenable_lmhosts_lookup.into() });
        if let Some(val) = winshost_lookup_file {
            args.push(MethodParameter { name: "WINSHostLookupFile".to_string(), value: val.into() });
        }
        if let Some(val) = winsscope_id {
            args.push(MethodParameter { name: "WINSScopeID".to_string(), value: val.into() });
        }
        self.invoke_method("EnableWINS", &args)

    }


/// 

    /// * `tcpip_netbios_options` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_tcpip_netbios(&self, tcpip_netbios_options: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TcpipNetbiosOptions".to_string(), value: tcpip_netbios_options.into() });
        self.invoke_method("SetTcpipNetbios", &args)

    }


/// 

    /// * `ipsec_permit_ipprotocols` -  (String[])
    /// * `ipsec_permit_tcpports` -  (String[])
    /// * `ipsec_permit_udpports` -  (String[])

    /// * `return_value` -  (u32)
    pub fn enable_ipsec(&self, ipsec_permit_tcpports: &Vec<String>, ipsec_permit_udpports: &Vec<String>, ipsec_permit_ipprotocols: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "IPSecPermitTCPPorts".to_string(), value: ipsec_permit_tcpports.into() });
        args.push(MethodParameter { name: "IPSecPermitUDPPorts".to_string(), value: ipsec_permit_udpports.into() });
        args.push(MethodParameter { name: "IPSecPermitIPProtocols".to_string(), value: ipsec_permit_ipprotocols.into() });
        self.invoke_method("EnableIPSec", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn disable_ipsec(&self) -> Result<(), WmiError> {
        self.invoke_method("DisableIPSec", &[])

    }


/// 

    /// * `ipxvirtual_net_number` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_ipxvirtual_network_number(&self, ipxvirtual_net_number: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "IPXVirtualNetNumber".to_string(), value: ipxvirtual_net_number.into() });
        self.invoke_method("SetIPXVirtualNetworkNumber", &args)

    }


/// 

    /// * `ipxframe_type` -  (u32[])
    /// * `ipxnetwork_number` -  (String[])

    /// * `return_value` -  (u32)
    pub fn set_ipxframe_type_network_pairs(&self, ipxnetwork_number: &Vec<String>, ipxframe_type: &Vec<u32>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "IPXNetworkNumber".to_string(), value: ipxnetwork_number.into() });
        args.push(MethodParameter { name: "IPXFrameType".to_string(), value: ipxframe_type.into() });
        self.invoke_method("SetIPXFrameTypeNetworkPairs", &args)

    }


/// 

    /// * `database_path` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_database_path(&self, database_path: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DatabasePath".to_string(), value: database_path.into() });
        self.invoke_method("SetDatabasePath", &args)

    }


/// 

    /// * `ipuse_zero_broadcast` -  (bool)

    /// * `return_value` -  (u32)
    pub fn set_ipuse_zero_broadcast(&self, ipuse_zero_broadcast: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "IPUseZeroBroadcast".to_string(), value: ipuse_zero_broadcast.into() });
        self.invoke_method("SetIPUseZeroBroadcast", &args)

    }


/// 

    /// * `arp_always_source_route` -  (bool)

    /// * `return_value` -  (u32)
    pub fn set_arp_always_source_route(&self, arp_always_source_route: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ArpAlwaysSourceRoute".to_string(), value: arp_always_source_route.into() });
        self.invoke_method("SetArpAlwaysSourceRoute", &args)

    }


/// 

    /// * `arp_use_ether_snap` -  (bool)

    /// * `return_value` -  (u32)
    pub fn set_arp_use_ether_snap(&self, arp_use_ether_snap: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ArpUseEtherSNAP".to_string(), value: arp_use_ether_snap.into() });
        self.invoke_method("SetArpUseEtherSNAP", &args)

    }


/// 

    /// * `default_tos` -  (u8)

    /// * `return_value` -  (u32)
    pub fn set_default_tos(&self, default_tos: u8) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DefaultTOS".to_string(), value: default_tos.into() });
        self.invoke_method("SetDefaultTOS", &args)

    }


/// 

    /// * `default_ttl` -  (u8)

    /// * `return_value` -  (u32)
    pub fn set_default_ttl(&self, default_ttl: u8) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DefaultTTL".to_string(), value: default_ttl.into() });
        self.invoke_method("SetDefaultTTL", &args)

    }


/// 

    /// * `dead_gwdetect_enabled` -  (bool)

    /// * `return_value` -  (u32)
    pub fn set_dead_gwdetect(&self, dead_gwdetect_enabled: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DeadGWDetectEnabled".to_string(), value: dead_gwdetect_enabled.into() });
        self.invoke_method("SetDeadGWDetect", &args)

    }


/// 

    /// * `pmtubhdetect_enabled` -  (bool)

    /// * `return_value` -  (u32)
    pub fn set_pmtubhdetect(&self, pmtubhdetect_enabled: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PMTUBHDetectEnabled".to_string(), value: pmtubhdetect_enabled.into() });
        self.invoke_method("SetPMTUBHDetect", &args)

    }


/// 

    /// * `pmtudiscovery_enabled` -  (bool)

    /// * `return_value` -  (u32)
    pub fn set_pmtudiscovery(&self, pmtudiscovery_enabled: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PMTUDiscoveryEnabled".to_string(), value: pmtudiscovery_enabled.into() });
        self.invoke_method("SetPMTUDiscovery", &args)

    }


/// 

    /// * `forward_buffer_memory` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_forward_buffer_memory(&self, forward_buffer_memory: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ForwardBufferMemory".to_string(), value: forward_buffer_memory.into() });
        self.invoke_method("SetForwardBufferMemory", &args)

    }


/// 

    /// * `igmplevel` -  (u8)

    /// * `return_value` -  (u32)
    pub fn set_igmplevel(&self, igmplevel: u8) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "IGMPLevel".to_string(), value: igmplevel.into() });
        self.invoke_method("SetIGMPLevel", &args)

    }


/// 

    /// * `keep_alive_interval` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_keep_alive_interval(&self, keep_alive_interval: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "KeepAliveInterval".to_string(), value: keep_alive_interval.into() });
        self.invoke_method("SetKeepAliveInterval", &args)

    }


/// 

    /// * `keep_alive_time` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_keep_alive_time(&self, keep_alive_time: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "KeepAliveTime".to_string(), value: keep_alive_time.into() });
        self.invoke_method("SetKeepAliveTime", &args)

    }


/// 

    /// * `mtu` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_mtu(&self, mtu: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "MTU".to_string(), value: mtu.into() });
        self.invoke_method("SetMTU", &args)

    }


/// 

    /// * `num_forward_packets` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_num_forward_packets(&self, num_forward_packets: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NumForwardPackets".to_string(), value: num_forward_packets.into() });
        self.invoke_method("SetNumForwardPackets", &args)

    }


/// 

    /// * `tcp_max_connect_retransmissions` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_tcp_max_connect_retransmissions(&self, tcp_max_connect_retransmissions: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TcpMaxConnectRetransmissions".to_string(), value: tcp_max_connect_retransmissions.into() });
        self.invoke_method("SetTcpMaxConnectRetransmissions", &args)

    }


/// 

    /// * `tcp_max_data_retransmissions` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_tcp_max_data_retransmissions(&self, tcp_max_data_retransmissions: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TcpMaxDataRetransmissions".to_string(), value: tcp_max_data_retransmissions.into() });
        self.invoke_method("SetTcpMaxDataRetransmissions", &args)

    }


/// 

    /// * `tcp_num_connections` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_tcp_num_connections(&self, tcp_num_connections: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TcpNumConnections".to_string(), value: tcp_num_connections.into() });
        self.invoke_method("SetTcpNumConnections", &args)

    }


/// 

    /// * `tcp_use_rfc1122_urgent_pointer` -  (bool)

    /// * `return_value` -  (u32)
    pub fn set_tcp_use_rfc1122_urgent_pointer(&self, tcp_use_rfc1122_urgent_pointer: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TcpUseRFC1122UrgentPointer".to_string(), value: tcp_use_rfc1122_urgent_pointer.into() });
        self.invoke_method("SetTcpUseRFC1122UrgentPointer", &args)

    }


/// 

    /// * `tcp_window_size` -  (u16)

    /// * `return_value` -  (u32)
    pub fn set_tcp_window_size(&self, tcp_window_size: u16) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TcpWindowSize".to_string(), value: tcp_window_size.into() });
        self.invoke_method("SetTcpWindowSize", &args)

    }


/// 

    /// * `ipfilter_security_enabled` -  (bool)

    /// * `return_value` -  (u32)
    pub fn enable_ipfilter_sec(&self, ipfilter_security_enabled: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "IPFilterSecurityEnabled".to_string(), value: ipfilter_security_enabled.into() });
        self.invoke_method("EnableIPFilterSec", &args)

    }

}

