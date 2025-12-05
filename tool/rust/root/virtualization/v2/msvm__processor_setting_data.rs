// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_ProcessorSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_ProcessorSettingData {
    #[serde(flatten)]
    pub base: CIM_ResourceAllocationSettingData,

/// 
    #[serde(rename = "AllowACountMCount")]
    pub allow_acount_mcount: Option<bool>,

/// 
    #[serde(rename = "ApicMode")]
    pub apic_mode: Option<u8>,

/// 
    #[serde(rename = "CpuBrandString")]
    pub cpu_brand_string: Option<String>,

/// 
    #[serde(rename = "CpuGroupId")]
    pub cpu_group_id: Option<String>,

/// 
    #[serde(rename = "DisableSpeculationControls")]
    pub disable_speculation_controls: Option<bool>,

/// 
    #[serde(rename = "EnableHierarchicalVirtualization")]
    pub enable_hierarchical_virtualization: Option<bool>,

/// 
    #[serde(rename = "EnableHostResourceProtection")]
    pub enable_host_resource_protection: Option<bool>,

/// 
    #[serde(rename = "EnableLegacyApicMode")]
    pub enable_legacy_apic_mode: Option<bool>,

/// 
    #[serde(rename = "EnablePageShattering")]
    pub enable_page_shattering: Option<u8>,

/// 
    #[serde(rename = "EnablePerfmonArchPmu")]
    pub enable_perfmon_arch_pmu: Option<bool>,

/// 
    #[serde(rename = "EnablePerfmonIpt")]
    pub enable_perfmon_ipt: Option<bool>,

/// 
    #[serde(rename = "EnablePerfmonLbr")]
    pub enable_perfmon_lbr: Option<bool>,

/// 
    #[serde(rename = "EnablePerfmonPebs")]
    pub enable_perfmon_pebs: Option<bool>,

/// 
    #[serde(rename = "EnablePerfmonPmu")]
    pub enable_perfmon_pmu: Option<bool>,

/// 
    #[serde(rename = "EnableSocketTopology")]
    pub enable_socket_topology: Option<bool>,

/// 
    #[serde(rename = "EnlightenmentSet")]
    pub enlightenment_set: Option<String>,

/// 
    #[serde(rename = "ExposeVirtualizationExtensions")]
    pub expose_virtualization_extensions: Option<bool>,

/// 
    #[serde(rename = "ExtendedVirtualizationExtensions")]
    pub extended_virtualization_extensions: Option<ProcessorSettingData_ExtendedVirtualizationExtensions>,

/// 
    #[serde(rename = "HideHypervisorPresent")]
    pub hide_hypervisor_present: Option<bool>,

/// 
    #[serde(rename = "HwThreadsPerCore")]
    pub hw_threads_per_core: Option<u64>,

/// 
    #[serde(rename = "L3CacheWays")]
    pub l3_cache_ways: Option<u32>,

/// 
    #[serde(rename = "L3ProcessorDistributionPolicy")]
    pub l3_processor_distribution_policy: Option<ProcessorSettingData_L3ProcessorDistributionPolicy>,

/// 
    #[serde(rename = "LimitCPUID")]
    pub limit_cpuid: Option<bool>,

/// 
    #[serde(rename = "LimitProcessorFeatures")]
    pub limit_processor_features: Option<bool>,

/// 
    #[serde(rename = "LimitProcessorFeaturesMode")]
    pub limit_processor_features_mode: Option<ProcessorSettingData_LimitProcessorFeaturesMode>,

/// 
    #[serde(rename = "MaxClusterCountPerSocket")]
    pub max_cluster_count_per_socket: Option<u32>,

/// 
    #[serde(rename = "MaxHierarchicalPartitions")]
    pub max_hierarchical_partitions: Option<u32>,

/// 
    #[serde(rename = "MaxHierarchicalVps")]
    pub max_hierarchical_vps: Option<u32>,

/// 
    #[serde(rename = "MaxHwIsolatedGuests")]
    pub max_hw_isolated_guests: Option<u32>,

/// 
    #[serde(rename = "MaxNumaNodesPerSocket")]
    pub max_numa_nodes_per_socket: Option<u64>,

/// 
    #[serde(rename = "MaxProcessorCountPerL3")]
    pub max_processor_count_per_l3: Option<u32>,

