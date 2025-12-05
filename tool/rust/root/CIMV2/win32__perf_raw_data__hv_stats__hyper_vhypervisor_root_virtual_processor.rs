// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_HvStats_HyperVHypervisorRootVirtualProcessor struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_HvStats_HyperVHypervisorRootVirtualProcessor {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "AddressDomainFlushesPersec")]
    pub address_domain_flushes_persec: Option<u64>,

/// 
    #[serde(rename = "AddressSpaceEvictionsPersec")]
    pub address_space_evictions_persec: Option<u64>,

/// 
    #[serde(rename = "AddressSpaceFlushesPersec")]
    pub address_space_flushes_persec: Option<u64>,

/// 
    #[serde(rename = "AddressSpaceSwitchesPersec")]
    pub address_space_switches_persec: Option<u64>,

/// 
    #[serde(rename = "APICEOIAccessesPersec")]
    pub apiceoiaccesses_persec: Option<u64>,

/// 
    #[serde(rename = "APICIPIsSentPersec")]
    pub apicipis_sent_persec: Option<u64>,

/// 
    #[serde(rename = "APICMMIOAccessesPersec")]
    pub apicmmioaccesses_persec: Option<u64>,

/// 
    #[serde(rename = "APICSelfIPIsSentPersec")]
    pub apicself_ipis_sent_persec: Option<u64>,

/// 
    #[serde(rename = "APICTPRAccessesPersec")]
    pub apictpraccesses_persec: Option<u64>,

/// 
    #[serde(rename = "BusLockAcquisitionsPersec")]
    pub bus_lock_acquisitions_persec: Option<u64>,

/// 
    #[serde(rename = "ControlRegisterAccessesCost")]
    pub control_register_accesses_cost: Option<u64>,

/// 
    #[serde(rename = "ControlRegisterAccessesCost_Base")]
    pub control_register_accesses_cost__base: Option<u64>,

/// 
    #[serde(rename = "ControlRegisterAccessesForwardedPersec")]
    pub control_register_accesses_forwarded_persec: Option<u64>,

/// 
    #[serde(rename = "ControlRegisterAccessesForwardingCost")]
    pub control_register_accesses_forwarding_cost: Option<u64>,

/// 
    #[serde(rename = "ControlRegisterAccessesForwardingCost_Base")]
    pub control_register_accesses_forwarding_cost__base: Option<u64>,

/// 
    #[serde(rename = "ControlRegisterAccessesPersec")]
    pub control_register_accesses_persec: Option<u64>,

/// 
    #[serde(rename = "CPPCRequestContextSwitchesPersec")]
    pub cppcrequest_context_switches_persec: Option<u64>,

/// 
    #[serde(rename = "CPUContentionTimePerDispatch")]
    pub cpucontention_time_per_dispatch: Option<u64>,

/// 
    #[serde(rename = "CPUContentionTimePerDispatch_Base")]
    pub cpucontention_time_per_dispatch__base: Option<u64>,

/// 
    #[serde(rename = "CPUGroupHypercallsPersec")]
    pub cpugroup_hypercalls_persec: Option<u64>,

/// 
    #[serde(rename = "CPUIDInstructionsCost")]
    pub cpuidinstructions_cost: Option<u64>,

/// 
    #[serde(rename = "CPUIDInstructionsCost_Base")]
    pub cpuidinstructions_cost__base: Option<u64>,

/// 
    #[serde(rename = "CPUIDInstructionsForwardedPersec")]
    pub cpuidinstructions_forwarded_persec: Option<u64>,

/// 
    #[serde(rename = "CPUIDInstructionsForwardingCost")]
    pub cpuidinstructions_forwarding_cost: Option<u64>,

/// 
    #[serde(rename = "CPUIDInstructionsForwardingCost_Base")]
    pub cpuidinstructions_forwarding_cost__base: Option<u64>,

/// 
    #[serde(rename = "CPUIDInstructionsPersec")]
    pub cpuidinstructions_persec: Option<u64>,

/// 
    #[serde(rename = "CPUWaitTimePerDispatch")]
    pub cpuwait_time_per_dispatch: Option<u64>,

/// 
    #[serde(rename = "CPUWaitTimePerDispatch_Base")]
    pub cpuwait_time_per_dispatch__base: Option<u64>,

/// 
    #[serde(rename = "CPUWakeUpTimePerDispatch")]
    pub cpuwake_up_time_per_dispatch: Option<u64>,

/// 
    #[serde(rename = "CPUWakeUpTimePerDispatch_Base")]
    pub cpuwake_up_time_per_dispatch__base: Option<u64>,

/// 
    #[serde(rename = "DebugRegisterAccessesCost")]
    pub debug_register_accesses_cost: Option<u64>,

/// 
    #[serde(rename = "DebugRegisterAccessesCost_Base")]
    pub debug_register_accesses_cost__base: Option<u64>,

/// 
    #[serde(rename = "DebugRegisterAccessesForwardedPersec")]
    pub debug_register_accesses_forwarded_persec: Option<u64>,

/// 
    #[serde(rename = "DebugRegisterAccessesForwardingCost")]
    pub debug_register_accesses_forwarding_cost: Option<u64>,

/// 
    #[serde(rename = "DebugRegisterAccessesForwardingCost_Base")]
    pub debug_register_accesses_forwarding_cost__base: Option<u64>,

/// 
    #[serde(rename = "DebugRegisterAccessesPersec")]
    pub debug_register_accesses_persec: Option<u64>,

/// 
    #[serde(rename = "DepositHypercallsPersec")]
    pub deposit_hypercalls_persec: Option<u64>,

/// 
    #[serde(rename = "DeviceDomainHypercallsPersec")]
    pub device_domain_hypercalls_persec: Option<u64>,

/// 
    #[serde(rename = "EmulatedInstructionsCost")]
    pub emulated_instructions_cost: Option<u64>,

/// 
    #[serde(rename = "EmulatedInstructionsCost_Base")]
    pub emulated_instructions_cost__base: Option<u64>,

/// 
    #[serde(rename = "EmulatedInstructionsForwardedPersec")]
    pub emulated_instructions_forwarded_persec: Option<u64>,

/// 
    #[serde(rename = "EmulatedInstructionsForwardingCost")]
    pub emulated_instructions_forwarding_cost: Option<u64>,

/// 
    #[serde(rename = "EmulatedInstructionsForwardingCost_Base")]
    pub emulated_instructions_forwarding_cost__base: Option<u64>,

/// 
    #[serde(rename = "EmulatedInstructionsPersec")]
    pub emulated_instructions_persec: Option<u64>,

/// 
    #[serde(rename = "EventLogHypercallsPersec")]
    pub event_log_hypercalls_persec: Option<u64>,

/// 
    #[serde(rename = "ExtendedHypercallInterceptMessagesPersec")]
    pub extended_hypercall_intercept_messages_persec: Option<u64>,

/// 
    #[serde(rename = "ExtendedHypercallsPersec")]
    pub extended_hypercalls_persec: Option<u64>,

/// 
    #[serde(rename = "ExternalInterruptsCost")]
    pub external_interrupts_cost: Option<u64>,

/// 
    #[serde(rename = "ExternalInterruptsCost_Base")]
    pub external_interrupts_cost__base: Option<u64>,

/// 
    #[serde(rename = "ExternalInterruptsForwardedPersec")]
    pub external_interrupts_forwarded_persec: Option<u64>,

/// 
    #[serde(rename = "ExternalInterruptsPersec")]
    pub external_interrupts_persec: Option<u64>,

/// 
    #[serde(rename = "FlushPhysicalAddressListHypercallsPersec")]
    pub flush_physical_address_list_hypercalls_persec: Option<u64>,

/// 
    #[serde(rename = "FlushPhysicalAddressSpaceHypercallsPersec")]
    pub flush_physical_address_space_hypercalls_persec: Option<u64>,

/// 
    #[serde(rename = "GIFInstructionEmulationCost")]
    pub gifinstruction_emulation_cost: Option<u64>,

/// 
    #[serde(rename = "GIFInstructionEmulationCost_Base")]
    pub gifinstruction_emulation_cost__base: Option<u64>,

/// 
    #[serde(rename = "GIFInstructionEmulationInterceptsPersec")]
    pub gifinstruction_emulation_intercepts_persec: Option<u64>,

/// 
    #[serde(rename = "GlobalGVARangeFlushesPersec")]
    pub global_gvarange_flushes_persec: Option<u64>,

/// 
    #[serde(rename = "GlobalIOTLBFlushCost")]
    pub global_iotlbflush_cost: Option<u64>,

/// 
    #[serde(rename = "GlobalIOTLBFlushCost_Base")]
    pub global_iotlbflush_cost__base: Option<u64>,

/// 
    #[serde(rename = "GlobalIOTLBFlushesPersec")]
    pub global_iotlbflushes_persec: Option<u64>,

/// 
    #[serde(rename = "GPASpaceHypercallsPersec")]
    pub gpaspace_hypercalls_persec: Option<u64>,

/// 
    #[serde(rename = "GuestPageTableMapsPersec")]
    pub guest_page_table_maps_persec: Option<u64>,

/// 
    #[serde(rename = "HardwareInterruptsPersec")]
    pub hardware_interrupts_persec: Option<u64>,

/// 
    #[serde(rename = "HLTInstructionsCost")]
    pub hltinstructions_cost: Option<u64>,

/// 
    #[serde(rename = "HLTInstructionsCost_Base")]
    pub hltinstructions_cost__base: Option<u64>,

/// 
    #[serde(rename = "HLTInstructionsForwardedPersec")]
    pub hltinstructions_forwarded_persec: Option<u64>,

/// 
    #[serde(rename = "HLTInstructionsForwardingCost")]
    pub hltinstructions_forwarding_cost: Option<u64>,

/// 
    #[serde(rename = "HLTInstructionsForwardingCost_Base")]
    pub hltinstructions_forwarding_cost__base: Option<u64>,

/// 
    #[serde(rename = "HLTInstructionsPersec")]
    pub hltinstructions_persec: Option<u64>,

/// 
    #[serde(rename = "HypercallsCost")]
    pub hypercalls_cost: Option<u64>,

/// 
    #[serde(rename = "HypercallsCost_Base")]
    pub hypercalls_cost__base: Option<u64>,

/// 
    #[serde(rename = "HypercallsForwardedPersec")]
    pub hypercalls_forwarded_persec: Option<u64>,

/// 
    #[serde(rename = "HypercallsForwardingCost")]
    pub hypercalls_forwarding_cost: Option<u64>,

/// 
    #[serde(rename = "HypercallsForwardingCost_Base")]
    pub hypercalls_forwarding_cost__base: Option<u64>,

/// 
    #[serde(rename = "HypercallsPersec")]
    pub hypercalls_persec: Option<u64>,

/// 
    #[serde(rename = "InvEptAllContextEmulationInterceptsPersec")]
    pub inv_ept_all_context_emulation_intercepts_persec: Option<u64>,

/// 
    #[serde(rename = "InvEptAllContextInstructionEmulationCost")]
    pub inv_ept_all_context_instruction_emulation_cost: Option<u64>,

/// 
    #[serde(rename = "InvEptAllContextInstructionEmulationCost_Base")]
    pub inv_ept_all_context_instruction_emulation_cost__base: Option<u64>,

/// 
    #[serde(rename = "InvEptSingleContextEmulationInterceptsPersec")]
    pub inv_ept_single_context_emulation_intercepts_persec: Option<u64>,

/// 
    #[serde(rename = "InvEptSingleContextInstructionEmulationCost")]
    pub inv_ept_single_context_instruction_emulation_cost: Option<u64>,

/// 
    #[serde(rename = "InvEptSingleContextInstructionEmulationCost_Base")]
    pub inv_ept_single_context_instruction_emulation_cost__base: Option<u64>,

/// 
    #[serde(rename = "InvVpidAllContextEmulationInterceptsPersec")]
    pub inv_vpid_all_context_emulation_intercepts_persec: Option<u64>,

/// 
    #[serde(rename = "InvVpidAllContextInstructionEmulationCost")]
    pub inv_vpid_all_context_instruction_emulation_cost: Option<u64>,

/// 
    #[serde(rename = "InvVpidAllContextInstructionEmulationCost_Base")]
    pub inv_vpid_all_context_instruction_emulation_cost__base: Option<u64>,

/// 
    #[serde(rename = "InvVpidSingleAddressEmulationInterceptsPersec")]
    pub inv_vpid_single_address_emulation_intercepts_persec: Option<u64>,

/// 
    #[serde(rename = "InvVpidSingleAddressInstructionEmulationCost")]
    pub inv_vpid_single_address_instruction_emulation_cost: Option<u64>,

/// 
    #[serde(rename = "InvVpidSingleAddressInstructionEmulationCost_Base")]
    pub inv_vpid_single_address_instruction_emulation_cost__base: Option<u64>,

/// 
    #[serde(rename = "InvVpidSingleContextEmulationInterceptsPersec")]
    pub inv_vpid_single_context_emulation_intercepts_persec: Option<u64>,

/// 
    #[serde(rename = "InvVpidSingleContextInstructionEmulationCost")]
    pub inv_vpid_single_context_instruction_emulation_cost: Option<u64>,

/// 
    #[serde(rename = "InvVpidSingleContextInstructionEmulationCost_Base")]
    pub inv_vpid_single_context_instruction_emulation_cost__base: Option<u64>,

/// 
    #[serde(rename = "IOInstructionsCost")]
    pub ioinstructions_cost: Option<u64>,

/// 
    #[serde(rename = "IOInstructionsCost_Base")]
    pub ioinstructions_cost__base: Option<u64>,

/// 
    #[serde(rename = "IOInstructionsForwardedPersec")]
    pub ioinstructions_forwarded_persec: Option<u64>,

/// 
    #[serde(rename = "IOInstructionsForwardingCost")]
    pub ioinstructions_forwarding_cost: Option<u64>,

/// 
    #[serde(rename = "IOInstructionsForwardingCost_Base")]
    pub ioinstructions_forwarding_cost__base: Option<u64>,

/// 
    #[serde(rename = "IOInstructionsPersec")]
    pub ioinstructions_persec: Option<u64>,

/// 
    #[serde(rename = "IOInterceptMessagesPersec")]
    pub iointercept_messages_persec: Option<u64>,

/// 
    #[serde(rename = "IOMMUHypercallsPersec")]
    pub iommuhypercalls_persec: Option<u64>,

/// 
    #[serde(rename = "LargePageTLBFillsPersec")]
    pub large_page_tlbfills_persec: Option<u64>,

/// 
    #[serde(rename = "LocalFlushedGVARangesPersec")]
    pub local_flushed_gvaranges_persec: Option<u64>,

/// 
    #[serde(rename = "LocalIOTLBFlushCost")]
    pub local_iotlbflush_cost: Option<u64>,

/// 
    #[serde(rename = "LocalIOTLBFlushCost_Base")]
    pub local_iotlbflush_cost__base: Option<u64>,

/// 
    #[serde(rename = "LocalIOTLBFlushesPersec")]
    pub local_iotlbflushes_persec: Option<u64>,

/// 
    #[serde(rename = "LogicalProcessorDispatchesPersec")]
    pub logical_processor_dispatches_persec: Option<u64>,

/// 
    #[serde(rename = "LogicalProcessorHypercallsPersec")]
    pub logical_processor_hypercalls_persec: Option<u64>,

/// 
    #[serde(rename = "LogicalProcessorMigrationsPersec")]
    pub logical_processor_migrations_persec: Option<u64>,

/// 
    #[serde(rename = "LongSpinWaitHypercallsPersec")]
    pub long_spin_wait_hypercalls_persec: Option<u64>,

/// 
    #[serde(rename = "MBECNestedPageTableSwitchesPersec")]
    pub mbecnested_page_table_switches_persec: Option<u64>,

/// 
    #[serde(rename = "MemoryInterceptMessagesPersec")]
    pub memory_intercept_messages_persec: Option<u64>,

/// 
    #[serde(rename = "MSRAccessesCost")]
    pub msraccesses_cost: Option<u64>,

/// 
    #[serde(rename = "MSRAccessesCost_Base")]
    pub msraccesses_cost__base: Option<u64>,

/// 
    #[serde(rename = "MSRAccessesForwardedPersec")]
    pub msraccesses_forwarded_persec: Option<u64>,

/// 
    #[serde(rename = "MSRAccessesForwardingCost")]
    pub msraccesses_forwarding_cost: Option<u64>,

/// 
    #[serde(rename = "MSRAccessesForwardingCost_Base")]
    pub msraccesses_forwarding_cost__base: Option<u64>,

/// 
    #[serde(rename = "MSRAccessesPersec")]
    pub msraccesses_persec: Option<u64>,

/// 
    #[serde(rename = "MWAITInstructionsCost")]
    pub mwaitinstructions_cost: Option<u64>,

/// 
    #[serde(rename = "MWAITInstructionsCost_Base")]
    pub mwaitinstructions_cost__base: Option<u64>,

/// 
    #[serde(rename = "MWAITInstructionsForwardedPersec")]
    pub mwaitinstructions_forwarded_persec: Option<u64>,

/// 
    #[serde(rename = "MWAITInstructionsForwardingCost")]
    pub mwaitinstructions_forwarding_cost: Option<u64>,

/// 
    #[serde(rename = "MWAITInstructionsForwardingCost_Base")]
    pub mwaitinstructions_forwarding_cost__base: Option<u64>,

/// 
    #[serde(rename = "MWAITInstructionsPersec")]
    pub mwaitinstructions_persec: Option<u64>,

/// 
    #[serde(rename = "NestedPageFaultInterceptsCost")]
    pub nested_page_fault_intercepts_cost: Option<u64>,

/// 
    #[serde(rename = "NestedPageFaultInterceptsCost_Base")]
    pub nested_page_fault_intercepts_cost__base: Option<u64>,

/// 
    #[serde(rename = "NestedPageFaultInterceptsPersec")]
    pub nested_page_fault_intercepts_persec: Option<u64>,

/// 
    #[serde(rename = "NestedSLATHardPageFaultsCost")]
    pub nested_slathard_page_faults_cost: Option<u64>,

/// 
    #[serde(rename = "NestedSLATHardPageFaultsCost_Base")]
    pub nested_slathard_page_faults_cost__base: Option<u64>,

/// 
    #[serde(rename = "NestedSLATHardPageFaultsPersec")]
    pub nested_slathard_page_faults_persec: Option<u64>,

/// 
    #[serde(rename = "NestedSLATSoftPageFaultsCost")]
    pub nested_slatsoft_page_faults_cost: Option<u64>,

/// 
    #[serde(rename = "NestedSLATSoftPageFaultsCost_Base")]
    pub nested_slatsoft_page_faults_cost__base: Option<u64>,

