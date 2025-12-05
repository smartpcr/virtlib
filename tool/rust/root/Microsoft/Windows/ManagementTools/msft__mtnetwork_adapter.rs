// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ManagementTools
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MTNetworkAdapter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MTNetworkAdapter {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "BytesReceived")]
    pub bytes_received: Option<u64>,

/// 
    #[serde(rename = "BytesReceivedPerInterval")]
    pub bytes_received_per_interval: Option<u64>,

/// 
    #[serde(rename = "BytesReceivedThroughput")]
    pub bytes_received_throughput: Option<f32>,

/// 
    #[serde(rename = "BytesSent")]
    pub bytes_sent: Option<u64>,

/// 
    #[serde(rename = "BytesSentPerInterval")]
    pub bytes_sent_per_interval: Option<u64>,

/// 
    #[serde(rename = "BytesSentThroughput")]
    pub bytes_sent_throughput: Option<f32>,

/// 
    #[serde(rename = "BytesTotal")]
    pub bytes_total: Option<u64>,

/// 
    #[serde(rename = "BytesTotalPerInterval")]
    pub bytes_total_per_interval: Option<u64>,

/// 
    #[serde(rename = "BytesTotalThroughput")]
    pub bytes_total_throughput: Option<f32>,

/// 
    #[serde(rename = "CurrentIndex")]
    pub current_index: Option<u16>,

/// 
    #[serde(rename = "DNSName")]
    pub dnsname: Option<String>,

/// 
    #[serde(rename = "InterfaceDescription")]
    pub interface_description: Option<String>,

/// 
    #[serde(rename = "InterfaceGuid")]
    pub interface_guid: Option<String>,

/// 
    #[serde(rename = "IntervalSeconds")]
    pub interval_seconds: Option<u16>,

/// 
    #[serde(rename = "IPv4Address")]
    pub ipv4_address: Option<String>,

/// 
    #[serde(rename = "IPv6Address")]
    pub ipv6_address: Option<String>,

/// 
    #[serde(rename = "LinkSpeed")]
    pub link_speed: Option<u64>,

/// 
    #[serde(rename = "MachineJoinedName")]
    pub machine_joined_name: Option<String>,

/// 
    #[serde(rename = "MachineJoinedType")]
    pub machine_joined_type: Option<u16>,

/// 
    #[serde(rename = "MaxReceivedBitsPerSecond")]
    pub max_received_bits_per_second: Vec<f32>,

/// 
    #[serde(rename = "MaxSentBitsPerSecond")]
    pub max_sent_bits_per_second: Vec<f32>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "NdisMedium")]
    pub ndis_medium: Option<u32>,

/// 
    #[serde(rename = "NdisPhysicalMedium")]
    pub ndis_physical_medium: Option<u32>,

/// 
    #[serde(rename = "NonUniCastsReceived")]
    pub non_uni_casts_received: Option<u64>,

/// 
    #[serde(rename = "NonUniCastsReceivedPerInterval")]
    pub non_uni_casts_received_per_interval: Option<u64>,

/// 
    #[serde(rename = "NonUniCastsSent")]
    pub non_uni_casts_sent: Option<u64>,

/// 
    #[serde(rename = "NonUniCastsSentPerInterval")]
    pub non_uni_casts_sent_per_interval: Option<u64>,

/// 
    #[serde(rename = "NonUniCastsTotal")]
    pub non_uni_casts_total: Option<u64>,

/// 
    #[serde(rename = "NonUniCastsTotalPerInterval")]
    pub non_uni_casts_total_per_interval: Option<u64>,

/// 
    #[serde(rename = "OperationStatus")]
    pub operation_status: Option<u16>,

/// 
    #[serde(rename = "ReceivedBitsPerSecond")]
    pub received_bits_per_second: Vec<f32>,

/// 
    #[serde(rename = "ReceivedThroughput")]
    pub received_throughput: Vec<f32>,

/// 
    #[serde(rename = "SentBitsPerSecond")]
    pub sent_bits_per_second: Vec<f32>,

/// 
    #[serde(rename = "SentThroughput")]
    pub sent_throughput: Vec<f32>,