/// 
    #[serde(rename = "MaxProcessorsPerNumaNode")]
    pub max_processors_per_numa_node: Option<u64>,

/// 
    #[serde(rename = "PartitionDiagnosticBufferCount")]
    pub partition_diagnostic_buffer_count: Option<u32>,

/// 
    #[serde(rename = "PartitionDiagnosticBufferSizeInPages")]
    pub partition_diagnostic_buffer_size_in_pages: Option<u32>,

/// 
    #[serde(rename = "PerfCpuFreqCapMhz")]
    pub perf_cpu_freq_cap_mhz: Option<u32>,

/// 
    #[serde(rename = "PhysicalAddressWidth")]
    pub physical_address_width: Option<u32>,

/// 
    #[serde(rename = "ProcessorFeatureSet")]
    pub processor_feature_set: Option<String>,
}

impl Msvm_ProcessorSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ResourceAllocationSettingData::new(),
            allow_acount_mcount: None,
            apic_mode: None,
            cpu_brand_string: None,
            cpu_group_id: None,
            disable_speculation_controls: None,
            enable_hierarchical_virtualization: None,
            enable_host_resource_protection: None,
            enable_legacy_apic_mode: None,
            enable_page_shattering: None,
            enable_perfmon_arch_pmu: None,
            enable_perfmon_ipt: None,
            enable_perfmon_lbr: None,
            enable_perfmon_pebs: None,
            enable_perfmon_pmu: None,
            enable_socket_topology: None,
            enlightenment_set: None,
            expose_virtualization_extensions: None,
            extended_virtualization_extensions: None,
            hide_hypervisor_present: None,
            hw_threads_per_core: None,
            l3_cache_ways: None,
            l3_processor_distribution_policy: None,
            limit_cpuid: None,
            limit_processor_features: None,
            limit_processor_features_mode: None,
            max_cluster_count_per_socket: None,
            max_hierarchical_partitions: None,
            max_hierarchical_vps: None,
            max_hw_isolated_guests: None,
            max_numa_nodes_per_socket: None,
            max_processor_count_per_l3: None,
            max_processors_per_numa_node: None,
            partition_diagnostic_buffer_count: None,
            partition_diagnostic_buffer_size_in_pages: None,
            perf_cpu_freq_cap_mhz: None,
            physical_address_width: None,
            processor_feature_set: None,
        }
    }


    /// Sets the value of AllowACountMCount
    pub fn set_allow_acount_mcount(&mut self, value: bool) {
        self.allow_acount_mcount = Some(value);
    }

    /// Gets the value of AllowACountMCount
    pub fn get_allow_acount_mcount(&self) -> Option<&bool> {
        self.allow_acount_mcount.as_ref()
    }

    /// Sets the value of ApicMode
    pub fn set_apic_mode(&mut self, value: u8) {
        self.apic_mode = Some(value);
    }

    /// Gets the value of ApicMode
    pub fn get_apic_mode(&self) -> Option<&u8> {
        self.apic_mode.as_ref()
    }

    /// Sets the value of CpuBrandString
    pub fn set_cpu_brand_string(&mut self, value: String) {
        self.cpu_brand_string = Some(value);
    }

    /// Gets the value of CpuBrandString
    pub fn get_cpu_brand_string(&self) -> Option<&String> {
        self.cpu_brand_string.as_ref()
    }

    /// Sets the value of CpuGroupId
    pub fn set_cpu_group_id(&mut self, value: String) {
        self.cpu_group_id = Some(value);
    }

    /// Gets the value of CpuGroupId
    pub fn get_cpu_group_id(&self) -> Option<&String> {
        self.cpu_group_id.as_ref()
    }

    /// Sets the value of DisableSpeculationControls
    pub fn set_disable_speculation_controls(&mut self, value: bool) {
        self.disable_speculation_controls = Some(value);
    }

    /// Gets the value of DisableSpeculationControls
    pub fn get_disable_speculation_controls(&self) -> Option<&bool> {
        self.disable_speculation_controls.as_ref()
    }

    /// Sets the value of EnableHierarchicalVirtualization
    pub fn set_enable_hierarchical_virtualization(&mut self, value: bool) {
        self.enable_hierarchical_virtualization = Some(value);
    }

    /// Gets the value of EnableHierarchicalVirtualization
    pub fn get_enable_hierarchical_virtualization(&self) -> Option<&bool> {
        self.enable_hierarchical_virtualization.as_ref()
    }

    /// Sets the value of EnableHostResourceProtection
    pub fn set_enable_host_resource_protection(&mut self, value: bool) {
        self.enable_host_resource_protection = Some(value);
    }

    /// Gets the value of EnableHostResourceProtection
    pub fn get_enable_host_resource_protection(&self) -> Option<&bool> {
        self.enable_host_resource_protection.as_ref()
    }

    /// Sets the value of EnableLegacyApicMode
    pub fn set_enable_legacy_apic_mode(&mut self, value: bool) {
        self.enable_legacy_apic_mode = Some(value);
    }

    /// Gets the value of EnableLegacyApicMode
    pub fn get_enable_legacy_apic_mode(&self) -> Option<&bool> {
        self.enable_legacy_apic_mode.as_ref()
    }

    /// Sets the value of EnablePageShattering
    pub fn set_enable_page_shattering(&mut self, value: u8) {
        self.enable_page_shattering = Some(value);
    }

    /// Gets the value of EnablePageShattering
    pub fn get_enable_page_shattering(&self) -> Option<&u8> {
        self.enable_page_shattering.as_ref()
    }

    /// Sets the value of EnablePerfmonArchPmu
    pub fn set_enable_perfmon_arch_pmu(&mut self, value: bool) {
        self.enable_perfmon_arch_pmu = Some(value);
    }

    /// Gets the value of EnablePerfmonArchPmu
    pub fn get_enable_perfmon_arch_pmu(&self) -> Option<&bool> {
        self.enable_perfmon_arch_pmu.as_ref()
    }

    /// Sets the value of EnablePerfmonIpt
    pub fn set_enable_perfmon_ipt(&mut self, value: bool) {
        self.enable_perfmon_ipt = Some(value);
    }

    /// Gets the value of EnablePerfmonIpt
    pub fn get_enable_perfmon_ipt(&self) -> Option<&bool> {
        self.enable_perfmon_ipt.as_ref()
    }

    /// Sets the value of EnablePerfmonLbr
    pub fn set_enable_perfmon_lbr(&mut self, value: bool) {
        self.enable_perfmon_lbr = Some(value);
    }

    /// Gets the value of EnablePerfmonLbr
    pub fn get_enable_perfmon_lbr(&self) -> Option<&bool> {
        self.enable_perfmon_lbr.as_ref()
    }

    /// Sets the value of EnablePerfmonPebs
    pub fn set_enable_perfmon_pebs(&mut self, value: bool) {
        self.enable_perfmon_pebs = Some(value);
    }

    /// Gets the value of EnablePerfmonPebs
    pub fn get_enable_perfmon_pebs(&self) -> Option<&bool> {
        self.enable_perfmon_pebs.as_ref()
    }

    /// Sets the value of EnablePerfmonPmu
    pub fn set_enable_perfmon_pmu(&mut self, value: bool) {
        self.enable_perfmon_pmu = Some(value);
    }

    /// Gets the value of EnablePerfmonPmu
    pub fn get_enable_perfmon_pmu(&self) -> Option<&bool> {
        self.enable_perfmon_pmu.as_ref()
    }

    /// Sets the value of EnableSocketTopology
    pub fn set_enable_socket_topology(&mut self, value: bool) {
        self.enable_socket_topology = Some(value);
    }

    /// Gets the value of EnableSocketTopology
    pub fn get_enable_socket_topology(&self) -> Option<&bool> {
        self.enable_socket_topology.as_ref()
    }

    /// Sets the value of EnlightenmentSet
    pub fn set_enlightenment_set(&mut self, value: String) {
        self.enlightenment_set = Some(value);
    }

    /// Gets the value of EnlightenmentSet
    pub fn get_enlightenment_set(&self) -> Option<&String> {
        self.enlightenment_set.as_ref()
    }

    /// Sets the value of ExposeVirtualizationExtensions
    pub fn set_expose_virtualization_extensions(&mut self, value: bool) {
        self.expose_virtualization_extensions = Some(value);
    }

    /// Gets the value of ExposeVirtualizationExtensions
    pub fn get_expose_virtualization_extensions(&self) -> Option<&bool> {
        self.expose_virtualization_extensions.as_ref()
    }

    /// Sets the value of ExtendedVirtualizationExtensions
    pub fn set_extended_virtualization_extensions(&mut self, value: ProcessorSettingData_ExtendedVirtualizationExtensions) {
        self.extended_virtualization_extensions = Some(value);
    }

    /// Gets the value of ExtendedVirtualizationExtensions
    pub fn get_extended_virtualization_extensions(&self) -> Option<&ProcessorSettingData_ExtendedVirtualizationExtensions> {
        self.extended_virtualization_extensions.as_ref()
    }

    /// Sets the value of HideHypervisorPresent
    pub fn set_hide_hypervisor_present(&mut self, value: bool) {
        self.hide_hypervisor_present = Some(value);
    }

    /// Gets the value of HideHypervisorPresent
    pub fn get_hide_hypervisor_present(&self) -> Option<&bool> {
        self.hide_hypervisor_present.as_ref()
    }

    /// Sets the value of HwThreadsPerCore
    pub fn set_hw_threads_per_core(&mut self, value: u64) {
        self.hw_threads_per_core = Some(value);
    }

    /// Gets the value of HwThreadsPerCore
    pub fn get_hw_threads_per_core(&self) -> Option<&u64> {
        self.hw_threads_per_core.as_ref()
    }

    /// Sets the value of L3CacheWays
    pub fn set_l3_cache_ways(&mut self, value: u32) {
        self.l3_cache_ways = Some(value);
    }

    /// Gets the value of L3CacheWays
    pub fn get_l3_cache_ways(&self) -> Option<&u32> {
        self.l3_cache_ways.as_ref()
    }

    /// Sets the value of L3ProcessorDistributionPolicy
    pub fn set_l3_processor_distribution_policy(&mut self, value: ProcessorSettingData_L3ProcessorDistributionPolicy) {
        self.l3_processor_distribution_policy = Some(value);
    }

    /// Gets the value of L3ProcessorDistributionPolicy
    pub fn get_l3_processor_distribution_policy(&self) -> Option<&ProcessorSettingData_L3ProcessorDistributionPolicy> {
        self.l3_processor_distribution_policy.as_ref()
    }

    /// Sets the value of LimitCPUID
    pub fn set_limit_cpuid(&mut self, value: bool) {
        self.limit_cpuid = Some(value);
    }

    /// Gets the value of LimitCPUID
    pub fn get_limit_cpuid(&self) -> Option<&bool> {
        self.limit_cpuid.as_ref()
    }

    /// Sets the value of LimitProcessorFeatures
    pub fn set_limit_processor_features(&mut self, value: bool) {
        self.limit_processor_features = Some(value);
    }

    /// Gets the value of LimitProcessorFeatures
    pub fn get_limit_processor_features(&self) -> Option<&bool> {
        self.limit_processor_features.as_ref()
    }

    /// Sets the value of LimitProcessorFeaturesMode
    pub fn set_limit_processor_features_mode(&mut self, value: ProcessorSettingData_LimitProcessorFeaturesMode) {
        self.limit_processor_features_mode = Some(value);
    }

    /// Gets the value of LimitProcessorFeaturesMode
    pub fn get_limit_processor_features_mode(&self) -> Option<&ProcessorSettingData_LimitProcessorFeaturesMode> {
        self.limit_processor_features_mode.as_ref()
    }

    /// Sets the value of MaxClusterCountPerSocket
    pub fn set_max_cluster_count_per_socket(&mut self, value: u32) {
        self.max_cluster_count_per_socket = Some(value);
    }

    /// Gets the value of MaxClusterCountPerSocket
    pub fn get_max_cluster_count_per_socket(&self) -> Option<&u32> {
        self.max_cluster_count_per_socket.as_ref()
    }

    /// Sets the value of MaxHierarchicalPartitions
    pub fn set_max_hierarchical_partitions(&mut self, value: u32) {
        self.max_hierarchical_partitions = Some(value);
    }

    /// Gets the value of MaxHierarchicalPartitions
    pub fn get_max_hierarchical_partitions(&self) -> Option<&u32> {
        self.max_hierarchical_partitions.as_ref()
    }

    /// Sets the value of MaxHierarchicalVps
    pub fn set_max_hierarchical_vps(&mut self, value: u32) {
        self.max_hierarchical_vps = Some(value);
    }

    /// Gets the value of MaxHierarchicalVps
    pub fn get_max_hierarchical_vps(&self) -> Option<&u32> {
        self.max_hierarchical_vps.as_ref()
    }

    /// Sets the value of MaxHwIsolatedGuests
    pub fn set_max_hw_isolated_guests(&mut self, value: u32) {
        self.max_hw_isolated_guests = Some(value);
    }

    /// Gets the value of MaxHwIsolatedGuests
    pub fn get_max_hw_isolated_guests(&self) -> Option<&u32> {
        self.max_hw_isolated_guests.as_ref()
    }

    /// Sets the value of MaxNumaNodesPerSocket
    pub fn set_max_numa_nodes_per_socket(&mut self, value: u64) {
        self.max_numa_nodes_per_socket = Some(value);
    }

    /// Gets the value of MaxNumaNodesPerSocket
    pub fn get_max_numa_nodes_per_socket(&self) -> Option<&u64> {
        self.max_numa_nodes_per_socket.as_ref()
    }

    /// Sets the value of MaxProcessorCountPerL3
    pub fn set_max_processor_count_per_l3(&mut self, value: u32) {
        self.max_processor_count_per_l3 = Some(value);
    }

    /// Gets the value of MaxProcessorCountPerL3
    pub fn get_max_processor_count_per_l3(&self) -> Option<&u32> {
        self.max_processor_count_per_l3.as_ref()
    }

    /// Sets the value of MaxProcessorsPerNumaNode
    pub fn set_max_processors_per_numa_node(&mut self, value: u64) {
        self.max_processors_per_numa_node = Some(value);
    }

    /// Gets the value of MaxProcessorsPerNumaNode
    pub fn get_max_processors_per_numa_node(&self) -> Option<&u64> {
        self.max_processors_per_numa_node.as_ref()
    }

    /// Sets the value of PartitionDiagnosticBufferCount
    pub fn set_partition_diagnostic_buffer_count(&mut self, value: u32) {
        self.partition_diagnostic_buffer_count = Some(value);
    }

    /// Gets the value of PartitionDiagnosticBufferCount
    pub fn get_partition_diagnostic_buffer_count(&self) -> Option<&u32> {
        self.partition_diagnostic_buffer_count.as_ref()
    }

    /// Sets the value of PartitionDiagnosticBufferSizeInPages
    pub fn set_partition_diagnostic_buffer_size_in_pages(&mut self, value: u32) {
        self.partition_diagnostic_buffer_size_in_pages = Some(value);
    }

    /// Gets the value of PartitionDiagnosticBufferSizeInPages
    pub fn get_partition_diagnostic_buffer_size_in_pages(&self) -> Option<&u32> {
        self.partition_diagnostic_buffer_size_in_pages.as_ref()
    }

    /// Sets the value of PerfCpuFreqCapMhz
    pub fn set_perf_cpu_freq_cap_mhz(&mut self, value: u32) {
        self.perf_cpu_freq_cap_mhz = Some(value);
    }

    /// Gets the value of PerfCpuFreqCapMhz
    pub fn get_perf_cpu_freq_cap_mhz(&self) -> Option<&u32> {
        self.perf_cpu_freq_cap_mhz.as_ref()
    }

    /// Sets the value of PhysicalAddressWidth
    pub fn set_physical_address_width(&mut self, value: u32) {
        self.physical_address_width = Some(value);
    }

    /// Gets the value of PhysicalAddressWidth
    pub fn get_physical_address_width(&self) -> Option<&u32> {
        self.physical_address_width.as_ref()
    }

    /// Sets the value of ProcessorFeatureSet
    pub fn set_processor_feature_set(&mut self, value: String) {
        self.processor_feature_set = Some(value);
    }

    /// Gets the value of ProcessorFeatureSet
    pub fn get_processor_feature_set(&self) -> Option<&String> {
        self.processor_feature_set.as_ref()
    }
}

impl Msvm_ProcessorSettingData {
    /// Gets the related Msvm_AllocationCapabilities object(s)
    pub fn get_related__allocation_capabilities(&self) -> Result<Msvm_AllocationCapabilities, WmiError> {
        self.get_related("Msvm_AllocationCapabilities")
    }

}