/// 
    #[serde(rename = "NestedSLATSoftPageFaultsPersec")]
    pub nested_slatsoft_page_faults_persec: Option<u64>,

/// 
    #[serde(rename = "NestedTLBPageTableEvictionsPersec")]
    pub nested_tlbpage_table_evictions_persec: Option<u64>,

/// 
    #[serde(rename = "NestedTLBPageTableReclamationsPersec")]
    pub nested_tlbpage_table_reclamations_persec: Option<u64>,

/// 
    #[serde(rename = "NestedVMEntriesCost")]
    pub nested_vmentries_cost: Option<u64>,

/// 
    #[serde(rename = "NestedVMEntriesCost_Base")]
    pub nested_vmentries_cost__base: Option<u64>,

/// 
    #[serde(rename = "NestedVMEntriesPersec")]
    pub nested_vmentries_persec: Option<u64>,

/// 
    #[serde(rename = "OtherHypercallsPersec")]
    pub other_hypercalls_persec: Option<u64>,

/// 
    #[serde(rename = "OtherInterceptsCost")]
    pub other_intercepts_cost: Option<u64>,

/// 
    #[serde(rename = "OtherInterceptsCost_Base")]
    pub other_intercepts_cost__base: Option<u64>,

/// 
    #[serde(rename = "OtherInterceptsForwardedPersec")]
    pub other_intercepts_forwarded_persec: Option<u64>,

/// 
    #[serde(rename = "OtherInterceptsForwardingCost")]
    pub other_intercepts_forwarding_cost: Option<u64>,

/// 
    #[serde(rename = "OtherInterceptsForwardingCost_Base")]
    pub other_intercepts_forwarding_cost__base: Option<u64>,

/// 
    #[serde(rename = "OtherInterceptsPersec")]
    pub other_intercepts_persec: Option<u64>,

/// 
    #[serde(rename = "OtherMessagesPersec")]
    pub other_messages_persec: Option<u64>,

/// 
    #[serde(rename = "OtherReflectedGuestExceptionsPersec")]
    pub other_reflected_guest_exceptions_persec: Option<u64>,

/// 
    #[serde(rename = "PageFaultInterceptsCost")]
    pub page_fault_intercepts_cost: Option<u64>,

/// 
    #[serde(rename = "PageFaultInterceptsCost_Base")]
    pub page_fault_intercepts_cost__base: Option<u64>,

/// 
    #[serde(rename = "PageFaultInterceptsForwardedPersec")]
    pub page_fault_intercepts_forwarded_persec: Option<u64>,

/// 
    #[serde(rename = "PageFaultInterceptsForwardingCost")]
    pub page_fault_intercepts_forwarding_cost: Option<u64>,

/// 
    #[serde(rename = "PageFaultInterceptsForwardingCost_Base")]
    pub page_fault_intercepts_forwarding_cost__base: Option<u64>,

/// 
    #[serde(rename = "PageFaultInterceptsPersec")]
    pub page_fault_intercepts_persec: Option<u64>,

/// 
    #[serde(rename = "PageInvalidationsCost")]
    pub page_invalidations_cost: Option<u64>,

/// 
    #[serde(rename = "PageInvalidationsCost_Base")]
    pub page_invalidations_cost__base: Option<u64>,

/// 
    #[serde(rename = "PageInvalidationsForwardedPersec")]
    pub page_invalidations_forwarded_persec: Option<u64>,

/// 
    #[serde(rename = "PageInvalidationsForwardingCost")]
    pub page_invalidations_forwarding_cost: Option<u64>,

/// 
    #[serde(rename = "PageInvalidationsForwardingCost_Base")]
    pub page_invalidations_forwarding_cost__base: Option<u64>,

/// 
    #[serde(rename = "PageInvalidationsPersec")]
    pub page_invalidations_persec: Option<u64>,

/// 
    #[serde(rename = "PageScansPersec")]
    pub page_scans_persec: Option<u64>,

/// 
    #[serde(rename = "PageTableAllocationsPersec")]
    pub page_table_allocations_persec: Option<u64>,

/// 
    #[serde(rename = "PageTableEvictionsPersec")]
    pub page_table_evictions_persec: Option<u64>,

/// 
    #[serde(rename = "PageTableReclamationsPersec")]
    pub page_table_reclamations_persec: Option<u64>,

/// 
    #[serde(rename = "PageTableResetsPersec")]
    pub page_table_resets_persec: Option<u64>,

/// 
    #[serde(rename = "PageTableValidationsPersec")]
    pub page_table_validations_persec: Option<u64>,

/// 
    #[serde(rename = "PageTableWriteInterceptsPersec")]
    pub page_table_write_intercepts_persec: Option<u64>,

/// 
    #[serde(rename = "PendingInterruptsCost")]
    pub pending_interrupts_cost: Option<u64>,

/// 
    #[serde(rename = "PendingInterruptsCost_Base")]
    pub pending_interrupts_cost__base: Option<u64>,

/// 
    #[serde(rename = "PendingInterruptsForwardedPersec")]
    pub pending_interrupts_forwarded_persec: Option<u64>,

/// 
    #[serde(rename = "PendingInterruptsForwardingCost")]
    pub pending_interrupts_forwarding_cost: Option<u64>,

/// 
    #[serde(rename = "PendingInterruptsForwardingCost_Base")]
    pub pending_interrupts_forwarding_cost__base: Option<u64>,

/// 
    #[serde(rename = "PendingInterruptsPersec")]
    pub pending_interrupts_persec: Option<u64>,

/// 
    #[serde(rename = "PercentGuestRelativeUtilization")]
    pub percent_guest_relative_utilization: Option<u64>,

/// 
    #[serde(rename = "PercentGuestRelativeUtilization_Base")]
    pub percent_guest_relative_utilization__base: Option<u64>,

/// 
    #[serde(rename = "PercentGuestRunTime")]
    pub percent_guest_run_time: Option<u64>,

/// 
    #[serde(rename = "PercentGuestRunTime_Base")]
    pub percent_guest_run_time__base: Option<u64>,

/// 
    #[serde(rename = "PercentHypervisorRunTime")]
    pub percent_hypervisor_run_time: Option<u64>,

/// 
    #[serde(rename = "PercentHypervisorRunTime_Base")]
    pub percent_hypervisor_run_time__base: Option<u64>,

/// 
    #[serde(rename = "PercentRemoteRunTime")]
    pub percent_remote_run_time: Option<u64>,

/// 
    #[serde(rename = "PercentRemoteRunTime_Base")]
    pub percent_remote_run_time__base: Option<u64>,

/// 
    #[serde(rename = "PercentTotalCoreRunTime")]
    pub percent_total_core_run_time: Option<u64>,

/// 
    #[serde(rename = "PercentTotalCoreRunTime_Base")]
    pub percent_total_core_run_time__base: Option<u64>,

/// 
    #[serde(rename = "PercentTotalRunTime")]
    pub percent_total_run_time: Option<u64>,

/// 
    #[serde(rename = "PercentTotalRunTime_Base")]
    pub percent_total_run_time__base: Option<u64>,

/// 
    #[serde(rename = "PercentVTL1RunTime")]
    pub percent_vtl1_run_time: Option<u64>,

/// 
    #[serde(rename = "PercentVTL1RunTime_Base")]
    pub percent_vtl1_run_time__base: Option<u64>,

/// 
    #[serde(rename = "PercentVTL2RunTime")]
    pub percent_vtl2_run_time: Option<u64>,

/// 
    #[serde(rename = "PercentVTL2RunTime_Base")]
    pub percent_vtl2_run_time__base: Option<u64>,

/// 
    #[serde(rename = "PerformanceMonitoringInterruptsPersec")]
    pub performance_monitoring_interrupts_persec: Option<u64>,

/// 
    #[serde(rename = "PerformanceMonitoringIPTMSRAccessesPersec")]
    pub performance_monitoring_iptmsraccesses_persec: Option<u64>,

/// 
    #[serde(rename = "PerformanceMonitoringLBRMSRAccessesPersec")]
    pub performance_monitoring_lbrmsraccesses_persec: Option<u64>,

/// 
    #[serde(rename = "PerformanceMonitoringvPMUMSRAccessesPersec")]
    pub performance_monitoringv_pmumsraccesses_persec: Option<u64>,

/// 
    #[serde(rename = "PostedInterruptNotificationsPersec")]
    pub posted_interrupt_notifications_persec: Option<u64>,

/// 
    #[serde(rename = "PostedInterruptScansPersec")]
    pub posted_interrupt_scans_persec: Option<u64>,

/// 
    #[serde(rename = "RDPMCInstructionsCost")]
    pub rdpmcinstructions_cost: Option<u64>,

/// 
    #[serde(rename = "RDPMCInstructionsCost_Base")]
    pub rdpmcinstructions_cost__base: Option<u64>,

/// 
    #[serde(rename = "RDPMCInstructionsPersec")]
    pub rdpmcinstructions_persec: Option<u64>,

/// 
    #[serde(rename = "ReflectedGuestPageFaultsPersec")]
    pub reflected_guest_page_faults_persec: Option<u64>,

/// 
    #[serde(rename = "SchedulingPriority")]
    pub scheduling_priority: Option<u64>,

/// 
    #[serde(rename = "SmallPageTLBFillsPersec")]
    pub small_page_tlbfills_persec: Option<u64>,

/// 
    #[serde(rename = "SVMHypercallsPersec")]
    pub svmhypercalls_persec: Option<u64>,

/// 
    #[serde(rename = "SyntheticInterruptHypercallsPersec")]
    pub synthetic_interrupt_hypercalls_persec: Option<u64>,

/// 
    #[serde(rename = "SyntheticInterruptsPersec")]
    pub synthetic_interrupts_persec: Option<u64>,

/// 
    #[serde(rename = "TotalInterceptsCost")]
    pub total_intercepts_cost: Option<u64>,

/// 
    #[serde(rename = "TotalInterceptsCost_Base")]
    pub total_intercepts_cost__base: Option<u64>,

/// 
    #[serde(rename = "TotalInterceptsPersec")]
    pub total_intercepts_persec: Option<u64>,

/// 
    #[serde(rename = "TotalMessagesPersec")]
    pub total_messages_persec: Option<u64>,

/// 
    #[serde(rename = "TotalVirtualizationInstructionsEmulatedPersec")]
    pub total_virtualization_instructions_emulated_persec: Option<u64>,

/// 
    #[serde(rename = "TotalVirtualizationInstructionsEmulationCost")]
    pub total_virtualization_instructions_emulation_cost: Option<u64>,

/// 
    #[serde(rename = "TotalVirtualizationInstructionsEmulationCost_Base")]
    pub total_virtualization_instructions_emulation_cost__base: Option<u64>,

/// 
    #[serde(rename = "VirtualInterruptHypercallsPersec")]
    pub virtual_interrupt_hypercalls_persec: Option<u64>,

/// 
    #[serde(rename = "VirtualInterruptsPersec")]
    pub virtual_interrupts_persec: Option<u64>,

/// 
    #[serde(rename = "VirtualMMUHypercallsPersec")]
    pub virtual_mmuhypercalls_persec: Option<u64>,

/// 
    #[serde(rename = "VirtualProcessorHypercallsPersec")]
    pub virtual_processor_hypercalls_persec: Option<u64>,

/// 
    #[serde(rename = "VMCLEAREmulationInterceptsPersec")]
    pub vmclearemulation_intercepts_persec: Option<u64>,

/// 
    #[serde(rename = "VMCLEARInstructionEmulationCost")]
    pub vmclearinstruction_emulation_cost: Option<u64>,

/// 
    #[serde(rename = "VMCLEARInstructionEmulationCost_Base")]
    pub vmclearinstruction_emulation_cost__base: Option<u64>,

/// 
    #[serde(rename = "VMLOADEmulationInterceptsPersec")]
    pub vmloademulation_intercepts_persec: Option<u64>,

/// 
    #[serde(rename = "VMLOADInstructionEmulationCost")]
    pub vmloadinstruction_emulation_cost: Option<u64>,

/// 
    #[serde(rename = "VMLOADInstructionEmulationCost_Base")]
    pub vmloadinstruction_emulation_cost__base: Option<u64>,

/// 
    #[serde(rename = "VMPTRLDEmulationInterceptsPersec")]
    pub vmptrldemulation_intercepts_persec: Option<u64>,

/// 
    #[serde(rename = "VMPTRLDInstructionEmulationCost")]
    pub vmptrldinstruction_emulation_cost: Option<u64>,

/// 
    #[serde(rename = "VMPTRLDInstructionEmulationCost_Base")]
    pub vmptrldinstruction_emulation_cost__base: Option<u64>,

/// 
    #[serde(rename = "VMPTRSTEmulationInterceptsPersec")]
    pub vmptrstemulation_intercepts_persec: Option<u64>,

/// 
    #[serde(rename = "VMPTRSTInstructionEmulationCost")]
    pub vmptrstinstruction_emulation_cost: Option<u64>,

/// 
    #[serde(rename = "VMPTRSTInstructionEmulationCost_Base")]
    pub vmptrstinstruction_emulation_cost__base: Option<u64>,

/// 
    #[serde(rename = "VMREADEmulationInterceptsPersec")]
    pub vmreademulation_intercepts_persec: Option<u64>,

/// 
    #[serde(rename = "VMREADInstructionEmulationCost")]
    pub vmreadinstruction_emulation_cost: Option<u64>,

/// 
    #[serde(rename = "VMREADInstructionEmulationCost_Base")]
    pub vmreadinstruction_emulation_cost__base: Option<u64>,

/// 
    #[serde(rename = "VMSAVEEmulationInterceptsPersec")]
    pub vmsaveemulation_intercepts_persec: Option<u64>,

/// 
    #[serde(rename = "VMSAVEInstructionEmulationCost")]
    pub vmsaveinstruction_emulation_cost: Option<u64>,

/// 
    #[serde(rename = "VMSAVEInstructionEmulationCost_Base")]
    pub vmsaveinstruction_emulation_cost__base: Option<u64>,

/// 
    #[serde(rename = "VMWRITEEmulationInterceptsPersec")]
    pub vmwriteemulation_intercepts_persec: Option<u64>,

/// 
    #[serde(rename = "VMWRITEInstructionEmulationCost")]
    pub vmwriteinstruction_emulation_cost: Option<u64>,

/// 
    #[serde(rename = "VMWRITEInstructionEmulationCost_Base")]
    pub vmwriteinstruction_emulation_cost__base: Option<u64>,

/// 
    #[serde(rename = "VMXOFFEmulationInterceptsPersec")]
    pub vmxoffemulation_intercepts_persec: Option<u64>,

/// 
    #[serde(rename = "VMXOFFInstructionEmulationCost")]
    pub vmxoffinstruction_emulation_cost: Option<u64>,

/// 
    #[serde(rename = "VMXOFFInstructionEmulationCost_Base")]
    pub vmxoffinstruction_emulation_cost__base: Option<u64>,

/// 
    #[serde(rename = "VMXONEmulationInterceptsPersec")]
    pub vmxonemulation_intercepts_persec: Option<u64>,

/// 
    #[serde(rename = "VMXONInstructionEmulationCost")]
    pub vmxoninstruction_emulation_cost: Option<u64>,

/// 
    #[serde(rename = "VMXONInstructionEmulationCost_Base")]
    pub vmxoninstruction_emulation_cost__base: Option<u64>,

/// 
    #[serde(rename = "VSMHypercallsPersec")]
    pub vsmhypercalls_persec: Option<u64>,

/// 
    #[serde(rename = "VTL1AverageRunTime")]
    pub vtl1_average_run_time: Option<u64>,

/// 
    #[serde(rename = "VTL1AverageRunTime_Base")]
    pub vtl1_average_run_time__base: Option<u64>,

/// 
    #[serde(rename = "VTL1DispatchesPersec")]
    pub vtl1_dispatches_persec: Option<u64>,

/// 
    #[serde(rename = "VTL2AverageRunTime")]
    pub vtl2_average_run_time: Option<u64>,

/// 
    #[serde(rename = "VTL2AverageRunTime_Base")]
    pub vtl2_average_run_time__base: Option<u64>,

/// 
    #[serde(rename = "VTL2DispatchesPersec")]
    pub vtl2_dispatches_persec: Option<u64>,
}