/// 
    #[serde(rename = "UniCastsReceived")]
    pub uni_casts_received: Option<u64>,

/// 
    #[serde(rename = "UniCastsReceivedPerInterval")]
    pub uni_casts_received_per_interval: Option<u64>,

/// 
    #[serde(rename = "UniCastsSent")]
    pub uni_casts_sent: Option<u64>,

/// 
    #[serde(rename = "UniCastsSentPerInterval")]
    pub uni_casts_sent_per_interval: Option<u64>,

/// 
    #[serde(rename = "UniCastsTotal")]
    pub uni_casts_total: Option<u64>,

/// 
    #[serde(rename = "UniCastsTotalPerInterval")]
    pub uni_casts_total_per_interval: Option<u64>,

/// 
    #[serde(rename = "Utilization")]
    pub utilization: Option<f32>,
}

impl MSFT_MTNetworkAdapter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            bytes_received: None,
            bytes_received_per_interval: None,
            bytes_received_throughput: None,
            bytes_sent: None,
            bytes_sent_per_interval: None,
            bytes_sent_throughput: None,
            bytes_total: None,
            bytes_total_per_interval: None,
            bytes_total_throughput: None,
            current_index: None,
            dnsname: None,
            interface_description: None,
            interface_guid: None,
            interval_seconds: None,
            ipv4_address: None,
            ipv6_address: None,
            link_speed: None,
            machine_joined_name: None,
            machine_joined_type: None,
            max_received_bits_per_second: Vec::new(),
            max_sent_bits_per_second: Vec::new(),
            name: None,
            ndis_medium: None,
            ndis_physical_medium: None,
            non_uni_casts_received: None,
            non_uni_casts_received_per_interval: None,
            non_uni_casts_sent: None,
            non_uni_casts_sent_per_interval: None,
            non_uni_casts_total: None,
            non_uni_casts_total_per_interval: None,
            operation_status: None,
            received_bits_per_second: Vec::new(),
            received_throughput: Vec::new(),
            sent_bits_per_second: Vec::new(),
            sent_throughput: Vec::new(),
            uni_casts_received: None,
            uni_casts_received_per_interval: None,
            uni_casts_sent: None,
            uni_casts_sent_per_interval: None,
            uni_casts_total: None,
            uni_casts_total_per_interval: None,
            utilization: None,
        }
    }


    /// Sets the value of BytesReceived
    pub fn set_bytes_received(&mut self, value: u64) {
        self.bytes_received = Some(value);
    }

    /// Gets the value of BytesReceived
    pub fn get_bytes_received(&self) -> Option<&u64> {
        self.bytes_received.as_ref()
    }

    /// Sets the value of BytesReceivedPerInterval
    pub fn set_bytes_received_per_interval(&mut self, value: u64) {
        self.bytes_received_per_interval = Some(value);
    }

    /// Gets the value of BytesReceivedPerInterval
    pub fn get_bytes_received_per_interval(&self) -> Option<&u64> {
        self.bytes_received_per_interval.as_ref()
    }

    /// Sets the value of BytesReceivedThroughput
    pub fn set_bytes_received_throughput(&mut self, value: f32) {
        self.bytes_received_throughput = Some(value);
    }

    /// Gets the value of BytesReceivedThroughput
    pub fn get_bytes_received_throughput(&self) -> Option<&f32> {
        self.bytes_received_throughput.as_ref()
    }

    /// Sets the value of BytesSent
    pub fn set_bytes_sent(&mut self, value: u64) {
        self.bytes_sent = Some(value);
    }

    /// Gets the value of BytesSent
    pub fn get_bytes_sent(&self) -> Option<&u64> {
        self.bytes_sent.as_ref()
    }

    /// Sets the value of BytesSentPerInterval
    pub fn set_bytes_sent_per_interval(&mut self, value: u64) {
        self.bytes_sent_per_interval = Some(value);
    }

    /// Gets the value of BytesSentPerInterval
    pub fn get_bytes_sent_per_interval(&self) -> Option<&u64> {
        self.bytes_sent_per_interval.as_ref()
    }

    /// Sets the value of BytesSentThroughput
    pub fn set_bytes_sent_throughput(&mut self, value: f32) {
        self.bytes_sent_throughput = Some(value);
    }

    /// Gets the value of BytesSentThroughput
    pub fn get_bytes_sent_throughput(&self) -> Option<&f32> {
        self.bytes_sent_throughput.as_ref()
    }

    /// Sets the value of BytesTotal
    pub fn set_bytes_total(&mut self, value: u64) {
        self.bytes_total = Some(value);
    }

    /// Gets the value of BytesTotal
    pub fn get_bytes_total(&self) -> Option<&u64> {
        self.bytes_total.as_ref()
    }

    /// Sets the value of BytesTotalPerInterval
    pub fn set_bytes_total_per_interval(&mut self, value: u64) {
        self.bytes_total_per_interval = Some(value);
    }

    /// Gets the value of BytesTotalPerInterval
    pub fn get_bytes_total_per_interval(&self) -> Option<&u64> {
        self.bytes_total_per_interval.as_ref()
    }

    /// Sets the value of BytesTotalThroughput
    pub fn set_bytes_total_throughput(&mut self, value: f32) {
        self.bytes_total_throughput = Some(value);
    }

    /// Gets the value of BytesTotalThroughput
    pub fn get_bytes_total_throughput(&self) -> Option<&f32> {
        self.bytes_total_throughput.as_ref()
    }

    /// Sets the value of CurrentIndex
    pub fn set_current_index(&mut self, value: u16) {
        self.current_index = Some(value);
    }

    /// Gets the value of CurrentIndex
    pub fn get_current_index(&self) -> Option<&u16> {
        self.current_index.as_ref()
    }

    /// Sets the value of DNSName
    pub fn set_dnsname(&mut self, value: String) {
        self.dnsname = Some(value);
    }

    /// Gets the value of DNSName
    pub fn get_dnsname(&self) -> Option<&String> {
        self.dnsname.as_ref()
    }

    /// Sets the value of InterfaceDescription
    pub fn set_interface_description(&mut self, value: String) {
        self.interface_description = Some(value);
    }

    /// Gets the value of InterfaceDescription
    pub fn get_interface_description(&self) -> Option<&String> {
        self.interface_description.as_ref()
    }

    /// Sets the value of InterfaceGuid
    pub fn set_interface_guid(&mut self, value: String) {
        self.interface_guid = Some(value);
    }

    /// Gets the value of InterfaceGuid
    pub fn get_interface_guid(&self) -> Option<&String> {
        self.interface_guid.as_ref()
    }

    /// Sets the value of IntervalSeconds
    pub fn set_interval_seconds(&mut self, value: u16) {
        self.interval_seconds = Some(value);
    }

    /// Gets the value of IntervalSeconds
    pub fn get_interval_seconds(&self) -> Option<&u16> {
        self.interval_seconds.as_ref()
    }

    /// Sets the value of IPv4Address
    pub fn set_ipv4_address(&mut self, value: String) {
        self.ipv4_address = Some(value);
    }

    /// Gets the value of IPv4Address
    pub fn get_ipv4_address(&self) -> Option<&String> {
        self.ipv4_address.as_ref()
    }

    /// Sets the value of IPv6Address
    pub fn set_ipv6_address(&mut self, value: String) {
        self.ipv6_address = Some(value);
    }

    /// Gets the value of IPv6Address
    pub fn get_ipv6_address(&self) -> Option<&String> {
        self.ipv6_address.as_ref()
    }

    /// Sets the value of LinkSpeed
    pub fn set_link_speed(&mut self, value: u64) {
        self.link_speed = Some(value);
    }

    /// Gets the value of LinkSpeed
    pub fn get_link_speed(&self) -> Option<&u64> {
        self.link_speed.as_ref()
    }

    /// Sets the value of MachineJoinedName
    pub fn set_machine_joined_name(&mut self, value: String) {
        self.machine_joined_name = Some(value);
    }

    /// Gets the value of MachineJoinedName
    pub fn get_machine_joined_name(&self) -> Option<&String> {
        self.machine_joined_name.as_ref()
    }

    /// Sets the value of MachineJoinedType
    pub fn set_machine_joined_type(&mut self, value: u16) {
        self.machine_joined_type = Some(value);
    }

    /// Gets the value of MachineJoinedType
    pub fn get_machine_joined_type(&self) -> Option<&u16> {
        self.machine_joined_type.as_ref()
    }

    /// Sets the value of MaxReceivedBitsPerSecond
    pub fn set_max_received_bits_per_second(&mut self, value: Vec<f32>) {
        self.max_received_bits_per_second = value;
    }

    /// Gets the value of MaxReceivedBitsPerSecond
    pub fn get_max_received_bits_per_second(&self) -> &Vec<f32> {
        &self.max_received_bits_per_second
    }

    /// Sets the value of MaxSentBitsPerSecond
    pub fn set_max_sent_bits_per_second(&mut self, value: Vec<f32>) {
        self.max_sent_bits_per_second = value;
    }

    /// Gets the value of MaxSentBitsPerSecond
    pub fn get_max_sent_bits_per_second(&self) -> &Vec<f32> {
        &self.max_sent_bits_per_second
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of NdisMedium
    pub fn set_ndis_medium(&mut self, value: u32) {
        self.ndis_medium = Some(value);
    }

    /// Gets the value of NdisMedium
    pub fn get_ndis_medium(&self) -> Option<&u32> {
        self.ndis_medium.as_ref()
    }

    /// Sets the value of NdisPhysicalMedium
    pub fn set_ndis_physical_medium(&mut self, value: u32) {
        self.ndis_physical_medium = Some(value);
    }

    /// Gets the value of NdisPhysicalMedium
    pub fn get_ndis_physical_medium(&self) -> Option<&u32> {
        self.ndis_physical_medium.as_ref()
    }

    /// Sets the value of NonUniCastsReceived
    pub fn set_non_uni_casts_received(&mut self, value: u64) {
        self.non_uni_casts_received = Some(value);
    }

    /// Gets the value of NonUniCastsReceived
    pub fn get_non_uni_casts_received(&self) -> Option<&u64> {
        self.non_uni_casts_received.as_ref()
    }

    /// Sets the value of NonUniCastsReceivedPerInterval
    pub fn set_non_uni_casts_received_per_interval(&mut self, value: u64) {
        self.non_uni_casts_received_per_interval = Some(value);
    }

    /// Gets the value of NonUniCastsReceivedPerInterval
    pub fn get_non_uni_casts_received_per_interval(&self) -> Option<&u64> {
        self.non_uni_casts_received_per_interval.as_ref()
    }

    /// Sets the value of NonUniCastsSent
    pub fn set_non_uni_casts_sent(&mut self, value: u64) {
        self.non_uni_casts_sent = Some(value);
    }

    /// Gets the value of NonUniCastsSent
    pub fn get_non_uni_casts_sent(&self) -> Option<&u64> {
        self.non_uni_casts_sent.as_ref()
    }

    /// Sets the value of NonUniCastsSentPerInterval
    pub fn set_non_uni_casts_sent_per_interval(&mut self, value: u64) {
        self.non_uni_casts_sent_per_interval = Some(value);
    }

    /// Gets the value of NonUniCastsSentPerInterval
    pub fn get_non_uni_casts_sent_per_interval(&self) -> Option<&u64> {
        self.non_uni_casts_sent_per_interval.as_ref()
    }

    /// Sets the value of NonUniCastsTotal
    pub fn set_non_uni_casts_total(&mut self, value: u64) {
        self.non_uni_casts_total = Some(value);
    }

    /// Gets the value of NonUniCastsTotal
    pub fn get_non_uni_casts_total(&self) -> Option<&u64> {
        self.non_uni_casts_total.as_ref()
    }

    /// Sets the value of NonUniCastsTotalPerInterval
    pub fn set_non_uni_casts_total_per_interval(&mut self, value: u64) {
        self.non_uni_casts_total_per_interval = Some(value);
    }

    /// Gets the value of NonUniCastsTotalPerInterval
    pub fn get_non_uni_casts_total_per_interval(&self) -> Option<&u64> {
        self.non_uni_casts_total_per_interval.as_ref()
    }

    /// Sets the value of OperationStatus
    pub fn set_operation_status(&mut self, value: u16) {
        self.operation_status = Some(value);
    }

    /// Gets the value of OperationStatus
    pub fn get_operation_status(&self) -> Option<&u16> {
        self.operation_status.as_ref()
    }

    /// Sets the value of ReceivedBitsPerSecond
    pub fn set_received_bits_per_second(&mut self, value: Vec<f32>) {
        self.received_bits_per_second = value;
    }

    /// Gets the value of ReceivedBitsPerSecond
    pub fn get_received_bits_per_second(&self) -> &Vec<f32> {
        &self.received_bits_per_second
    }

    /// Sets the value of ReceivedThroughput
    pub fn set_received_throughput(&mut self, value: Vec<f32>) {
        self.received_throughput = value;
    }

    /// Gets the value of ReceivedThroughput
    pub fn get_received_throughput(&self) -> &Vec<f32> {
        &self.received_throughput
    }

    /// Sets the value of SentBitsPerSecond
    pub fn set_sent_bits_per_second(&mut self, value: Vec<f32>) {
        self.sent_bits_per_second = value;
    }

    /// Gets the value of SentBitsPerSecond
    pub fn get_sent_bits_per_second(&self) -> &Vec<f32> {
        &self.sent_bits_per_second
    }

    /// Sets the value of SentThroughput
    pub fn set_sent_throughput(&mut self, value: Vec<f32>) {
        self.sent_throughput = value;
    }

    /// Gets the value of SentThroughput
    pub fn get_sent_throughput(&self) -> &Vec<f32> {
        &self.sent_throughput
    }

    /// Sets the value of UniCastsReceived
    pub fn set_uni_casts_received(&mut self, value: u64) {
        self.uni_casts_received = Some(value);
    }

    /// Gets the value of UniCastsReceived
    pub fn get_uni_casts_received(&self) -> Option<&u64> {
        self.uni_casts_received.as_ref()
    }

    /// Sets the value of UniCastsReceivedPerInterval
    pub fn set_uni_casts_received_per_interval(&mut self, value: u64) {
        self.uni_casts_received_per_interval = Some(value);
    }

    /// Gets the value of UniCastsReceivedPerInterval
    pub fn get_uni_casts_received_per_interval(&self) -> Option<&u64> {
        self.uni_casts_received_per_interval.as_ref()
    }

    /// Sets the value of UniCastsSent
    pub fn set_uni_casts_sent(&mut self, value: u64) {
        self.uni_casts_sent = Some(value);
    }

    /// Gets the value of UniCastsSent
    pub fn get_uni_casts_sent(&self) -> Option<&u64> {
        self.uni_casts_sent.as_ref()
    }

    /// Sets the value of UniCastsSentPerInterval
    pub fn set_uni_casts_sent_per_interval(&mut self, value: u64) {
        self.uni_casts_sent_per_interval = Some(value);
    }

    /// Gets the value of UniCastsSentPerInterval
    pub fn get_uni_casts_sent_per_interval(&self) -> Option<&u64> {
        self.uni_casts_sent_per_interval.as_ref()
    }

    /// Sets the value of UniCastsTotal
    pub fn set_uni_casts_total(&mut self, value: u64) {
        self.uni_casts_total = Some(value);
    }

    /// Gets the value of UniCastsTotal
    pub fn get_uni_casts_total(&self) -> Option<&u64> {
        self.uni_casts_total.as_ref()
    }

    /// Sets the value of UniCastsTotalPerInterval
    pub fn set_uni_casts_total_per_interval(&mut self, value: u64) {
        self.uni_casts_total_per_interval = Some(value);
    }

    /// Gets the value of UniCastsTotalPerInterval
    pub fn get_uni_casts_total_per_interval(&self) -> Option<&u64> {
        self.uni_casts_total_per_interval.as_ref()
    }

    /// Sets the value of Utilization
    pub fn set_utilization(&mut self, value: f32) {
        self.utilization = Some(value);
    }

    /// Gets the value of Utilization
    pub fn get_utilization(&self) -> Option<&f32> {
        self.utilization.as_ref()
    }
}