impl Win32_PerfRawData_HvStats_HyperVHypervisorRootVirtualProcessor {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            address_domain_flushes_persec: None,
            address_space_evictions_persec: None,
            address_space_flushes_persec: None,
            address_space_switches_persec: None,
            apiceoiaccesses_persec: None,
            apicipis_sent_persec: None,
            apicmmioaccesses_persec: None,
            apicself_ipis_sent_persec: None,
            apictpraccesses_persec: None,
            bus_lock_acquisitions_persec: None,
            control_register_accesses_cost: None,
            control_register_accesses_cost__base: None,
            control_register_accesses_forwarded_persec: None,
            control_register_accesses_forwarding_cost: None,
            control_register_accesses_forwarding_cost__base: None,
            control_register_accesses_persec: None,
            cppcrequest_context_switches_persec: None,
            cpucontention_time_per_dispatch: None,
            cpucontention_time_per_dispatch__base: None,
            cpugroup_hypercalls_persec: None,
            cpuidinstructions_cost: None,
            cpuidinstructions_cost__base: None,
            cpuidinstructions_forwarded_persec: None,
            cpuidinstructions_forwarding_cost: None,
            cpuidinstructions_forwarding_cost__base: None,
            cpuidinstructions_persec: None,
            cpuwait_time_per_dispatch: None,
            cpuwait_time_per_dispatch__base: None,
            cpuwake_up_time_per_dispatch: None,
            cpuwake_up_time_per_dispatch__base: None,
            debug_register_accesses_cost: None,
            debug_register_accesses_cost__base: None,
            debug_register_accesses_forwarded_persec: None,
            debug_register_accesses_forwarding_cost: None,
            debug_register_accesses_forwarding_cost__base: None,
            debug_register_accesses_persec: None,
            deposit_hypercalls_persec: None,
            device_domain_hypercalls_persec: None,
            emulated_instructions_cost: None,
            emulated_instructions_cost__base: None,
            emulated_instructions_forwarded_persec: None,
            emulated_instructions_forwarding_cost: None,
            emulated_instructions_forwarding_cost__base: None,
            emulated_instructions_persec: None,
            event_log_hypercalls_persec: None,
            extended_hypercall_intercept_messages_persec: None,
            extended_hypercalls_persec: None,
            external_interrupts_cost: None,
            external_interrupts_cost__base: None,
            external_interrupts_forwarded_persec: None,
            external_interrupts_persec: None,
            flush_physical_address_list_hypercalls_persec: None,
            flush_physical_address_space_hypercalls_persec: None,
            gifinstruction_emulation_cost: None,
            gifinstruction_emulation_cost__base: None,
            gifinstruction_emulation_intercepts_persec: None,
            global_gvarange_flushes_persec: None,
            global_iotlbflush_cost: None,
            global_iotlbflush_cost__base: None,
            global_iotlbflushes_persec: None,
            gpaspace_hypercalls_persec: None,
            guest_page_table_maps_persec: None,
            hardware_interrupts_persec: None,
            hltinstructions_cost: None,
            hltinstructions_cost__base: None,
            hltinstructions_forwarded_persec: None,
            hltinstructions_forwarding_cost: None,
            hltinstructions_forwarding_cost__base: None,
            hltinstructions_persec: None,
            hypercalls_cost: None,
            hypercalls_cost__base: None,
            hypercalls_forwarded_persec: None,
            hypercalls_forwarding_cost: None,
            hypercalls_forwarding_cost__base: None,
            hypercalls_persec: None,
            inv_ept_all_context_emulation_intercepts_persec: None,
            inv_ept_all_context_instruction_emulation_cost: None,
            inv_ept_all_context_instruction_emulation_cost__base: None,
            inv_ept_single_context_emulation_intercepts_persec: None,
            inv_ept_single_context_instruction_emulation_cost: None,
            inv_ept_single_context_instruction_emulation_cost__base: None,
            inv_vpid_all_context_emulation_intercepts_persec: None,
            inv_vpid_all_context_instruction_emulation_cost: None,
            inv_vpid_all_context_instruction_emulation_cost__base: None,
            inv_vpid_single_address_emulation_intercepts_persec: None,
            inv_vpid_single_address_instruction_emulation_cost: None,
            inv_vpid_single_address_instruction_emulation_cost__base: None,
            inv_vpid_single_context_emulation_intercepts_persec: None,
            inv_vpid_single_context_instruction_emulation_cost: None,
            inv_vpid_single_context_instruction_emulation_cost__base: None,
            ioinstructions_cost: None,
            ioinstructions_cost__base: None,
            ioinstructions_forwarded_persec: None,
            ioinstructions_forwarding_cost: None,
            ioinstructions_forwarding_cost__base: None,
            ioinstructions_persec: None,
            iointercept_messages_persec: None,
            iommuhypercalls_persec: None,
            large_page_tlbfills_persec: None,
            local_flushed_gvaranges_persec: None,
            local_iotlbflush_cost: None,
            local_iotlbflush_cost__base: None,
            local_iotlbflushes_persec: None,
            logical_processor_dispatches_persec: None,
            logical_processor_hypercalls_persec: None,
            logical_processor_migrations_persec: None,
            long_spin_wait_hypercalls_persec: None,
            mbecnested_page_table_switches_persec: None,
            memory_intercept_messages_persec: None,
            msraccesses_cost: None,
            msraccesses_cost__base: None,
            msraccesses_forwarded_persec: None,
            msraccesses_forwarding_cost: None,
            msraccesses_forwarding_cost__base: None,
            msraccesses_persec: None,
            mwaitinstructions_cost: None,
            mwaitinstructions_cost__base: None,
            mwaitinstructions_forwarded_persec: None,
            mwaitinstructions_forwarding_cost: None,
            mwaitinstructions_forwarding_cost__base: None,
            mwaitinstructions_persec: None,
            nested_page_fault_intercepts_cost: None,
            nested_page_fault_intercepts_cost__base: None,
            nested_page_fault_intercepts_persec: None,
            nested_slathard_page_faults_cost: None,
            nested_slathard_page_faults_cost__base: None,
            nested_slathard_page_faults_persec: None,
            nested_slatsoft_page_faults_cost: None,
            nested_slatsoft_page_faults_cost__base: None,
            nested_slatsoft_page_faults_persec: None,
            nested_tlbpage_table_evictions_persec: None,
            nested_tlbpage_table_reclamations_persec: None,
            nested_vmentries_cost: None,
            nested_vmentries_cost__base: None,
            nested_vmentries_persec: None,
            other_hypercalls_persec: None,
            other_intercepts_cost: None,
            other_intercepts_cost__base: None,
            other_intercepts_forwarded_persec: None,
            other_intercepts_forwarding_cost: None,
            other_intercepts_forwarding_cost__base: None,
            other_intercepts_persec: None,
            other_messages_persec: None,
            other_reflected_guest_exceptions_persec: None,
            page_fault_intercepts_cost: None,
            page_fault_intercepts_cost__base: None,
            page_fault_intercepts_forwarded_persec: None,
            page_fault_intercepts_forwarding_cost: None,
            page_fault_intercepts_forwarding_cost__base: None,
            page_fault_intercepts_persec: None,
            page_invalidations_cost: None,
            page_invalidations_cost__base: None,
            page_invalidations_forwarded_persec: None,
            page_invalidations_forwarding_cost: None,
            page_invalidations_forwarding_cost__base: None,
            page_invalidations_persec: None,
            page_scans_persec: None,
            page_table_allocations_persec: None,
            page_table_evictions_persec: None,
            page_table_reclamations_persec: None,
            page_table_resets_persec: None,
            page_table_validations_persec: None,
            page_table_write_intercepts_persec: None,
            pending_interrupts_cost: None,
            pending_interrupts_cost__base: None,
            pending_interrupts_forwarded_persec: None,
            pending_interrupts_forwarding_cost: None,
            pending_interrupts_forwarding_cost__base: None,
            pending_interrupts_persec: None,
            percent_guest_relative_utilization: None,
            percent_guest_relative_utilization__base: None,
            percent_guest_run_time: None,
            percent_guest_run_time__base: None,
            percent_hypervisor_run_time: None,
            percent_hypervisor_run_time__base: None,
            percent_remote_run_time: None,
            percent_remote_run_time__base: None,
            percent_total_core_run_time: None,
            percent_total_core_run_time__base: None,
            percent_total_run_time: None,
            percent_total_run_time__base: None,
            percent_vtl1_run_time: None,
            percent_vtl1_run_time__base: None,
            percent_vtl2_run_time: None,
            percent_vtl2_run_time__base: None,
            performance_monitoring_interrupts_persec: None,
            performance_monitoring_iptmsraccesses_persec: None,
            performance_monitoring_lbrmsraccesses_persec: None,
            performance_monitoringv_pmumsraccesses_persec: None,
            posted_interrupt_notifications_persec: None,
            posted_interrupt_scans_persec: None,
            rdpmcinstructions_cost: None,
            rdpmcinstructions_cost__base: None,
            rdpmcinstructions_persec: None,
            reflected_guest_page_faults_persec: None,
            scheduling_priority: None,
            small_page_tlbfills_persec: None,
            svmhypercalls_persec: None,
            synthetic_interrupt_hypercalls_persec: None,
            synthetic_interrupts_persec: None,
            total_intercepts_cost: None,
            total_intercepts_cost__base: None,
            total_intercepts_persec: None,
            total_messages_persec: None,
            total_virtualization_instructions_emulated_persec: None,
            total_virtualization_instructions_emulation_cost: None,
            total_virtualization_instructions_emulation_cost__base: None,
            virtual_interrupt_hypercalls_persec: None,
            virtual_interrupts_persec: None,
            virtual_mmuhypercalls_persec: None,
            virtual_processor_hypercalls_persec: None,
            vmclearemulation_intercepts_persec: None,
            vmclearinstruction_emulation_cost: None,
            vmclearinstruction_emulation_cost__base: None,
            vmloademulation_intercepts_persec: None,
            vmloadinstruction_emulation_cost: None,
            vmloadinstruction_emulation_cost__base: None,
            vmptrldemulation_intercepts_persec: None,
            vmptrldinstruction_emulation_cost: None,
            vmptrldinstruction_emulation_cost__base: None,
            vmptrstemulation_intercepts_persec: None,
            vmptrstinstruction_emulation_cost: None,
            vmptrstinstruction_emulation_cost__base: None,
            vmreademulation_intercepts_persec: None,
            vmreadinstruction_emulation_cost: None,
            vmreadinstruction_emulation_cost__base: None,
            vmsaveemulation_intercepts_persec: None,
            vmsaveinstruction_emulation_cost: None,
            vmsaveinstruction_emulation_cost__base: None,
            vmwriteemulation_intercepts_persec: None,
            vmwriteinstruction_emulation_cost: None,
            vmwriteinstruction_emulation_cost__base: None,
            vmxoffemulation_intercepts_persec: None,
            vmxoffinstruction_emulation_cost: None,
            vmxoffinstruction_emulation_cost__base: None,
            vmxonemulation_intercepts_persec: None,
            vmxoninstruction_emulation_cost: None,
            vmxoninstruction_emulation_cost__base: None,
            vsmhypercalls_persec: None,
            vtl1_average_run_time: None,
            vtl1_average_run_time__base: None,
            vtl1_dispatches_persec: None,
            vtl2_average_run_time: None,
            vtl2_average_run_time__base: None,
            vtl2_dispatches_persec: None,
        }
    }


    /// Sets the value of AddressDomainFlushesPersec
    pub fn set_address_domain_flushes_persec(&mut self, value: u64) {
        self.address_domain_flushes_persec = Some(value);
    }

    /// Gets the value of AddressDomainFlushesPersec
    pub fn get_address_domain_flushes_persec(&self) -> Option<&u64> {
        self.address_domain_flushes_persec.as_ref()
    }

    /// Sets the value of AddressSpaceEvictionsPersec
    pub fn set_address_space_evictions_persec(&mut self, value: u64) {
        self.address_space_evictions_persec = Some(value);
    }

    /// Gets the value of AddressSpaceEvictionsPersec
    pub fn get_address_space_evictions_persec(&self) -> Option<&u64> {
        self.address_space_evictions_persec.as_ref()
    }

    /// Sets the value of AddressSpaceFlushesPersec
    pub fn set_address_space_flushes_persec(&mut self, value: u64) {
        self.address_space_flushes_persec = Some(value);
    }

    /// Gets the value of AddressSpaceFlushesPersec
    pub fn get_address_space_flushes_persec(&self) -> Option<&u64> {
        self.address_space_flushes_persec.as_ref()
    }

    /// Sets the value of AddressSpaceSwitchesPersec
    pub fn set_address_space_switches_persec(&mut self, value: u64) {
        self.address_space_switches_persec = Some(value);
    }

    /// Gets the value of AddressSpaceSwitchesPersec
    pub fn get_address_space_switches_persec(&self) -> Option<&u64> {
        self.address_space_switches_persec.as_ref()
    }

    /// Sets the value of APICEOIAccessesPersec
    pub fn set_apiceoiaccesses_persec(&mut self, value: u64) {
        self.apiceoiaccesses_persec = Some(value);
    }

    /// Gets the value of APICEOIAccessesPersec
    pub fn get_apiceoiaccesses_persec(&self) -> Option<&u64> {
        self.apiceoiaccesses_persec.as_ref()
    }

    /// Sets the value of APICIPIsSentPersec
    pub fn set_apicipis_sent_persec(&mut self, value: u64) {
        self.apicipis_sent_persec = Some(value);
    }

    /// Gets the value of APICIPIsSentPersec
    pub fn get_apicipis_sent_persec(&self) -> Option<&u64> {
        self.apicipis_sent_persec.as_ref()
    }

    /// Sets the value of APICMMIOAccessesPersec
    pub fn set_apicmmioaccesses_persec(&mut self, value: u64) {
        self.apicmmioaccesses_persec = Some(value);
    }

    /// Gets the value of APICMMIOAccessesPersec
    pub fn get_apicmmioaccesses_persec(&self) -> Option<&u64> {
        self.apicmmioaccesses_persec.as_ref()
    }

    /// Sets the value of APICSelfIPIsSentPersec
    pub fn set_apicself_ipis_sent_persec(&mut self, value: u64) {
        self.apicself_ipis_sent_persec = Some(value);
    }

    /// Gets the value of APICSelfIPIsSentPersec
    pub fn get_apicself_ipis_sent_persec(&self) -> Option<&u64> {
        self.apicself_ipis_sent_persec.as_ref()
    }

    /// Sets the value of APICTPRAccessesPersec
    pub fn set_apictpraccesses_persec(&mut self, value: u64) {
        self.apictpraccesses_persec = Some(value);
    }

    /// Gets the value of APICTPRAccessesPersec
    pub fn get_apictpraccesses_persec(&self) -> Option<&u64> {
        self.apictpraccesses_persec.as_ref()
    }

    /// Sets the value of BusLockAcquisitionsPersec
    pub fn set_bus_lock_acquisitions_persec(&mut self, value: u64) {
        self.bus_lock_acquisitions_persec = Some(value);
    }

    /// Gets the value of BusLockAcquisitionsPersec
    pub fn get_bus_lock_acquisitions_persec(&self) -> Option<&u64> {
        self.bus_lock_acquisitions_persec.as_ref()
    }

    /// Sets the value of ControlRegisterAccessesCost
    pub fn set_control_register_accesses_cost(&mut self, value: u64) {
        self.control_register_accesses_cost = Some(value);
    }

    /// Gets the value of ControlRegisterAccessesCost
    pub fn get_control_register_accesses_cost(&self) -> Option<&u64> {
        self.control_register_accesses_cost.as_ref()
    }

    /// Sets the value of ControlRegisterAccessesCost_Base
    pub fn set_control_register_accesses_cost__base(&mut self, value: u64) {
        self.control_register_accesses_cost__base = Some(value);
    }

    /// Gets the value of ControlRegisterAccessesCost_Base
    pub fn get_control_register_accesses_cost__base(&self) -> Option<&u64> {
        self.control_register_accesses_cost__base.as_ref()
    }

    /// Sets the value of ControlRegisterAccessesForwardedPersec
    pub fn set_control_register_accesses_forwarded_persec(&mut self, value: u64) {
        self.control_register_accesses_forwarded_persec = Some(value);
    }

    /// Gets the value of ControlRegisterAccessesForwardedPersec
    pub fn get_control_register_accesses_forwarded_persec(&self) -> Option<&u64> {
        self.control_register_accesses_forwarded_persec.as_ref()
    }

    /// Sets the value of ControlRegisterAccessesForwardingCost
    pub fn set_control_register_accesses_forwarding_cost(&mut self, value: u64) {
        self.control_register_accesses_forwarding_cost = Some(value);
    }

    /// Gets the value of ControlRegisterAccessesForwardingCost
    pub fn get_control_register_accesses_forwarding_cost(&self) -> Option<&u64> {
        self.control_register_accesses_forwarding_cost.as_ref()
    }

    /// Sets the value of ControlRegisterAccessesForwardingCost_Base
    pub fn set_control_register_accesses_forwarding_cost__base(&mut self, value: u64) {
        self.control_register_accesses_forwarding_cost__base = Some(value);
    }

    /// Gets the value of ControlRegisterAccessesForwardingCost_Base
    pub fn get_control_register_accesses_forwarding_cost__base(&self) -> Option<&u64> {
        self.control_register_accesses_forwarding_cost__base.as_ref()
    }

    /// Sets the value of ControlRegisterAccessesPersec
    pub fn set_control_register_accesses_persec(&mut self, value: u64) {
        self.control_register_accesses_persec = Some(value);
    }

    /// Gets the value of ControlRegisterAccessesPersec
    pub fn get_control_register_accesses_persec(&self) -> Option<&u64> {
        self.control_register_accesses_persec.as_ref()
    }

    /// Sets the value of CPPCRequestContextSwitchesPersec
    pub fn set_cppcrequest_context_switches_persec(&mut self, value: u64) {
        self.cppcrequest_context_switches_persec = Some(value);
    }

    /// Gets the value of CPPCRequestContextSwitchesPersec
    pub fn get_cppcrequest_context_switches_persec(&self) -> Option<&u64> {
        self.cppcrequest_context_switches_persec.as_ref()
    }

    /// Sets the value of CPUContentionTimePerDispatch
    pub fn set_cpucontention_time_per_dispatch(&mut self, value: u64) {
        self.cpucontention_time_per_dispatch = Some(value);
    }

    /// Gets the value of CPUContentionTimePerDispatch
    pub fn get_cpucontention_time_per_dispatch(&self) -> Option<&u64> {
        self.cpucontention_time_per_dispatch.as_ref()
    }

    /// Sets the value of CPUContentionTimePerDispatch_Base
    pub fn set_cpucontention_time_per_dispatch__base(&mut self, value: u64) {
        self.cpucontention_time_per_dispatch__base = Some(value);
    }

    /// Gets the value of CPUContentionTimePerDispatch_Base
    pub fn get_cpucontention_time_per_dispatch__base(&self) -> Option<&u64> {
        self.cpucontention_time_per_dispatch__base.as_ref()
    }

    /// Sets the value of CPUGroupHypercallsPersec
    pub fn set_cpugroup_hypercalls_persec(&mut self, value: u64) {
        self.cpugroup_hypercalls_persec = Some(value);
    }

    /// Gets the value of CPUGroupHypercallsPersec
    pub fn get_cpugroup_hypercalls_persec(&self) -> Option<&u64> {
        self.cpugroup_hypercalls_persec.as_ref()
    }

    /// Sets the value of CPUIDInstructionsCost
    pub fn set_cpuidinstructions_cost(&mut self, value: u64) {
        self.cpuidinstructions_cost = Some(value);
    }

    /// Gets the value of CPUIDInstructionsCost
    pub fn get_cpuidinstructions_cost(&self) -> Option<&u64> {
        self.cpuidinstructions_cost.as_ref()
    }

    /// Sets the value of CPUIDInstructionsCost_Base
    pub fn set_cpuidinstructions_cost__base(&mut self, value: u64) {
        self.cpuidinstructions_cost__base = Some(value);
    }

    /// Gets the value of CPUIDInstructionsCost_Base
    pub fn get_cpuidinstructions_cost__base(&self) -> Option<&u64> {
        self.cpuidinstructions_cost__base.as_ref()
    }

    /// Sets the value of CPUIDInstructionsForwardedPersec
    pub fn set_cpuidinstructions_forwarded_persec(&mut self, value: u64) {
        self.cpuidinstructions_forwarded_persec = Some(value);
    }

    /// Gets the value of CPUIDInstructionsForwardedPersec
    pub fn get_cpuidinstructions_forwarded_persec(&self) -> Option<&u64> {
        self.cpuidinstructions_forwarded_persec.as_ref()
    }

    /// Sets the value of CPUIDInstructionsForwardingCost
    pub fn set_cpuidinstructions_forwarding_cost(&mut self, value: u64) {
        self.cpuidinstructions_forwarding_cost = Some(value);
    }

    /// Gets the value of CPUIDInstructionsForwardingCost
    pub fn get_cpuidinstructions_forwarding_cost(&self) -> Option<&u64> {
        self.cpuidinstructions_forwarding_cost.as_ref()
    }

    /// Sets the value of CPUIDInstructionsForwardingCost_Base
    pub fn set_cpuidinstructions_forwarding_cost__base(&mut self, value: u64) {
        self.cpuidinstructions_forwarding_cost__base = Some(value);
    }

    /// Gets the value of CPUIDInstructionsForwardingCost_Base
    pub fn get_cpuidinstructions_forwarding_cost__base(&self) -> Option<&u64> {
        self.cpuidinstructions_forwarding_cost__base.as_ref()
    }

    /// Sets the value of CPUIDInstructionsPersec
    pub fn set_cpuidinstructions_persec(&mut self, value: u64) {
        self.cpuidinstructions_persec = Some(value);
    }

    /// Gets the value of CPUIDInstructionsPersec
    pub fn get_cpuidinstructions_persec(&self) -> Option<&u64> {
        self.cpuidinstructions_persec.as_ref()
    }

    /// Sets the value of CPUWaitTimePerDispatch
    pub fn set_cpuwait_time_per_dispatch(&mut self, value: u64) {
        self.cpuwait_time_per_dispatch = Some(value);
    }

    /// Gets the value of CPUWaitTimePerDispatch
    pub fn get_cpuwait_time_per_dispatch(&self) -> Option<&u64> {
        self.cpuwait_time_per_dispatch.as_ref()
    }

    /// Sets the value of CPUWaitTimePerDispatch_Base
    pub fn set_cpuwait_time_per_dispatch__base(&mut self, value: u64) {
        self.cpuwait_time_per_dispatch__base = Some(value);
    }

    /// Gets the value of CPUWaitTimePerDispatch_Base
    pub fn get_cpuwait_time_per_dispatch__base(&self) -> Option<&u64> {
        self.cpuwait_time_per_dispatch__base.as_ref()
    }

    /// Sets the value of CPUWakeUpTimePerDispatch
    pub fn set_cpuwake_up_time_per_dispatch(&mut self, value: u64) {
        self.cpuwake_up_time_per_dispatch = Some(value);
    }

    /// Gets the value of CPUWakeUpTimePerDispatch
    pub fn get_cpuwake_up_time_per_dispatch(&self) -> Option<&u64> {
        self.cpuwake_up_time_per_dispatch.as_ref()
    }

    /// Sets the value of CPUWakeUpTimePerDispatch_Base
    pub fn set_cpuwake_up_time_per_dispatch__base(&mut self, value: u64) {
        self.cpuwake_up_time_per_dispatch__base = Some(value);
    }

    /// Gets the value of CPUWakeUpTimePerDispatch_Base
    pub fn get_cpuwake_up_time_per_dispatch__base(&self) -> Option<&u64> {
        self.cpuwake_up_time_per_dispatch__base.as_ref()
    }

    /// Sets the value of DebugRegisterAccessesCost
    pub fn set_debug_register_accesses_cost(&mut self, value: u64) {
        self.debug_register_accesses_cost = Some(value);
    }

    /// Gets the value of DebugRegisterAccessesCost
    pub fn get_debug_register_accesses_cost(&self) -> Option<&u64> {
        self.debug_register_accesses_cost.as_ref()
    }

    /// Sets the value of DebugRegisterAccessesCost_Base
    pub fn set_debug_register_accesses_cost__base(&mut self, value: u64) {
        self.debug_register_accesses_cost__base = Some(value);
    }

    /// Gets the value of DebugRegisterAccessesCost_Base
    pub fn get_debug_register_accesses_cost__base(&self) -> Option<&u64> {
        self.debug_register_accesses_cost__base.as_ref()
    }

    /// Sets the value of DebugRegisterAccessesForwardedPersec
    pub fn set_debug_register_accesses_forwarded_persec(&mut self, value: u64) {
        self.debug_register_accesses_forwarded_persec = Some(value);
    }

    /// Gets the value of DebugRegisterAccessesForwardedPersec
    pub fn get_debug_register_accesses_forwarded_persec(&self) -> Option<&u64> {
        self.debug_register_accesses_forwarded_persec.as_ref()
    }

    /// Sets the value of DebugRegisterAccessesForwardingCost
    pub fn set_debug_register_accesses_forwarding_cost(&mut self, value: u64) {
        self.debug_register_accesses_forwarding_cost = Some(value);
    }

    /// Gets the value of DebugRegisterAccessesForwardingCost
    pub fn get_debug_register_accesses_forwarding_cost(&self) -> Option<&u64> {
        self.debug_register_accesses_forwarding_cost.as_ref()
    }

    /// Sets the value of DebugRegisterAccessesForwardingCost_Base
    pub fn set_debug_register_accesses_forwarding_cost__base(&mut self, value: u64) {
        self.debug_register_accesses_forwarding_cost__base = Some(value);
    }

    /// Gets the value of DebugRegisterAccessesForwardingCost_Base
    pub fn get_debug_register_accesses_forwarding_cost__base(&self) -> Option<&u64> {
        self.debug_register_accesses_forwarding_cost__base.as_ref()
    }

    /// Sets the value of DebugRegisterAccessesPersec
    pub fn set_debug_register_accesses_persec(&mut self, value: u64) {
        self.debug_register_accesses_persec = Some(value);
    }

    /// Gets the value of DebugRegisterAccessesPersec
    pub fn get_debug_register_accesses_persec(&self) -> Option<&u64> {
        self.debug_register_accesses_persec.as_ref()
    }

    /// Sets the value of DepositHypercallsPersec
    pub fn set_deposit_hypercalls_persec(&mut self, value: u64) {
        self.deposit_hypercalls_persec = Some(value);
    }

    /// Gets the value of DepositHypercallsPersec
    pub fn get_deposit_hypercalls_persec(&self) -> Option<&u64> {
        self.deposit_hypercalls_persec.as_ref()
    }

    /// Sets the value of DeviceDomainHypercallsPersec
    pub fn set_device_domain_hypercalls_persec(&mut self, value: u64) {
        self.device_domain_hypercalls_persec = Some(value);
    }

    /// Gets the value of DeviceDomainHypercallsPersec
    pub fn get_device_domain_hypercalls_persec(&self) -> Option<&u64> {
        self.device_domain_hypercalls_persec.as_ref()
    }

    /// Sets the value of EmulatedInstructionsCost
    pub fn set_emulated_instructions_cost(&mut self, value: u64) {
        self.emulated_instructions_cost = Some(value);
    }

    /// Gets the value of EmulatedInstructionsCost
    pub fn get_emulated_instructions_cost(&self) -> Option<&u64> {
        self.emulated_instructions_cost.as_ref()
    }

    /// Sets the value of EmulatedInstructionsCost_Base
    pub fn set_emulated_instructions_cost__base(&mut self, value: u64) {
        self.emulated_instructions_cost__base = Some(value);
    }

    /// Gets the value of EmulatedInstructionsCost_Base
    pub fn get_emulated_instructions_cost__base(&self) -> Option<&u64> {
        self.emulated_instructions_cost__base.as_ref()
    }

    /// Sets the value of EmulatedInstructionsForwardedPersec
    pub fn set_emulated_instructions_forwarded_persec(&mut self, value: u64) {
        self.emulated_instructions_forwarded_persec = Some(value);
    }

    /// Gets the value of EmulatedInstructionsForwardedPersec
    pub fn get_emulated_instructions_forwarded_persec(&self) -> Option<&u64> {
        self.emulated_instructions_forwarded_persec.as_ref()
    }

    /// Sets the value of EmulatedInstructionsForwardingCost
    pub fn set_emulated_instructions_forwarding_cost(&mut self, value: u64) {
        self.emulated_instructions_forwarding_cost = Some(value);
    }

    /// Gets the value of EmulatedInstructionsForwardingCost
    pub fn get_emulated_instructions_forwarding_cost(&self) -> Option<&u64> {
        self.emulated_instructions_forwarding_cost.as_ref()
    }

    /// Sets the value of EmulatedInstructionsForwardingCost_Base
    pub fn set_emulated_instructions_forwarding_cost__base(&mut self, value: u64) {
        self.emulated_instructions_forwarding_cost__base = Some(value);
    }

    /// Gets the value of EmulatedInstructionsForwardingCost_Base
    pub fn get_emulated_instructions_forwarding_cost__base(&self) -> Option<&u64> {
        self.emulated_instructions_forwarding_cost__base.as_ref()
    }

    /// Sets the value of EmulatedInstructionsPersec
    pub fn set_emulated_instructions_persec(&mut self, value: u64) {
        self.emulated_instructions_persec = Some(value);
    }

    /// Gets the value of EmulatedInstructionsPersec
    pub fn get_emulated_instructions_persec(&self) -> Option<&u64> {
        self.emulated_instructions_persec.as_ref()
    }

    /// Sets the value of EventLogHypercallsPersec
    pub fn set_event_log_hypercalls_persec(&mut self, value: u64) {
        self.event_log_hypercalls_persec = Some(value);
    }

    /// Gets the value of EventLogHypercallsPersec
    pub fn get_event_log_hypercalls_persec(&self) -> Option<&u64> {
        self.event_log_hypercalls_persec.as_ref()
    }

    /// Sets the value of ExtendedHypercallInterceptMessagesPersec
    pub fn set_extended_hypercall_intercept_messages_persec(&mut self, value: u64) {
        self.extended_hypercall_intercept_messages_persec = Some(value);
    }

    /// Gets the value of ExtendedHypercallInterceptMessagesPersec
    pub fn get_extended_hypercall_intercept_messages_persec(&self) -> Option<&u64> {
        self.extended_hypercall_intercept_messages_persec.as_ref()
    }

    /// Sets the value of ExtendedHypercallsPersec
    pub fn set_extended_hypercalls_persec(&mut self, value: u64) {
        self.extended_hypercalls_persec = Some(value);
    }

    /// Gets the value of ExtendedHypercallsPersec
    pub fn get_extended_hypercalls_persec(&self) -> Option<&u64> {
        self.extended_hypercalls_persec.as_ref()
    }

    /// Sets the value of ExternalInterruptsCost
    pub fn set_external_interrupts_cost(&mut self, value: u64) {
        self.external_interrupts_cost = Some(value);
    }

    /// Gets the value of ExternalInterruptsCost
    pub fn get_external_interrupts_cost(&self) -> Option<&u64> {
        self.external_interrupts_cost.as_ref()
    }

    /// Sets the value of ExternalInterruptsCost_Base
    pub fn set_external_interrupts_cost__base(&mut self, value: u64) {
        self.external_interrupts_cost__base = Some(value);
    }

    /// Gets the value of ExternalInterruptsCost_Base
    pub fn get_external_interrupts_cost__base(&self) -> Option<&u64> {
        self.external_interrupts_cost__base.as_ref()
    }

    /// Sets the value of ExternalInterruptsForwardedPersec
    pub fn set_external_interrupts_forwarded_persec(&mut self, value: u64) {
        self.external_interrupts_forwarded_persec = Some(value);
    }

    /// Gets the value of ExternalInterruptsForwardedPersec
    pub fn get_external_interrupts_forwarded_persec(&self) -> Option<&u64> {
        self.external_interrupts_forwarded_persec.as_ref()
    }

    /// Sets the value of ExternalInterruptsPersec
    pub fn set_external_interrupts_persec(&mut self, value: u64) {
        self.external_interrupts_persec = Some(value);
    }

    /// Gets the value of ExternalInterruptsPersec
    pub fn get_external_interrupts_persec(&self) -> Option<&u64> {
        self.external_interrupts_persec.as_ref()
    }

    /// Sets the value of FlushPhysicalAddressListHypercallsPersec
    pub fn set_flush_physical_address_list_hypercalls_persec(&mut self, value: u64) {
        self.flush_physical_address_list_hypercalls_persec = Some(value);
    }

    /// Gets the value of FlushPhysicalAddressListHypercallsPersec
    pub fn get_flush_physical_address_list_hypercalls_persec(&self) -> Option<&u64> {
        self.flush_physical_address_list_hypercalls_persec.as_ref()
    }

    /// Sets the value of FlushPhysicalAddressSpaceHypercallsPersec
    pub fn set_flush_physical_address_space_hypercalls_persec(&mut self, value: u64) {
        self.flush_physical_address_space_hypercalls_persec = Some(value);
    }

    /// Gets the value of FlushPhysicalAddressSpaceHypercallsPersec
    pub fn get_flush_physical_address_space_hypercalls_persec(&self) -> Option<&u64> {
        self.flush_physical_address_space_hypercalls_persec.as_ref()
    }

    /// Sets the value of GIFInstructionEmulationCost
    pub fn set_gifinstruction_emulation_cost(&mut self, value: u64) {
        self.gifinstruction_emulation_cost = Some(value);
    }

    /// Gets the value of GIFInstructionEmulationCost
    pub fn get_gifinstruction_emulation_cost(&self) -> Option<&u64> {
        self.gifinstruction_emulation_cost.as_ref()
    }

    /// Sets the value of GIFInstructionEmulationCost_Base
    pub fn set_gifinstruction_emulation_cost__base(&mut self, value: u64) {
        self.gifinstruction_emulation_cost__base = Some(value);
    }

    /// Gets the value of GIFInstructionEmulationCost_Base
    pub fn get_gifinstruction_emulation_cost__base(&self) -> Option<&u64> {
        self.gifinstruction_emulation_cost__base.as_ref()
    }

    /// Sets the value of GIFInstructionEmulationInterceptsPersec
    pub fn set_gifinstruction_emulation_intercepts_persec(&mut self, value: u64) {
        self.gifinstruction_emulation_intercepts_persec = Some(value);
    }

    /// Gets the value of GIFInstructionEmulationInterceptsPersec
    pub fn get_gifinstruction_emulation_intercepts_persec(&self) -> Option<&u64> {
        self.gifinstruction_emulation_intercepts_persec.as_ref()
    }

    /// Sets the value of GlobalGVARangeFlushesPersec
    pub fn set_global_gvarange_flushes_persec(&mut self, value: u64) {
        self.global_gvarange_flushes_persec = Some(value);
    }

    /// Gets the value of GlobalGVARangeFlushesPersec
    pub fn get_global_gvarange_flushes_persec(&self) -> Option<&u64> {
        self.global_gvarange_flushes_persec.as_ref()
    }

    /// Sets the value of GlobalIOTLBFlushCost
    pub fn set_global_iotlbflush_cost(&mut self, value: u64) {
        self.global_iotlbflush_cost = Some(value);
    }

    /// Gets the value of GlobalIOTLBFlushCost
    pub fn get_global_iotlbflush_cost(&self) -> Option<&u64> {
        self.global_iotlbflush_cost.as_ref()
    }

    /// Sets the value of GlobalIOTLBFlushCost_Base
    pub fn set_global_iotlbflush_cost__base(&mut self, value: u64) {
        self.global_iotlbflush_cost__base = Some(value);
    }

    /// Gets the value of GlobalIOTLBFlushCost_Base
    pub fn get_global_iotlbflush_cost__base(&self) -> Option<&u64> {
        self.global_iotlbflush_cost__base.as_ref()
    }

    /// Sets the value of GlobalIOTLBFlushesPersec
    pub fn set_global_iotlbflushes_persec(&mut self, value: u64) {
        self.global_iotlbflushes_persec = Some(value);
    }

    /// Gets the value of GlobalIOTLBFlushesPersec
    pub fn get_global_iotlbflushes_persec(&self) -> Option<&u64> {
        self.global_iotlbflushes_persec.as_ref()
    }

    /// Sets the value of GPASpaceHypercallsPersec
    pub fn set_gpaspace_hypercalls_persec(&mut self, value: u64) {
        self.gpaspace_hypercalls_persec = Some(value);
    }

    /// Gets the value of GPASpaceHypercallsPersec
    pub fn get_gpaspace_hypercalls_persec(&self) -> Option<&u64> {
        self.gpaspace_hypercalls_persec.as_ref()
    }

    /// Sets the value of GuestPageTableMapsPersec
    pub fn set_guest_page_table_maps_persec(&mut self, value: u64) {
        self.guest_page_table_maps_persec = Some(value);
    }

    /// Gets the value of GuestPageTableMapsPersec
    pub fn get_guest_page_table_maps_persec(&self) -> Option<&u64> {
        self.guest_page_table_maps_persec.as_ref()
    }

    /// Sets the value of HardwareInterruptsPersec
    pub fn set_hardware_interrupts_persec(&mut self, value: u64) {
        self.hardware_interrupts_persec = Some(value);
    }

    /// Gets the value of HardwareInterruptsPersec
    pub fn get_hardware_interrupts_persec(&self) -> Option<&u64> {
        self.hardware_interrupts_persec.as_ref()
    }

    /// Sets the value of HLTInstructionsCost
    pub fn set_hltinstructions_cost(&mut self, value: u64) {
        self.hltinstructions_cost = Some(value);
    }

    /// Gets the value of HLTInstructionsCost
    pub fn get_hltinstructions_cost(&self) -> Option<&u64> {
        self.hltinstructions_cost.as_ref()
    }

    /// Sets the value of HLTInstructionsCost_Base
    pub fn set_hltinstructions_cost__base(&mut self, value: u64) {
        self.hltinstructions_cost__base = Some(value);
    }

    /// Gets the value of HLTInstructionsCost_Base
    pub fn get_hltinstructions_cost__base(&self) -> Option<&u64> {
        self.hltinstructions_cost__base.as_ref()
    }

    /// Sets the value of HLTInstructionsForwardedPersec
    pub fn set_hltinstructions_forwarded_persec(&mut self, value: u64) {
        self.hltinstructions_forwarded_persec = Some(value);
    }

    /// Gets the value of HLTInstructionsForwardedPersec
    pub fn get_hltinstructions_forwarded_persec(&self) -> Option<&u64> {
        self.hltinstructions_forwarded_persec.as_ref()
    }

    /// Sets the value of HLTInstructionsForwardingCost
    pub fn set_hltinstructions_forwarding_cost(&mut self, value: u64) {
        self.hltinstructions_forwarding_cost = Some(value);
    }

    /// Gets the value of HLTInstructionsForwardingCost
    pub fn get_hltinstructions_forwarding_cost(&self) -> Option<&u64> {
        self.hltinstructions_forwarding_cost.as_ref()
    }

    /// Sets the value of HLTInstructionsForwardingCost_Base
    pub fn set_hltinstructions_forwarding_cost__base(&mut self, value: u64) {
        self.hltinstructions_forwarding_cost__base = Some(value);
    }

    /// Gets the value of HLTInstructionsForwardingCost_Base
    pub fn get_hltinstructions_forwarding_cost__base(&self) -> Option<&u64> {
        self.hltinstructions_forwarding_cost__base.as_ref()
    }

    /// Sets the value of HLTInstructionsPersec
    pub fn set_hltinstructions_persec(&mut self, value: u64) {
        self.hltinstructions_persec = Some(value);
    }

    /// Gets the value of HLTInstructionsPersec
    pub fn get_hltinstructions_persec(&self) -> Option<&u64> {
        self.hltinstructions_persec.as_ref()
    }

    /// Sets the value of HypercallsCost
    pub fn set_hypercalls_cost(&mut self, value: u64) {
        self.hypercalls_cost = Some(value);
    }

    /// Gets the value of HypercallsCost
    pub fn get_hypercalls_cost(&self) -> Option<&u64> {
        self.hypercalls_cost.as_ref()
    }

    /// Sets the value of HypercallsCost_Base
    pub fn set_hypercalls_cost__base(&mut self, value: u64) {
        self.hypercalls_cost__base = Some(value);
    }

    /// Gets the value of HypercallsCost_Base
    pub fn get_hypercalls_cost__base(&self) -> Option<&u64> {
        self.hypercalls_cost__base.as_ref()
    }

    /// Sets the value of HypercallsForwardedPersec
    pub fn set_hypercalls_forwarded_persec(&mut self, value: u64) {
        self.hypercalls_forwarded_persec = Some(value);
    }

    /// Gets the value of HypercallsForwardedPersec
    pub fn get_hypercalls_forwarded_persec(&self) -> Option<&u64> {
        self.hypercalls_forwarded_persec.as_ref()
    }

    /// Sets the value of HypercallsForwardingCost
    pub fn set_hypercalls_forwarding_cost(&mut self, value: u64) {
        self.hypercalls_forwarding_cost = Some(value);
    }

    /// Gets the value of HypercallsForwardingCost
    pub fn get_hypercalls_forwarding_cost(&self) -> Option<&u64> {
        self.hypercalls_forwarding_cost.as_ref()
    }

    /// Sets the value of HypercallsForwardingCost_Base
    pub fn set_hypercalls_forwarding_cost__base(&mut self, value: u64) {
        self.hypercalls_forwarding_cost__base = Some(value);
    }

    /// Gets the value of HypercallsForwardingCost_Base
    pub fn get_hypercalls_forwarding_cost__base(&self) -> Option<&u64> {
        self.hypercalls_forwarding_cost__base.as_ref()
    }

    /// Sets the value of HypercallsPersec
    pub fn set_hypercalls_persec(&mut self, value: u64) {
        self.hypercalls_persec = Some(value);
    }

    /// Gets the value of HypercallsPersec
    pub fn get_hypercalls_persec(&self) -> Option<&u64> {
        self.hypercalls_persec.as_ref()
    }

    /// Sets the value of InvEptAllContextEmulationInterceptsPersec
    pub fn set_inv_ept_all_context_emulation_intercepts_persec(&mut self, value: u64) {
        self.inv_ept_all_context_emulation_intercepts_persec = Some(value);
    }

    /// Gets the value of InvEptAllContextEmulationInterceptsPersec
    pub fn get_inv_ept_all_context_emulation_intercepts_persec(&self) -> Option<&u64> {
        self.inv_ept_all_context_emulation_intercepts_persec.as_ref()
    }

    /// Sets the value of InvEptAllContextInstructionEmulationCost
    pub fn set_inv_ept_all_context_instruction_emulation_cost(&mut self, value: u64) {
        self.inv_ept_all_context_instruction_emulation_cost = Some(value);
    }

    /// Gets the value of InvEptAllContextInstructionEmulationCost
    pub fn get_inv_ept_all_context_instruction_emulation_cost(&self) -> Option<&u64> {
        self.inv_ept_all_context_instruction_emulation_cost.as_ref()
    }

    /// Sets the value of InvEptAllContextInstructionEmulationCost_Base
    pub fn set_inv_ept_all_context_instruction_emulation_cost__base(&mut self, value: u64) {
        self.inv_ept_all_context_instruction_emulation_cost__base = Some(value);
    }

    /// Gets the value of InvEptAllContextInstructionEmulationCost_Base
    pub fn get_inv_ept_all_context_instruction_emulation_cost__base(&self) -> Option<&u64> {
        self.inv_ept_all_context_instruction_emulation_cost__base.as_ref()
    }

    /// Sets the value of InvEptSingleContextEmulationInterceptsPersec
    pub fn set_inv_ept_single_context_emulation_intercepts_persec(&mut self, value: u64) {
        self.inv_ept_single_context_emulation_intercepts_persec = Some(value);
    }

    /// Gets the value of InvEptSingleContextEmulationInterceptsPersec
    pub fn get_inv_ept_single_context_emulation_intercepts_persec(&self) -> Option<&u64> {
        self.inv_ept_single_context_emulation_intercepts_persec.as_ref()
    }

    /// Sets the value of InvEptSingleContextInstructionEmulationCost
    pub fn set_inv_ept_single_context_instruction_emulation_cost(&mut self, value: u64) {
        self.inv_ept_single_context_instruction_emulation_cost = Some(value);
    }

    /// Gets the value of InvEptSingleContextInstructionEmulationCost
    pub fn get_inv_ept_single_context_instruction_emulation_cost(&self) -> Option<&u64> {
        self.inv_ept_single_context_instruction_emulation_cost.as_ref()
    }

    /// Sets the value of InvEptSingleContextInstructionEmulationCost_Base
    pub fn set_inv_ept_single_context_instruction_emulation_cost__base(&mut self, value: u64) {
        self.inv_ept_single_context_instruction_emulation_cost__base = Some(value);
    }

    /// Gets the value of InvEptSingleContextInstructionEmulationCost_Base
    pub fn get_inv_ept_single_context_instruction_emulation_cost__base(&self) -> Option<&u64> {
        self.inv_ept_single_context_instruction_emulation_cost__base.as_ref()
    }

    /// Sets the value of InvVpidAllContextEmulationInterceptsPersec
    pub fn set_inv_vpid_all_context_emulation_intercepts_persec(&mut self, value: u64) {
        self.inv_vpid_all_context_emulation_intercepts_persec = Some(value);
    }

    /// Gets the value of InvVpidAllContextEmulationInterceptsPersec
    pub fn get_inv_vpid_all_context_emulation_intercepts_persec(&self) -> Option<&u64> {
        self.inv_vpid_all_context_emulation_intercepts_persec.as_ref()
    }

    /// Sets the value of InvVpidAllContextInstructionEmulationCost
    pub fn set_inv_vpid_all_context_instruction_emulation_cost(&mut self, value: u64) {
        self.inv_vpid_all_context_instruction_emulation_cost = Some(value);
    }

    /// Gets the value of InvVpidAllContextInstructionEmulationCost
    pub fn get_inv_vpid_all_context_instruction_emulation_cost(&self) -> Option<&u64> {
        self.inv_vpid_all_context_instruction_emulation_cost.as_ref()
    }

    /// Sets the value of InvVpidAllContextInstructionEmulationCost_Base
    pub fn set_inv_vpid_all_context_instruction_emulation_cost__base(&mut self, value: u64) {
        self.inv_vpid_all_context_instruction_emulation_cost__base = Some(value);
    }

    /// Gets the value of InvVpidAllContextInstructionEmulationCost_Base
    pub fn get_inv_vpid_all_context_instruction_emulation_cost__base(&self) -> Option<&u64> {
        self.inv_vpid_all_context_instruction_emulation_cost__base.as_ref()
    }

    /// Sets the value of InvVpidSingleAddressEmulationInterceptsPersec
    pub fn set_inv_vpid_single_address_emulation_intercepts_persec(&mut self, value: u64) {
        self.inv_vpid_single_address_emulation_intercepts_persec = Some(value);
    }

    /// Gets the value of InvVpidSingleAddressEmulationInterceptsPersec
    pub fn get_inv_vpid_single_address_emulation_intercepts_persec(&self) -> Option<&u64> {
        self.inv_vpid_single_address_emulation_intercepts_persec.as_ref()
    }

    /// Sets the value of InvVpidSingleAddressInstructionEmulationCost
    pub fn set_inv_vpid_single_address_instruction_emulation_cost(&mut self, value: u64) {
        self.inv_vpid_single_address_instruction_emulation_cost = Some(value);
    }

    /// Gets the value of InvVpidSingleAddressInstructionEmulationCost
    pub fn get_inv_vpid_single_address_instruction_emulation_cost(&self) -> Option<&u64> {
        self.inv_vpid_single_address_instruction_emulation_cost.as_ref()
    }

    /// Sets the value of InvVpidSingleAddressInstructionEmulationCost_Base
    pub fn set_inv_vpid_single_address_instruction_emulation_cost__base(&mut self, value: u64) {
        self.inv_vpid_single_address_instruction_emulation_cost__base = Some(value);
    }

    /// Gets the value of InvVpidSingleAddressInstructionEmulationCost_Base
    pub fn get_inv_vpid_single_address_instruction_emulation_cost__base(&self) -> Option<&u64> {
        self.inv_vpid_single_address_instruction_emulation_cost__base.as_ref()
    }

    /// Sets the value of InvVpidSingleContextEmulationInterceptsPersec
    pub fn set_inv_vpid_single_context_emulation_intercepts_persec(&mut self, value: u64) {
        self.inv_vpid_single_context_emulation_intercepts_persec = Some(value);
    }

    /// Gets the value of InvVpidSingleContextEmulationInterceptsPersec
    pub fn get_inv_vpid_single_context_emulation_intercepts_persec(&self) -> Option<&u64> {
        self.inv_vpid_single_context_emulation_intercepts_persec.as_ref()
    }

    /// Sets the value of InvVpidSingleContextInstructionEmulationCost
    pub fn set_inv_vpid_single_context_instruction_emulation_cost(&mut self, value: u64) {
        self.inv_vpid_single_context_instruction_emulation_cost = Some(value);
    }

    /// Gets the value of InvVpidSingleContextInstructionEmulationCost
    pub fn get_inv_vpid_single_context_instruction_emulation_cost(&self) -> Option<&u64> {
        self.inv_vpid_single_context_instruction_emulation_cost.as_ref()
    }

    /// Sets the value of InvVpidSingleContextInstructionEmulationCost_Base
    pub fn set_inv_vpid_single_context_instruction_emulation_cost__base(&mut self, value: u64) {
        self.inv_vpid_single_context_instruction_emulation_cost__base = Some(value);
    }

    /// Gets the value of InvVpidSingleContextInstructionEmulationCost_Base
    pub fn get_inv_vpid_single_context_instruction_emulation_cost__base(&self) -> Option<&u64> {
        self.inv_vpid_single_context_instruction_emulation_cost__base.as_ref()
    }

    /// Sets the value of IOInstructionsCost
    pub fn set_ioinstructions_cost(&mut self, value: u64) {
        self.ioinstructions_cost = Some(value);
    }

    /// Gets the value of IOInstructionsCost
    pub fn get_ioinstructions_cost(&self) -> Option<&u64> {
        self.ioinstructions_cost.as_ref()
    }

    /// Sets the value of IOInstructionsCost_Base
    pub fn set_ioinstructions_cost__base(&mut self, value: u64) {
        self.ioinstructions_cost__base = Some(value);
    }

    /// Gets the value of IOInstructionsCost_Base
    pub fn get_ioinstructions_cost__base(&self) -> Option<&u64> {
        self.ioinstructions_cost__base.as_ref()
    }

    /// Sets the value of IOInstructionsForwardedPersec
    pub fn set_ioinstructions_forwarded_persec(&mut self, value: u64) {
        self.ioinstructions_forwarded_persec = Some(value);
    }

    /// Gets the value of IOInstructionsForwardedPersec
    pub fn get_ioinstructions_forwarded_persec(&self) -> Option<&u64> {
        self.ioinstructions_forwarded_persec.as_ref()
    }

    /// Sets the value of IOInstructionsForwardingCost
    pub fn set_ioinstructions_forwarding_cost(&mut self, value: u64) {
        self.ioinstructions_forwarding_cost = Some(value);
    }

    /// Gets the value of IOInstructionsForwardingCost
    pub fn get_ioinstructions_forwarding_cost(&self) -> Option<&u64> {
        self.ioinstructions_forwarding_cost.as_ref()
    }

    /// Sets the value of IOInstructionsForwardingCost_Base
    pub fn set_ioinstructions_forwarding_cost__base(&mut self, value: u64) {
        self.ioinstructions_forwarding_cost__base = Some(value);
    }

    /// Gets the value of IOInstructionsForwardingCost_Base
    pub fn get_ioinstructions_forwarding_cost__base(&self) -> Option<&u64> {
        self.ioinstructions_forwarding_cost__base.as_ref()
    }

    /// Sets the value of IOInstructionsPersec
    pub fn set_ioinstructions_persec(&mut self, value: u64) {
        self.ioinstructions_persec = Some(value);
    }

    /// Gets the value of IOInstructionsPersec
    pub fn get_ioinstructions_persec(&self) -> Option<&u64> {
        self.ioinstructions_persec.as_ref()
    }

    /// Sets the value of IOInterceptMessagesPersec
    pub fn set_iointercept_messages_persec(&mut self, value: u64) {
        self.iointercept_messages_persec = Some(value);
    }

    /// Gets the value of IOInterceptMessagesPersec
    pub fn get_iointercept_messages_persec(&self) -> Option<&u64> {
        self.iointercept_messages_persec.as_ref()
    }

    /// Sets the value of IOMMUHypercallsPersec
    pub fn set_iommuhypercalls_persec(&mut self, value: u64) {
        self.iommuhypercalls_persec = Some(value);
    }

    /// Gets the value of IOMMUHypercallsPersec
    pub fn get_iommuhypercalls_persec(&self) -> Option<&u64> {
        self.iommuhypercalls_persec.as_ref()
    }

    /// Sets the value of LargePageTLBFillsPersec
    pub fn set_large_page_tlbfills_persec(&mut self, value: u64) {
        self.large_page_tlbfills_persec = Some(value);
    }

    /// Gets the value of LargePageTLBFillsPersec
    pub fn get_large_page_tlbfills_persec(&self) -> Option<&u64> {
        self.large_page_tlbfills_persec.as_ref()
    }

    /// Sets the value of LocalFlushedGVARangesPersec
    pub fn set_local_flushed_gvaranges_persec(&mut self, value: u64) {
        self.local_flushed_gvaranges_persec = Some(value);
    }

    /// Gets the value of LocalFlushedGVARangesPersec
    pub fn get_local_flushed_gvaranges_persec(&self) -> Option<&u64> {
        self.local_flushed_gvaranges_persec.as_ref()
    }

    /// Sets the value of LocalIOTLBFlushCost
    pub fn set_local_iotlbflush_cost(&mut self, value: u64) {
        self.local_iotlbflush_cost = Some(value);
    }

    /// Gets the value of LocalIOTLBFlushCost
    pub fn get_local_iotlbflush_cost(&self) -> Option<&u64> {
        self.local_iotlbflush_cost.as_ref()
    }

    /// Sets the value of LocalIOTLBFlushCost_Base
    pub fn set_local_iotlbflush_cost__base(&mut self, value: u64) {
        self.local_iotlbflush_cost__base = Some(value);
    }

    /// Gets the value of LocalIOTLBFlushCost_Base
    pub fn get_local_iotlbflush_cost__base(&self) -> Option<&u64> {
        self.local_iotlbflush_cost__base.as_ref()
    }

    /// Sets the value of LocalIOTLBFlushesPersec
    pub fn set_local_iotlbflushes_persec(&mut self, value: u64) {
        self.local_iotlbflushes_persec = Some(value);
    }

    /// Gets the value of LocalIOTLBFlushesPersec
    pub fn get_local_iotlbflushes_persec(&self) -> Option<&u64> {
        self.local_iotlbflushes_persec.as_ref()
    }

    /// Sets the value of LogicalProcessorDispatchesPersec
    pub fn set_logical_processor_dispatches_persec(&mut self, value: u64) {
        self.logical_processor_dispatches_persec = Some(value);
    }

    /// Gets the value of LogicalProcessorDispatchesPersec
    pub fn get_logical_processor_dispatches_persec(&self) -> Option<&u64> {
        self.logical_processor_dispatches_persec.as_ref()
    }

    /// Sets the value of LogicalProcessorHypercallsPersec
    pub fn set_logical_processor_hypercalls_persec(&mut self, value: u64) {
        self.logical_processor_hypercalls_persec = Some(value);
    }

    /// Gets the value of LogicalProcessorHypercallsPersec
    pub fn get_logical_processor_hypercalls_persec(&self) -> Option<&u64> {
        self.logical_processor_hypercalls_persec.as_ref()
    }

    /// Sets the value of LogicalProcessorMigrationsPersec
    pub fn set_logical_processor_migrations_persec(&mut self, value: u64) {
        self.logical_processor_migrations_persec = Some(value);
    }

    /// Gets the value of LogicalProcessorMigrationsPersec
    pub fn get_logical_processor_migrations_persec(&self) -> Option<&u64> {
        self.logical_processor_migrations_persec.as_ref()
    }

    /// Sets the value of LongSpinWaitHypercallsPersec
    pub fn set_long_spin_wait_hypercalls_persec(&mut self, value: u64) {
        self.long_spin_wait_hypercalls_persec = Some(value);
    }

    /// Gets the value of LongSpinWaitHypercallsPersec
    pub fn get_long_spin_wait_hypercalls_persec(&self) -> Option<&u64> {
        self.long_spin_wait_hypercalls_persec.as_ref()
    }

    /// Sets the value of MBECNestedPageTableSwitchesPersec
    pub fn set_mbecnested_page_table_switches_persec(&mut self, value: u64) {
        self.mbecnested_page_table_switches_persec = Some(value);
    }

    /// Gets the value of MBECNestedPageTableSwitchesPersec
    pub fn get_mbecnested_page_table_switches_persec(&self) -> Option<&u64> {
        self.mbecnested_page_table_switches_persec.as_ref()
    }

    /// Sets the value of MemoryInterceptMessagesPersec
    pub fn set_memory_intercept_messages_persec(&mut self, value: u64) {
        self.memory_intercept_messages_persec = Some(value);
    }

    /// Gets the value of MemoryInterceptMessagesPersec
    pub fn get_memory_intercept_messages_persec(&self) -> Option<&u64> {
        self.memory_intercept_messages_persec.as_ref()
    }

    /// Sets the value of MSRAccessesCost
    pub fn set_msraccesses_cost(&mut self, value: u64) {
        self.msraccesses_cost = Some(value);
    }

    /// Gets the value of MSRAccessesCost
    pub fn get_msraccesses_cost(&self) -> Option<&u64> {
        self.msraccesses_cost.as_ref()
    }

    /// Sets the value of MSRAccessesCost_Base
    pub fn set_msraccesses_cost__base(&mut self, value: u64) {
        self.msraccesses_cost__base = Some(value);
    }

    /// Gets the value of MSRAccessesCost_Base
    pub fn get_msraccesses_cost__base(&self) -> Option<&u64> {
        self.msraccesses_cost__base.as_ref()
    }

    /// Sets the value of MSRAccessesForwardedPersec
    pub fn set_msraccesses_forwarded_persec(&mut self, value: u64) {
        self.msraccesses_forwarded_persec = Some(value);
    }

    /// Gets the value of MSRAccessesForwardedPersec
    pub fn get_msraccesses_forwarded_persec(&self) -> Option<&u64> {
        self.msraccesses_forwarded_persec.as_ref()
    }

    /// Sets the value of MSRAccessesForwardingCost
    pub fn set_msraccesses_forwarding_cost(&mut self, value: u64) {
        self.msraccesses_forwarding_cost = Some(value);
    }

    /// Gets the value of MSRAccessesForwardingCost
    pub fn get_msraccesses_forwarding_cost(&self) -> Option<&u64> {
        self.msraccesses_forwarding_cost.as_ref()
    }

    /// Sets the value of MSRAccessesForwardingCost_Base
    pub fn set_msraccesses_forwarding_cost__base(&mut self, value: u64) {
        self.msraccesses_forwarding_cost__base = Some(value);
    }

    /// Gets the value of MSRAccessesForwardingCost_Base
    pub fn get_msraccesses_forwarding_cost__base(&self) -> Option<&u64> {
        self.msraccesses_forwarding_cost__base.as_ref()
    }

    /// Sets the value of MSRAccessesPersec
    pub fn set_msraccesses_persec(&mut self, value: u64) {
        self.msraccesses_persec = Some(value);
    }

    /// Gets the value of MSRAccessesPersec
    pub fn get_msraccesses_persec(&self) -> Option<&u64> {
        self.msraccesses_persec.as_ref()
    }

    /// Sets the value of MWAITInstructionsCost
    pub fn set_mwaitinstructions_cost(&mut self, value: u64) {
        self.mwaitinstructions_cost = Some(value);
    }

    /// Gets the value of MWAITInstructionsCost
    pub fn get_mwaitinstructions_cost(&self) -> Option<&u64> {
        self.mwaitinstructions_cost.as_ref()
    }

    /// Sets the value of MWAITInstructionsCost_Base
    pub fn set_mwaitinstructions_cost__base(&mut self, value: u64) {
        self.mwaitinstructions_cost__base = Some(value);
    }

    /// Gets the value of MWAITInstructionsCost_Base
    pub fn get_mwaitinstructions_cost__base(&self) -> Option<&u64> {
        self.mwaitinstructions_cost__base.as_ref()
    }

    /// Sets the value of MWAITInstructionsForwardedPersec
    pub fn set_mwaitinstructions_forwarded_persec(&mut self, value: u64) {
        self.mwaitinstructions_forwarded_persec = Some(value);
    }

    /// Gets the value of MWAITInstructionsForwardedPersec
    pub fn get_mwaitinstructions_forwarded_persec(&self) -> Option<&u64> {
        self.mwaitinstructions_forwarded_persec.as_ref()
    }

    /// Sets the value of MWAITInstructionsForwardingCost
    pub fn set_mwaitinstructions_forwarding_cost(&mut self, value: u64) {
        self.mwaitinstructions_forwarding_cost = Some(value);
    }

    /// Gets the value of MWAITInstructionsForwardingCost
    pub fn get_mwaitinstructions_forwarding_cost(&self) -> Option<&u64> {
        self.mwaitinstructions_forwarding_cost.as_ref()
    }

    /// Sets the value of MWAITInstructionsForwardingCost_Base
    pub fn set_mwaitinstructions_forwarding_cost__base(&mut self, value: u64) {
        self.mwaitinstructions_forwarding_cost__base = Some(value);
    }

    /// Gets the value of MWAITInstructionsForwardingCost_Base
    pub fn get_mwaitinstructions_forwarding_cost__base(&self) -> Option<&u64> {
        self.mwaitinstructions_forwarding_cost__base.as_ref()
    }

    /// Sets the value of MWAITInstructionsPersec
    pub fn set_mwaitinstructions_persec(&mut self, value: u64) {
        self.mwaitinstructions_persec = Some(value);
    }

    /// Gets the value of MWAITInstructionsPersec
    pub fn get_mwaitinstructions_persec(&self) -> Option<&u64> {
        self.mwaitinstructions_persec.as_ref()
    }

    /// Sets the value of NestedPageFaultInterceptsCost
    pub fn set_nested_page_fault_intercepts_cost(&mut self, value: u64) {
        self.nested_page_fault_intercepts_cost = Some(value);
    }

    /// Gets the value of NestedPageFaultInterceptsCost
    pub fn get_nested_page_fault_intercepts_cost(&self) -> Option<&u64> {
        self.nested_page_fault_intercepts_cost.as_ref()
    }

    /// Sets the value of NestedPageFaultInterceptsCost_Base
    pub fn set_nested_page_fault_intercepts_cost__base(&mut self, value: u64) {
        self.nested_page_fault_intercepts_cost__base = Some(value);
    }

    /// Gets the value of NestedPageFaultInterceptsCost_Base
    pub fn get_nested_page_fault_intercepts_cost__base(&self) -> Option<&u64> {
        self.nested_page_fault_intercepts_cost__base.as_ref()
    }

    /// Sets the value of NestedPageFaultInterceptsPersec
    pub fn set_nested_page_fault_intercepts_persec(&mut self, value: u64) {
        self.nested_page_fault_intercepts_persec = Some(value);
    }

    /// Gets the value of NestedPageFaultInterceptsPersec
    pub fn get_nested_page_fault_intercepts_persec(&self) -> Option<&u64> {
        self.nested_page_fault_intercepts_persec.as_ref()
    }

    /// Sets the value of NestedSLATHardPageFaultsCost
    pub fn set_nested_slathard_page_faults_cost(&mut self, value: u64) {
        self.nested_slathard_page_faults_cost = Some(value);
    }

    /// Gets the value of NestedSLATHardPageFaultsCost
    pub fn get_nested_slathard_page_faults_cost(&self) -> Option<&u64> {
        self.nested_slathard_page_faults_cost.as_ref()
    }

    /// Sets the value of NestedSLATHardPageFaultsCost_Base
    pub fn set_nested_slathard_page_faults_cost__base(&mut self, value: u64) {
        self.nested_slathard_page_faults_cost__base = Some(value);
    }

    /// Gets the value of NestedSLATHardPageFaultsCost_Base
    pub fn get_nested_slathard_page_faults_cost__base(&self) -> Option<&u64> {
        self.nested_slathard_page_faults_cost__base.as_ref()
    }

    /// Sets the value of NestedSLATHardPageFaultsPersec
    pub fn set_nested_slathard_page_faults_persec(&mut self, value: u64) {
        self.nested_slathard_page_faults_persec = Some(value);
    }

    /// Gets the value of NestedSLATHardPageFaultsPersec
    pub fn get_nested_slathard_page_faults_persec(&self) -> Option<&u64> {
        self.nested_slathard_page_faults_persec.as_ref()
    }

    /// Sets the value of NestedSLATSoftPageFaultsCost
    pub fn set_nested_slatsoft_page_faults_cost(&mut self, value: u64) {
        self.nested_slatsoft_page_faults_cost = Some(value);
    }

    /// Gets the value of NestedSLATSoftPageFaultsCost
    pub fn get_nested_slatsoft_page_faults_cost(&self) -> Option<&u64> {
        self.nested_slatsoft_page_faults_cost.as_ref()
    }

    /// Sets the value of NestedSLATSoftPageFaultsCost_Base
    pub fn set_nested_slatsoft_page_faults_cost__base(&mut self, value: u64) {
        self.nested_slatsoft_page_faults_cost__base = Some(value);
    }

    /// Gets the value of NestedSLATSoftPageFaultsCost_Base
    pub fn get_nested_slatsoft_page_faults_cost__base(&self) -> Option<&u64> {
        self.nested_slatsoft_page_faults_cost__base.as_ref()
    }

    /// Sets the value of NestedSLATSoftPageFaultsPersec
    pub fn set_nested_slatsoft_page_faults_persec(&mut self, value: u64) {
        self.nested_slatsoft_page_faults_persec = Some(value);
    }

    /// Gets the value of NestedSLATSoftPageFaultsPersec
    pub fn get_nested_slatsoft_page_faults_persec(&self) -> Option<&u64> {
        self.nested_slatsoft_page_faults_persec.as_ref()
    }

    /// Sets the value of NestedTLBPageTableEvictionsPersec
    pub fn set_nested_tlbpage_table_evictions_persec(&mut self, value: u64) {
        self.nested_tlbpage_table_evictions_persec = Some(value);
    }

    /// Gets the value of NestedTLBPageTableEvictionsPersec
    pub fn get_nested_tlbpage_table_evictions_persec(&self) -> Option<&u64> {
        self.nested_tlbpage_table_evictions_persec.as_ref()
    }

    /// Sets the value of NestedTLBPageTableReclamationsPersec
    pub fn set_nested_tlbpage_table_reclamations_persec(&mut self, value: u64) {
        self.nested_tlbpage_table_reclamations_persec = Some(value);
    }

    /// Gets the value of NestedTLBPageTableReclamationsPersec
    pub fn get_nested_tlbpage_table_reclamations_persec(&self) -> Option<&u64> {
        self.nested_tlbpage_table_reclamations_persec.as_ref()
    }

    /// Sets the value of NestedVMEntriesCost
    pub fn set_nested_vmentries_cost(&mut self, value: u64) {
        self.nested_vmentries_cost = Some(value);
    }

    /// Gets the value of NestedVMEntriesCost
    pub fn get_nested_vmentries_cost(&self) -> Option<&u64> {
        self.nested_vmentries_cost.as_ref()
    }

    /// Sets the value of NestedVMEntriesCost_Base
    pub fn set_nested_vmentries_cost__base(&mut self, value: u64) {
        self.nested_vmentries_cost__base = Some(value);
    }

    /// Gets the value of NestedVMEntriesCost_Base
    pub fn get_nested_vmentries_cost__base(&self) -> Option<&u64> {
        self.nested_vmentries_cost__base.as_ref()
    }

    /// Sets the value of NestedVMEntriesPersec
    pub fn set_nested_vmentries_persec(&mut self, value: u64) {
        self.nested_vmentries_persec = Some(value);
    }

    /// Gets the value of NestedVMEntriesPersec
    pub fn get_nested_vmentries_persec(&self) -> Option<&u64> {
        self.nested_vmentries_persec.as_ref()
    }

    /// Sets the value of OtherHypercallsPersec
    pub fn set_other_hypercalls_persec(&mut self, value: u64) {
        self.other_hypercalls_persec = Some(value);
    }

    /// Gets the value of OtherHypercallsPersec
    pub fn get_other_hypercalls_persec(&self) -> Option<&u64> {
        self.other_hypercalls_persec.as_ref()
    }

    /// Sets the value of OtherInterceptsCost
    pub fn set_other_intercepts_cost(&mut self, value: u64) {
        self.other_intercepts_cost = Some(value);
    }

    /// Gets the value of OtherInterceptsCost
    pub fn get_other_intercepts_cost(&self) -> Option<&u64> {
        self.other_intercepts_cost.as_ref()
    }

    /// Sets the value of OtherInterceptsCost_Base
    pub fn set_other_intercepts_cost__base(&mut self, value: u64) {
        self.other_intercepts_cost__base = Some(value);
    }

    /// Gets the value of OtherInterceptsCost_Base
    pub fn get_other_intercepts_cost__base(&self) -> Option<&u64> {
        self.other_intercepts_cost__base.as_ref()
    }

    /// Sets the value of OtherInterceptsForwardedPersec
    pub fn set_other_intercepts_forwarded_persec(&mut self, value: u64) {
        self.other_intercepts_forwarded_persec = Some(value);
    }

    /// Gets the value of OtherInterceptsForwardedPersec
    pub fn get_other_intercepts_forwarded_persec(&self) -> Option<&u64> {
        self.other_intercepts_forwarded_persec.as_ref()
    }

    /// Sets the value of OtherInterceptsForwardingCost
    pub fn set_other_intercepts_forwarding_cost(&mut self, value: u64) {
        self.other_intercepts_forwarding_cost = Some(value);
    }

    /// Gets the value of OtherInterceptsForwardingCost
    pub fn get_other_intercepts_forwarding_cost(&self) -> Option<&u64> {
        self.other_intercepts_forwarding_cost.as_ref()
    }

    /// Sets the value of OtherInterceptsForwardingCost_Base
    pub fn set_other_intercepts_forwarding_cost__base(&mut self, value: u64) {
        self.other_intercepts_forwarding_cost__base = Some(value);
    }

    /// Gets the value of OtherInterceptsForwardingCost_Base
    pub fn get_other_intercepts_forwarding_cost__base(&self) -> Option<&u64> {
        self.other_intercepts_forwarding_cost__base.as_ref()
    }

    /// Sets the value of OtherInterceptsPersec
    pub fn set_other_intercepts_persec(&mut self, value: u64) {
        self.other_intercepts_persec = Some(value);
    }

    /// Gets the value of OtherInterceptsPersec
    pub fn get_other_intercepts_persec(&self) -> Option<&u64> {
        self.other_intercepts_persec.as_ref()
    }

    /// Sets the value of OtherMessagesPersec
    pub fn set_other_messages_persec(&mut self, value: u64) {
        self.other_messages_persec = Some(value);
    }

    /// Gets the value of OtherMessagesPersec
    pub fn get_other_messages_persec(&self) -> Option<&u64> {
        self.other_messages_persec.as_ref()
    }

    /// Sets the value of OtherReflectedGuestExceptionsPersec
    pub fn set_other_reflected_guest_exceptions_persec(&mut self, value: u64) {
        self.other_reflected_guest_exceptions_persec = Some(value);
    }

    /// Gets the value of OtherReflectedGuestExceptionsPersec
    pub fn get_other_reflected_guest_exceptions_persec(&self) -> Option<&u64> {
        self.other_reflected_guest_exceptions_persec.as_ref()
    }

    /// Sets the value of PageFaultInterceptsCost
    pub fn set_page_fault_intercepts_cost(&mut self, value: u64) {
        self.page_fault_intercepts_cost = Some(value);
    }

    /// Gets the value of PageFaultInterceptsCost
    pub fn get_page_fault_intercepts_cost(&self) -> Option<&u64> {
        self.page_fault_intercepts_cost.as_ref()
    }

    /// Sets the value of PageFaultInterceptsCost_Base
    pub fn set_page_fault_intercepts_cost__base(&mut self, value: u64) {
        self.page_fault_intercepts_cost__base = Some(value);
    }

    /// Gets the value of PageFaultInterceptsCost_Base
    pub fn get_page_fault_intercepts_cost__base(&self) -> Option<&u64> {
        self.page_fault_intercepts_cost__base.as_ref()
    }

    /// Sets the value of PageFaultInterceptsForwardedPersec
    pub fn set_page_fault_intercepts_forwarded_persec(&mut self, value: u64) {
        self.page_fault_intercepts_forwarded_persec = Some(value);
    }

    /// Gets the value of PageFaultInterceptsForwardedPersec
    pub fn get_page_fault_intercepts_forwarded_persec(&self) -> Option<&u64> {
        self.page_fault_intercepts_forwarded_persec.as_ref()
    }

    /// Sets the value of PageFaultInterceptsForwardingCost
    pub fn set_page_fault_intercepts_forwarding_cost(&mut self, value: u64) {
        self.page_fault_intercepts_forwarding_cost = Some(value);
    }

    /// Gets the value of PageFaultInterceptsForwardingCost
    pub fn get_page_fault_intercepts_forwarding_cost(&self) -> Option<&u64> {
        self.page_fault_intercepts_forwarding_cost.as_ref()
    }

    /// Sets the value of PageFaultInterceptsForwardingCost_Base
    pub fn set_page_fault_intercepts_forwarding_cost__base(&mut self, value: u64) {
        self.page_fault_intercepts_forwarding_cost__base = Some(value);
    }

    /// Gets the value of PageFaultInterceptsForwardingCost_Base
    pub fn get_page_fault_intercepts_forwarding_cost__base(&self) -> Option<&u64> {
        self.page_fault_intercepts_forwarding_cost__base.as_ref()
    }

    /// Sets the value of PageFaultInterceptsPersec
    pub fn set_page_fault_intercepts_persec(&mut self, value: u64) {
        self.page_fault_intercepts_persec = Some(value);
    }

    /// Gets the value of PageFaultInterceptsPersec
    pub fn get_page_fault_intercepts_persec(&self) -> Option<&u64> {
        self.page_fault_intercepts_persec.as_ref()
    }

    /// Sets the value of PageInvalidationsCost
    pub fn set_page_invalidations_cost(&mut self, value: u64) {
        self.page_invalidations_cost = Some(value);
    }

    /// Gets the value of PageInvalidationsCost
    pub fn get_page_invalidations_cost(&self) -> Option<&u64> {
        self.page_invalidations_cost.as_ref()
    }

    /// Sets the value of PageInvalidationsCost_Base
    pub fn set_page_invalidations_cost__base(&mut self, value: u64) {
        self.page_invalidations_cost__base = Some(value);
    }

    /// Gets the value of PageInvalidationsCost_Base
    pub fn get_page_invalidations_cost__base(&self) -> Option<&u64> {
        self.page_invalidations_cost__base.as_ref()
    }

    /// Sets the value of PageInvalidationsForwardedPersec
    pub fn set_page_invalidations_forwarded_persec(&mut self, value: u64) {
        self.page_invalidations_forwarded_persec = Some(value);
    }

    /// Gets the value of PageInvalidationsForwardedPersec
    pub fn get_page_invalidations_forwarded_persec(&self) -> Option<&u64> {
        self.page_invalidations_forwarded_persec.as_ref()
    }

    /// Sets the value of PageInvalidationsForwardingCost
    pub fn set_page_invalidations_forwarding_cost(&mut self, value: u64) {
        self.page_invalidations_forwarding_cost = Some(value);
    }

    /// Gets the value of PageInvalidationsForwardingCost
    pub fn get_page_invalidations_forwarding_cost(&self) -> Option<&u64> {
        self.page_invalidations_forwarding_cost.as_ref()
    }

    /// Sets the value of PageInvalidationsForwardingCost_Base
    pub fn set_page_invalidations_forwarding_cost__base(&mut self, value: u64) {
        self.page_invalidations_forwarding_cost__base = Some(value);
    }

    /// Gets the value of PageInvalidationsForwardingCost_Base
    pub fn get_page_invalidations_forwarding_cost__base(&self) -> Option<&u64> {
        self.page_invalidations_forwarding_cost__base.as_ref()
    }

    /// Sets the value of PageInvalidationsPersec
    pub fn set_page_invalidations_persec(&mut self, value: u64) {
        self.page_invalidations_persec = Some(value);
    }

    /// Gets the value of PageInvalidationsPersec
    pub fn get_page_invalidations_persec(&self) -> Option<&u64> {
        self.page_invalidations_persec.as_ref()
    }

    /// Sets the value of PageScansPersec
    pub fn set_page_scans_persec(&mut self, value: u64) {
        self.page_scans_persec = Some(value);
    }

    /// Gets the value of PageScansPersec
    pub fn get_page_scans_persec(&self) -> Option<&u64> {
        self.page_scans_persec.as_ref()
    }

    /// Sets the value of PageTableAllocationsPersec
    pub fn set_page_table_allocations_persec(&mut self, value: u64) {
        self.page_table_allocations_persec = Some(value);
    }

    /// Gets the value of PageTableAllocationsPersec
    pub fn get_page_table_allocations_persec(&self) -> Option<&u64> {
        self.page_table_allocations_persec.as_ref()
    }

    /// Sets the value of PageTableEvictionsPersec
    pub fn set_page_table_evictions_persec(&mut self, value: u64) {
        self.page_table_evictions_persec = Some(value);
    }

    /// Gets the value of PageTableEvictionsPersec
    pub fn get_page_table_evictions_persec(&self) -> Option<&u64> {
        self.page_table_evictions_persec.as_ref()
    }

    /// Sets the value of PageTableReclamationsPersec
    pub fn set_page_table_reclamations_persec(&mut self, value: u64) {
        self.page_table_reclamations_persec = Some(value);
    }

    /// Gets the value of PageTableReclamationsPersec
    pub fn get_page_table_reclamations_persec(&self) -> Option<&u64> {
        self.page_table_reclamations_persec.as_ref()
    }

    /// Sets the value of PageTableResetsPersec
    pub fn set_page_table_resets_persec(&mut self, value: u64) {
        self.page_table_resets_persec = Some(value);
    }

    /// Gets the value of PageTableResetsPersec
    pub fn get_page_table_resets_persec(&self) -> Option<&u64> {
        self.page_table_resets_persec.as_ref()
    }

    /// Sets the value of PageTableValidationsPersec
    pub fn set_page_table_validations_persec(&mut self, value: u64) {
        self.page_table_validations_persec = Some(value);
    }

    /// Gets the value of PageTableValidationsPersec
    pub fn get_page_table_validations_persec(&self) -> Option<&u64> {
        self.page_table_validations_persec.as_ref()
    }

    /// Sets the value of PageTableWriteInterceptsPersec
    pub fn set_page_table_write_intercepts_persec(&mut self, value: u64) {
        self.page_table_write_intercepts_persec = Some(value);
    }

    /// Gets the value of PageTableWriteInterceptsPersec
    pub fn get_page_table_write_intercepts_persec(&self) -> Option<&u64> {
        self.page_table_write_intercepts_persec.as_ref()
    }

    /// Sets the value of PendingInterruptsCost
    pub fn set_pending_interrupts_cost(&mut self, value: u64) {
        self.pending_interrupts_cost = Some(value);
    }

    /// Gets the value of PendingInterruptsCost
    pub fn get_pending_interrupts_cost(&self) -> Option<&u64> {
        self.pending_interrupts_cost.as_ref()
    }

    /// Sets the value of PendingInterruptsCost_Base
    pub fn set_pending_interrupts_cost__base(&mut self, value: u64) {
        self.pending_interrupts_cost__base = Some(value);
    }

    /// Gets the value of PendingInterruptsCost_Base
    pub fn get_pending_interrupts_cost__base(&self) -> Option<&u64> {
        self.pending_interrupts_cost__base.as_ref()
    }

    /// Sets the value of PendingInterruptsForwardedPersec
    pub fn set_pending_interrupts_forwarded_persec(&mut self, value: u64) {
        self.pending_interrupts_forwarded_persec = Some(value);
    }

    /// Gets the value of PendingInterruptsForwardedPersec
    pub fn get_pending_interrupts_forwarded_persec(&self) -> Option<&u64> {
        self.pending_interrupts_forwarded_persec.as_ref()
    }

    /// Sets the value of PendingInterruptsForwardingCost
    pub fn set_pending_interrupts_forwarding_cost(&mut self, value: u64) {
        self.pending_interrupts_forwarding_cost = Some(value);
    }

    /// Gets the value of PendingInterruptsForwardingCost
    pub fn get_pending_interrupts_forwarding_cost(&self) -> Option<&u64> {
        self.pending_interrupts_forwarding_cost.as_ref()
    }

    /// Sets the value of PendingInterruptsForwardingCost_Base
    pub fn set_pending_interrupts_forwarding_cost__base(&mut self, value: u64) {
        self.pending_interrupts_forwarding_cost__base = Some(value);
    }

    /// Gets the value of PendingInterruptsForwardingCost_Base
    pub fn get_pending_interrupts_forwarding_cost__base(&self) -> Option<&u64> {
        self.pending_interrupts_forwarding_cost__base.as_ref()
    }

    /// Sets the value of PendingInterruptsPersec
    pub fn set_pending_interrupts_persec(&mut self, value: u64) {
        self.pending_interrupts_persec = Some(value);
    }

    /// Gets the value of PendingInterruptsPersec
    pub fn get_pending_interrupts_persec(&self) -> Option<&u64> {
        self.pending_interrupts_persec.as_ref()
    }

    /// Sets the value of PercentGuestRelativeUtilization
    pub fn set_percent_guest_relative_utilization(&mut self, value: u64) {
        self.percent_guest_relative_utilization = Some(value);
    }

    /// Gets the value of PercentGuestRelativeUtilization
    pub fn get_percent_guest_relative_utilization(&self) -> Option<&u64> {
        self.percent_guest_relative_utilization.as_ref()
    }

    /// Sets the value of PercentGuestRelativeUtilization_Base
    pub fn set_percent_guest_relative_utilization__base(&mut self, value: u64) {
        self.percent_guest_relative_utilization__base = Some(value);
    }

    /// Gets the value of PercentGuestRelativeUtilization_Base
    pub fn get_percent_guest_relative_utilization__base(&self) -> Option<&u64> {
        self.percent_guest_relative_utilization__base.as_ref()
    }

    /// Sets the value of PercentGuestRunTime
    pub fn set_percent_guest_run_time(&mut self, value: u64) {
        self.percent_guest_run_time = Some(value);
    }

    /// Gets the value of PercentGuestRunTime
    pub fn get_percent_guest_run_time(&self) -> Option<&u64> {
        self.percent_guest_run_time.as_ref()
    }

    /// Sets the value of PercentGuestRunTime_Base
    pub fn set_percent_guest_run_time__base(&mut self, value: u64) {
        self.percent_guest_run_time__base = Some(value);
    }

    /// Gets the value of PercentGuestRunTime_Base
    pub fn get_percent_guest_run_time__base(&self) -> Option<&u64> {
        self.percent_guest_run_time__base.as_ref()
    }

    /// Sets the value of PercentHypervisorRunTime
    pub fn set_percent_hypervisor_run_time(&mut self, value: u64) {
        self.percent_hypervisor_run_time = Some(value);
    }

    /// Gets the value of PercentHypervisorRunTime
    pub fn get_percent_hypervisor_run_time(&self) -> Option<&u64> {
        self.percent_hypervisor_run_time.as_ref()
    }

    /// Sets the value of PercentHypervisorRunTime_Base
    pub fn set_percent_hypervisor_run_time__base(&mut self, value: u64) {
        self.percent_hypervisor_run_time__base = Some(value);
    }

    /// Gets the value of PercentHypervisorRunTime_Base
    pub fn get_percent_hypervisor_run_time__base(&self) -> Option<&u64> {
        self.percent_hypervisor_run_time__base.as_ref()
    }

    /// Sets the value of PercentRemoteRunTime
    pub fn set_percent_remote_run_time(&mut self, value: u64) {
        self.percent_remote_run_time = Some(value);
    }

    /// Gets the value of PercentRemoteRunTime
    pub fn get_percent_remote_run_time(&self) -> Option<&u64> {
        self.percent_remote_run_time.as_ref()
    }

    /// Sets the value of PercentRemoteRunTime_Base
    pub fn set_percent_remote_run_time__base(&mut self, value: u64) {
        self.percent_remote_run_time__base = Some(value);
    }

    /// Gets the value of PercentRemoteRunTime_Base
    pub fn get_percent_remote_run_time__base(&self) -> Option<&u64> {
        self.percent_remote_run_time__base.as_ref()
    }

    /// Sets the value of PercentTotalCoreRunTime
    pub fn set_percent_total_core_run_time(&mut self, value: u64) {
        self.percent_total_core_run_time = Some(value);
    }

    /// Gets the value of PercentTotalCoreRunTime
    pub fn get_percent_total_core_run_time(&self) -> Option<&u64> {
        self.percent_total_core_run_time.as_ref()
    }

    /// Sets the value of PercentTotalCoreRunTime_Base
    pub fn set_percent_total_core_run_time__base(&mut self, value: u64) {
        self.percent_total_core_run_time__base = Some(value);
    }

    /// Gets the value of PercentTotalCoreRunTime_Base
    pub fn get_percent_total_core_run_time__base(&self) -> Option<&u64> {
        self.percent_total_core_run_time__base.as_ref()
    }

    /// Sets the value of PercentTotalRunTime
    pub fn set_percent_total_run_time(&mut self, value: u64) {
        self.percent_total_run_time = Some(value);
    }

    /// Gets the value of PercentTotalRunTime
    pub fn get_percent_total_run_time(&self) -> Option<&u64> {
        self.percent_total_run_time.as_ref()
    }

    /// Sets the value of PercentTotalRunTime_Base
    pub fn set_percent_total_run_time__base(&mut self, value: u64) {
        self.percent_total_run_time__base = Some(value);
    }

    /// Gets the value of PercentTotalRunTime_Base
    pub fn get_percent_total_run_time__base(&self) -> Option<&u64> {
        self.percent_total_run_time__base.as_ref()
    }

    /// Sets the value of PercentVTL1RunTime
    pub fn set_percent_vtl1_run_time(&mut self, value: u64) {
        self.percent_vtl1_run_time = Some(value);
    }

    /// Gets the value of PercentVTL1RunTime
    pub fn get_percent_vtl1_run_time(&self) -> Option<&u64> {
        self.percent_vtl1_run_time.as_ref()
    }

    /// Sets the value of PercentVTL1RunTime_Base
    pub fn set_percent_vtl1_run_time__base(&mut self, value: u64) {
        self.percent_vtl1_run_time__base = Some(value);
    }

    /// Gets the value of PercentVTL1RunTime_Base
    pub fn get_percent_vtl1_run_time__base(&self) -> Option<&u64> {
        self.percent_vtl1_run_time__base.as_ref()
    }

    /// Sets the value of PercentVTL2RunTime
    pub fn set_percent_vtl2_run_time(&mut self, value: u64) {
        self.percent_vtl2_run_time = Some(value);
    }

    /// Gets the value of PercentVTL2RunTime
    pub fn get_percent_vtl2_run_time(&self) -> Option<&u64> {
        self.percent_vtl2_run_time.as_ref()
    }

    /// Sets the value of PercentVTL2RunTime_Base
    pub fn set_percent_vtl2_run_time__base(&mut self, value: u64) {
        self.percent_vtl2_run_time__base = Some(value);
    }

    /// Gets the value of PercentVTL2RunTime_Base
    pub fn get_percent_vtl2_run_time__base(&self) -> Option<&u64> {
        self.percent_vtl2_run_time__base.as_ref()
    }

    /// Sets the value of PerformanceMonitoringInterruptsPersec
    pub fn set_performance_monitoring_interrupts_persec(&mut self, value: u64) {
        self.performance_monitoring_interrupts_persec = Some(value);
    }

    /// Gets the value of PerformanceMonitoringInterruptsPersec
    pub fn get_performance_monitoring_interrupts_persec(&self) -> Option<&u64> {
        self.performance_monitoring_interrupts_persec.as_ref()
    }

    /// Sets the value of PerformanceMonitoringIPTMSRAccessesPersec
    pub fn set_performance_monitoring_iptmsraccesses_persec(&mut self, value: u64) {
        self.performance_monitoring_iptmsraccesses_persec = Some(value);
    }

    /// Gets the value of PerformanceMonitoringIPTMSRAccessesPersec
    pub fn get_performance_monitoring_iptmsraccesses_persec(&self) -> Option<&u64> {
        self.performance_monitoring_iptmsraccesses_persec.as_ref()
    }

    /// Sets the value of PerformanceMonitoringLBRMSRAccessesPersec
    pub fn set_performance_monitoring_lbrmsraccesses_persec(&mut self, value: u64) {
        self.performance_monitoring_lbrmsraccesses_persec = Some(value);
    }

    /// Gets the value of PerformanceMonitoringLBRMSRAccessesPersec
    pub fn get_performance_monitoring_lbrmsraccesses_persec(&self) -> Option<&u64> {
        self.performance_monitoring_lbrmsraccesses_persec.as_ref()
    }

    /// Sets the value of PerformanceMonitoringvPMUMSRAccessesPersec
    pub fn set_performance_monitoringv_pmumsraccesses_persec(&mut self, value: u64) {
        self.performance_monitoringv_pmumsraccesses_persec = Some(value);
    }

    /// Gets the value of PerformanceMonitoringvPMUMSRAccessesPersec
    pub fn get_performance_monitoringv_pmumsraccesses_persec(&self) -> Option<&u64> {
        self.performance_monitoringv_pmumsraccesses_persec.as_ref()
    }

    /// Sets the value of PostedInterruptNotificationsPersec
    pub fn set_posted_interrupt_notifications_persec(&mut self, value: u64) {
        self.posted_interrupt_notifications_persec = Some(value);
    }

    /// Gets the value of PostedInterruptNotificationsPersec
    pub fn get_posted_interrupt_notifications_persec(&self) -> Option<&u64> {
        self.posted_interrupt_notifications_persec.as_ref()
    }

    /// Sets the value of PostedInterruptScansPersec
    pub fn set_posted_interrupt_scans_persec(&mut self, value: u64) {
        self.posted_interrupt_scans_persec = Some(value);
    }

    /// Gets the value of PostedInterruptScansPersec
    pub fn get_posted_interrupt_scans_persec(&self) -> Option<&u64> {
        self.posted_interrupt_scans_persec.as_ref()
    }

    /// Sets the value of RDPMCInstructionsCost
    pub fn set_rdpmcinstructions_cost(&mut self, value: u64) {
        self.rdpmcinstructions_cost = Some(value);
    }

    /// Gets the value of RDPMCInstructionsCost
    pub fn get_rdpmcinstructions_cost(&self) -> Option<&u64> {
        self.rdpmcinstructions_cost.as_ref()
    }

    /// Sets the value of RDPMCInstructionsCost_Base
    pub fn set_rdpmcinstructions_cost__base(&mut self, value: u64) {
        self.rdpmcinstructions_cost__base = Some(value);
    }

    /// Gets the value of RDPMCInstructionsCost_Base
    pub fn get_rdpmcinstructions_cost__base(&self) -> Option<&u64> {
        self.rdpmcinstructions_cost__base.as_ref()
    }

    /// Sets the value of RDPMCInstructionsPersec
    pub fn set_rdpmcinstructions_persec(&mut self, value: u64) {
        self.rdpmcinstructions_persec = Some(value);
    }

    /// Gets the value of RDPMCInstructionsPersec
    pub fn get_rdpmcinstructions_persec(&self) -> Option<&u64> {
        self.rdpmcinstructions_persec.as_ref()
    }

    /// Sets the value of ReflectedGuestPageFaultsPersec
    pub fn set_reflected_guest_page_faults_persec(&mut self, value: u64) {
        self.reflected_guest_page_faults_persec = Some(value);
    }

    /// Gets the value of ReflectedGuestPageFaultsPersec
    pub fn get_reflected_guest_page_faults_persec(&self) -> Option<&u64> {
        self.reflected_guest_page_faults_persec.as_ref()
    }

    /// Sets the value of SchedulingPriority
    pub fn set_scheduling_priority(&mut self, value: u64) {
        self.scheduling_priority = Some(value);
    }

    /// Gets the value of SchedulingPriority
    pub fn get_scheduling_priority(&self) -> Option<&u64> {
        self.scheduling_priority.as_ref()
    }

    /// Sets the value of SmallPageTLBFillsPersec
    pub fn set_small_page_tlbfills_persec(&mut self, value: u64) {
        self.small_page_tlbfills_persec = Some(value);
    }

    /// Gets the value of SmallPageTLBFillsPersec
    pub fn get_small_page_tlbfills_persec(&self) -> Option<&u64> {
        self.small_page_tlbfills_persec.as_ref()
    }

    /// Sets the value of SVMHypercallsPersec
    pub fn set_svmhypercalls_persec(&mut self, value: u64) {
        self.svmhypercalls_persec = Some(value);
    }

    /// Gets the value of SVMHypercallsPersec
    pub fn get_svmhypercalls_persec(&self) -> Option<&u64> {
        self.svmhypercalls_persec.as_ref()
    }

    /// Sets the value of SyntheticInterruptHypercallsPersec
    pub fn set_synthetic_interrupt_hypercalls_persec(&mut self, value: u64) {
        self.synthetic_interrupt_hypercalls_persec = Some(value);
    }

    /// Gets the value of SyntheticInterruptHypercallsPersec
    pub fn get_synthetic_interrupt_hypercalls_persec(&self) -> Option<&u64> {
        self.synthetic_interrupt_hypercalls_persec.as_ref()
    }

    /// Sets the value of SyntheticInterruptsPersec
    pub fn set_synthetic_interrupts_persec(&mut self, value: u64) {
        self.synthetic_interrupts_persec = Some(value);
    }

    /// Gets the value of SyntheticInterruptsPersec
    pub fn get_synthetic_interrupts_persec(&self) -> Option<&u64> {
        self.synthetic_interrupts_persec.as_ref()
    }

    /// Sets the value of TotalInterceptsCost
    pub fn set_total_intercepts_cost(&mut self, value: u64) {
        self.total_intercepts_cost = Some(value);
    }

    /// Gets the value of TotalInterceptsCost
    pub fn get_total_intercepts_cost(&self) -> Option<&u64> {
        self.total_intercepts_cost.as_ref()
    }

    /// Sets the value of TotalInterceptsCost_Base
    pub fn set_total_intercepts_cost__base(&mut self, value: u64) {
        self.total_intercepts_cost__base = Some(value);
    }

    /// Gets the value of TotalInterceptsCost_Base
    pub fn get_total_intercepts_cost__base(&self) -> Option<&u64> {
        self.total_intercepts_cost__base.as_ref()
    }

    /// Sets the value of TotalInterceptsPersec
    pub fn set_total_intercepts_persec(&mut self, value: u64) {
        self.total_intercepts_persec = Some(value);
    }

    /// Gets the value of TotalInterceptsPersec
    pub fn get_total_intercepts_persec(&self) -> Option<&u64> {
        self.total_intercepts_persec.as_ref()
    }

    /// Sets the value of TotalMessagesPersec
    pub fn set_total_messages_persec(&mut self, value: u64) {
        self.total_messages_persec = Some(value);
    }

    /// Gets the value of TotalMessagesPersec
    pub fn get_total_messages_persec(&self) -> Option<&u64> {
        self.total_messages_persec.as_ref()
    }

    /// Sets the value of TotalVirtualizationInstructionsEmulatedPersec
    pub fn set_total_virtualization_instructions_emulated_persec(&mut self, value: u64) {
        self.total_virtualization_instructions_emulated_persec = Some(value);
    }

    /// Gets the value of TotalVirtualizationInstructionsEmulatedPersec
    pub fn get_total_virtualization_instructions_emulated_persec(&self) -> Option<&u64> {
        self.total_virtualization_instructions_emulated_persec.as_ref()
    }

    /// Sets the value of TotalVirtualizationInstructionsEmulationCost
    pub fn set_total_virtualization_instructions_emulation_cost(&mut self, value: u64) {
        self.total_virtualization_instructions_emulation_cost = Some(value);
    }

    /// Gets the value of TotalVirtualizationInstructionsEmulationCost
    pub fn get_total_virtualization_instructions_emulation_cost(&self) -> Option<&u64> {
        self.total_virtualization_instructions_emulation_cost.as_ref()
    }

    /// Sets the value of TotalVirtualizationInstructionsEmulationCost_Base
    pub fn set_total_virtualization_instructions_emulation_cost__base(&mut self, value: u64) {
        self.total_virtualization_instructions_emulation_cost__base = Some(value);
    }

    /// Gets the value of TotalVirtualizationInstructionsEmulationCost_Base
    pub fn get_total_virtualization_instructions_emulation_cost__base(&self) -> Option<&u64> {
        self.total_virtualization_instructions_emulation_cost__base.as_ref()
    }

    /// Sets the value of VirtualInterruptHypercallsPersec
    pub fn set_virtual_interrupt_hypercalls_persec(&mut self, value: u64) {
        self.virtual_interrupt_hypercalls_persec = Some(value);
    }

    /// Gets the value of VirtualInterruptHypercallsPersec
    pub fn get_virtual_interrupt_hypercalls_persec(&self) -> Option<&u64> {
        self.virtual_interrupt_hypercalls_persec.as_ref()
    }

    /// Sets the value of VirtualInterruptsPersec
    pub fn set_virtual_interrupts_persec(&mut self, value: u64) {
        self.virtual_interrupts_persec = Some(value);
    }

    /// Gets the value of VirtualInterruptsPersec
    pub fn get_virtual_interrupts_persec(&self) -> Option<&u64> {
        self.virtual_interrupts_persec.as_ref()
    }

    /// Sets the value of VirtualMMUHypercallsPersec
    pub fn set_virtual_mmuhypercalls_persec(&mut self, value: u64) {
        self.virtual_mmuhypercalls_persec = Some(value);
    }

    /// Gets the value of VirtualMMUHypercallsPersec
    pub fn get_virtual_mmuhypercalls_persec(&self) -> Option<&u64> {
        self.virtual_mmuhypercalls_persec.as_ref()
    }

    /// Sets the value of VirtualProcessorHypercallsPersec
    pub fn set_virtual_processor_hypercalls_persec(&mut self, value: u64) {
        self.virtual_processor_hypercalls_persec = Some(value);
    }

    /// Gets the value of VirtualProcessorHypercallsPersec
    pub fn get_virtual_processor_hypercalls_persec(&self) -> Option<&u64> {
        self.virtual_processor_hypercalls_persec.as_ref()
    }

    /// Sets the value of VMCLEAREmulationInterceptsPersec
    pub fn set_vmclearemulation_intercepts_persec(&mut self, value: u64) {
        self.vmclearemulation_intercepts_persec = Some(value);
    }

    /// Gets the value of VMCLEAREmulationInterceptsPersec
    pub fn get_vmclearemulation_intercepts_persec(&self) -> Option<&u64> {
        self.vmclearemulation_intercepts_persec.as_ref()
    }

    /// Sets the value of VMCLEARInstructionEmulationCost
    pub fn set_vmclearinstruction_emulation_cost(&mut self, value: u64) {
        self.vmclearinstruction_emulation_cost = Some(value);
    }

    /// Gets the value of VMCLEARInstructionEmulationCost
    pub fn get_vmclearinstruction_emulation_cost(&self) -> Option<&u64> {
        self.vmclearinstruction_emulation_cost.as_ref()
    }

    /// Sets the value of VMCLEARInstructionEmulationCost_Base
    pub fn set_vmclearinstruction_emulation_cost__base(&mut self, value: u64) {
        self.vmclearinstruction_emulation_cost__base = Some(value);
    }

    /// Gets the value of VMCLEARInstructionEmulationCost_Base
    pub fn get_vmclearinstruction_emulation_cost__base(&self) -> Option<&u64> {
        self.vmclearinstruction_emulation_cost__base.as_ref()
    }

    /// Sets the value of VMLOADEmulationInterceptsPersec
    pub fn set_vmloademulation_intercepts_persec(&mut self, value: u64) {
        self.vmloademulation_intercepts_persec = Some(value);
    }

    /// Gets the value of VMLOADEmulationInterceptsPersec
    pub fn get_vmloademulation_intercepts_persec(&self) -> Option<&u64> {
        self.vmloademulation_intercepts_persec.as_ref()
    }

    /// Sets the value of VMLOADInstructionEmulationCost
    pub fn set_vmloadinstruction_emulation_cost(&mut self, value: u64) {
        self.vmloadinstruction_emulation_cost = Some(value);
    }

    /// Gets the value of VMLOADInstructionEmulationCost
    pub fn get_vmloadinstruction_emulation_cost(&self) -> Option<&u64> {
        self.vmloadinstruction_emulation_cost.as_ref()
    }

    /// Sets the value of VMLOADInstructionEmulationCost_Base
    pub fn set_vmloadinstruction_emulation_cost__base(&mut self, value: u64) {
        self.vmloadinstruction_emulation_cost__base = Some(value);
    }

    /// Gets the value of VMLOADInstructionEmulationCost_Base
    pub fn get_vmloadinstruction_emulation_cost__base(&self) -> Option<&u64> {
        self.vmloadinstruction_emulation_cost__base.as_ref()
    }

    /// Sets the value of VMPTRLDEmulationInterceptsPersec
    pub fn set_vmptrldemulation_intercepts_persec(&mut self, value: u64) {
        self.vmptrldemulation_intercepts_persec = Some(value);
    }

    /// Gets the value of VMPTRLDEmulationInterceptsPersec
    pub fn get_vmptrldemulation_intercepts_persec(&self) -> Option<&u64> {
        self.vmptrldemulation_intercepts_persec.as_ref()
    }

    /// Sets the value of VMPTRLDInstructionEmulationCost
    pub fn set_vmptrldinstruction_emulation_cost(&mut self, value: u64) {
        self.vmptrldinstruction_emulation_cost = Some(value);
    }

    /// Gets the value of VMPTRLDInstructionEmulationCost
    pub fn get_vmptrldinstruction_emulation_cost(&self) -> Option<&u64> {
        self.vmptrldinstruction_emulation_cost.as_ref()
    }

    /// Sets the value of VMPTRLDInstructionEmulationCost_Base
    pub fn set_vmptrldinstruction_emulation_cost__base(&mut self, value: u64) {
        self.vmptrldinstruction_emulation_cost__base = Some(value);
    }

    /// Gets the value of VMPTRLDInstructionEmulationCost_Base
    pub fn get_vmptrldinstruction_emulation_cost__base(&self) -> Option<&u64> {
        self.vmptrldinstruction_emulation_cost__base.as_ref()
    }

    /// Sets the value of VMPTRSTEmulationInterceptsPersec
    pub fn set_vmptrstemulation_intercepts_persec(&mut self, value: u64) {
        self.vmptrstemulation_intercepts_persec = Some(value);
    }

    /// Gets the value of VMPTRSTEmulationInterceptsPersec
    pub fn get_vmptrstemulation_intercepts_persec(&self) -> Option<&u64> {
        self.vmptrstemulation_intercepts_persec.as_ref()
    }

    /// Sets the value of VMPTRSTInstructionEmulationCost
    pub fn set_vmptrstinstruction_emulation_cost(&mut self, value: u64) {
        self.vmptrstinstruction_emulation_cost = Some(value);
    }

    /// Gets the value of VMPTRSTInstructionEmulationCost
    pub fn get_vmptrstinstruction_emulation_cost(&self) -> Option<&u64> {
        self.vmptrstinstruction_emulation_cost.as_ref()
    }

    /// Sets the value of VMPTRSTInstructionEmulationCost_Base
    pub fn set_vmptrstinstruction_emulation_cost__base(&mut self, value: u64) {
        self.vmptrstinstruction_emulation_cost__base = Some(value);
    }

    /// Gets the value of VMPTRSTInstructionEmulationCost_Base
    pub fn get_vmptrstinstruction_emulation_cost__base(&self) -> Option<&u64> {
        self.vmptrstinstruction_emulation_cost__base.as_ref()
    }

    /// Sets the value of VMREADEmulationInterceptsPersec
    pub fn set_vmreademulation_intercepts_persec(&mut self, value: u64) {
        self.vmreademulation_intercepts_persec = Some(value);
    }

    /// Gets the value of VMREADEmulationInterceptsPersec
    pub fn get_vmreademulation_intercepts_persec(&self) -> Option<&u64> {
        self.vmreademulation_intercepts_persec.as_ref()
    }

    /// Sets the value of VMREADInstructionEmulationCost
    pub fn set_vmreadinstruction_emulation_cost(&mut self, value: u64) {
        self.vmreadinstruction_emulation_cost = Some(value);
    }

    /// Gets the value of VMREADInstructionEmulationCost
    pub fn get_vmreadinstruction_emulation_cost(&self) -> Option<&u64> {
        self.vmreadinstruction_emulation_cost.as_ref()
    }

    /// Sets the value of VMREADInstructionEmulationCost_Base
    pub fn set_vmreadinstruction_emulation_cost__base(&mut self, value: u64) {
        self.vmreadinstruction_emulation_cost__base = Some(value);
    }

    /// Gets the value of VMREADInstructionEmulationCost_Base
    pub fn get_vmreadinstruction_emulation_cost__base(&self) -> Option<&u64> {
        self.vmreadinstruction_emulation_cost__base.as_ref()
    }

    /// Sets the value of VMSAVEEmulationInterceptsPersec
    pub fn set_vmsaveemulation_intercepts_persec(&mut self, value: u64) {
        self.vmsaveemulation_intercepts_persec = Some(value);
    }

    /// Gets the value of VMSAVEEmulationInterceptsPersec
    pub fn get_vmsaveemulation_intercepts_persec(&self) -> Option<&u64> {
        self.vmsaveemulation_intercepts_persec.as_ref()
    }

    /// Sets the value of VMSAVEInstructionEmulationCost
    pub fn set_vmsaveinstruction_emulation_cost(&mut self, value: u64) {
        self.vmsaveinstruction_emulation_cost = Some(value);
    }

    /// Gets the value of VMSAVEInstructionEmulationCost
    pub fn get_vmsaveinstruction_emulation_cost(&self) -> Option<&u64> {
        self.vmsaveinstruction_emulation_cost.as_ref()
    }

    /// Sets the value of VMSAVEInstructionEmulationCost_Base
    pub fn set_vmsaveinstruction_emulation_cost__base(&mut self, value: u64) {
        self.vmsaveinstruction_emulation_cost__base = Some(value);
    }

    /// Gets the value of VMSAVEInstructionEmulationCost_Base
    pub fn get_vmsaveinstruction_emulation_cost__base(&self) -> Option<&u64> {
        self.vmsaveinstruction_emulation_cost__base.as_ref()
    }

    /// Sets the value of VMWRITEEmulationInterceptsPersec
    pub fn set_vmwriteemulation_intercepts_persec(&mut self, value: u64) {
        self.vmwriteemulation_intercepts_persec = Some(value);
    }

    /// Gets the value of VMWRITEEmulationInterceptsPersec
    pub fn get_vmwriteemulation_intercepts_persec(&self) -> Option<&u64> {
        self.vmwriteemulation_intercepts_persec.as_ref()
    }

    /// Sets the value of VMWRITEInstructionEmulationCost
    pub fn set_vmwriteinstruction_emulation_cost(&mut self, value: u64) {
        self.vmwriteinstruction_emulation_cost = Some(value);
    }

    /// Gets the value of VMWRITEInstructionEmulationCost
    pub fn get_vmwriteinstruction_emulation_cost(&self) -> Option<&u64> {
        self.vmwriteinstruction_emulation_cost.as_ref()
    }

    /// Sets the value of VMWRITEInstructionEmulationCost_Base
    pub fn set_vmwriteinstruction_emulation_cost__base(&mut self, value: u64) {
        self.vmwriteinstruction_emulation_cost__base = Some(value);
    }

    /// Gets the value of VMWRITEInstructionEmulationCost_Base
    pub fn get_vmwriteinstruction_emulation_cost__base(&self) -> Option<&u64> {
        self.vmwriteinstruction_emulation_cost__base.as_ref()
    }

    /// Sets the value of VMXOFFEmulationInterceptsPersec
    pub fn set_vmxoffemulation_intercepts_persec(&mut self, value: u64) {
        self.vmxoffemulation_intercepts_persec = Some(value);
    }

    /// Gets the value of VMXOFFEmulationInterceptsPersec
    pub fn get_vmxoffemulation_intercepts_persec(&self) -> Option<&u64> {
        self.vmxoffemulation_intercepts_persec.as_ref()
    }

    /// Sets the value of VMXOFFInstructionEmulationCost
    pub fn set_vmxoffinstruction_emulation_cost(&mut self, value: u64) {
        self.vmxoffinstruction_emulation_cost = Some(value);
    }

    /// Gets the value of VMXOFFInstructionEmulationCost
    pub fn get_vmxoffinstruction_emulation_cost(&self) -> Option<&u64> {
        self.vmxoffinstruction_emulation_cost.as_ref()
    }

    /// Sets the value of VMXOFFInstructionEmulationCost_Base
    pub fn set_vmxoffinstruction_emulation_cost__base(&mut self, value: u64) {
        self.vmxoffinstruction_emulation_cost__base = Some(value);
    }

    /// Gets the value of VMXOFFInstructionEmulationCost_Base
    pub fn get_vmxoffinstruction_emulation_cost__base(&self) -> Option<&u64> {
        self.vmxoffinstruction_emulation_cost__base.as_ref()
    }

    /// Sets the value of VMXONEmulationInterceptsPersec
    pub fn set_vmxonemulation_intercepts_persec(&mut self, value: u64) {
        self.vmxonemulation_intercepts_persec = Some(value);
    }

    /// Gets the value of VMXONEmulationInterceptsPersec
    pub fn get_vmxonemulation_intercepts_persec(&self) -> Option<&u64> {
        self.vmxonemulation_intercepts_persec.as_ref()
    }

    /// Sets the value of VMXONInstructionEmulationCost
    pub fn set_vmxoninstruction_emulation_cost(&mut self, value: u64) {
        self.vmxoninstruction_emulation_cost = Some(value);
    }

    /// Gets the value of VMXONInstructionEmulationCost
    pub fn get_vmxoninstruction_emulation_cost(&self) -> Option<&u64> {
        self.vmxoninstruction_emulation_cost.as_ref()
    }

    /// Sets the value of VMXONInstructionEmulationCost_Base
    pub fn set_vmxoninstruction_emulation_cost__base(&mut self, value: u64) {
        self.vmxoninstruction_emulation_cost__base = Some(value);
    }

    /// Gets the value of VMXONInstructionEmulationCost_Base
    pub fn get_vmxoninstruction_emulation_cost__base(&self) -> Option<&u64> {
        self.vmxoninstruction_emulation_cost__base.as_ref()
    }

    /// Sets the value of VSMHypercallsPersec
    pub fn set_vsmhypercalls_persec(&mut self, value: u64) {
        self.vsmhypercalls_persec = Some(value);
    }

    /// Gets the value of VSMHypercallsPersec
    pub fn get_vsmhypercalls_persec(&self) -> Option<&u64> {
        self.vsmhypercalls_persec.as_ref()
    }

    /// Sets the value of VTL1AverageRunTime
    pub fn set_vtl1_average_run_time(&mut self, value: u64) {
        self.vtl1_average_run_time = Some(value);
    }

    /// Gets the value of VTL1AverageRunTime
    pub fn get_vtl1_average_run_time(&self) -> Option<&u64> {
        self.vtl1_average_run_time.as_ref()
    }

    /// Sets the value of VTL1AverageRunTime_Base
    pub fn set_vtl1_average_run_time__base(&mut self, value: u64) {
        self.vtl1_average_run_time__base = Some(value);
    }

    /// Gets the value of VTL1AverageRunTime_Base
    pub fn get_vtl1_average_run_time__base(&self) -> Option<&u64> {
        self.vtl1_average_run_time__base.as_ref()
    }

    /// Sets the value of VTL1DispatchesPersec
    pub fn set_vtl1_dispatches_persec(&mut self, value: u64) {
        self.vtl1_dispatches_persec = Some(value);
    }

    /// Gets the value of VTL1DispatchesPersec
    pub fn get_vtl1_dispatches_persec(&self) -> Option<&u64> {
        self.vtl1_dispatches_persec.as_ref()
    }

    /// Sets the value of VTL2AverageRunTime
    pub fn set_vtl2_average_run_time(&mut self, value: u64) {
        self.vtl2_average_run_time = Some(value);
    }

    /// Gets the value of VTL2AverageRunTime
    pub fn get_vtl2_average_run_time(&self) -> Option<&u64> {
        self.vtl2_average_run_time.as_ref()
    }

    /// Sets the value of VTL2AverageRunTime_Base
    pub fn set_vtl2_average_run_time__base(&mut self, value: u64) {
        self.vtl2_average_run_time__base = Some(value);
    }

    /// Gets the value of VTL2AverageRunTime_Base
    pub fn get_vtl2_average_run_time__base(&self) -> Option<&u64> {
        self.vtl2_average_run_time__base.as_ref()
    }

    /// Sets the value of VTL2DispatchesPersec
    pub fn set_vtl2_dispatches_persec(&mut self, value: u64) {
        self.vtl2_dispatches_persec = Some(value);
    }

    /// Gets the value of VTL2DispatchesPersec
    pub fn get_vtl2_dispatches_persec(&self) -> Option<&u64> {
        self.vtl2_dispatches_persec.as_ref()
    }
}

