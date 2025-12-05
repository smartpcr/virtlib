// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_HvStats_HyperVHypervisorLogicalProcessor struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_HvStats_HyperVHypervisorLogicalProcessor {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "C1TransitionsPersec")]
    pub c1_transitions_persec: Option<u64>,

/// 
    #[serde(rename = "C2TransitionsPersec")]
    pub c2_transitions_persec: Option<u64>,

/// 
    #[serde(rename = "C3TransitionsPersec")]
    pub c3_transitions_persec: Option<u64>,

/// 
    #[serde(rename = "ContextSwitchesPersec")]
    pub context_switches_persec: Option<u64>,

/// 
    #[serde(rename = "CPPCRequestContextSwitchesPersec")]
    pub cppcrequest_context_switches_persec: Option<u64>,

/// 
    #[serde(rename = "HardwareInterruptsPersec")]
    pub hardware_interrupts_persec: Option<u64>,

/// 
    #[serde(rename = "HypervisorBranchPredictorFlushesPersec")]
    pub hypervisor_branch_predictor_flushes_persec: Option<u64>,

/// 
    #[serde(rename = "HypervisorImmediateL1DataCacheFlushesPersec")]
    pub hypervisor_immediate_l1_data_cache_flushes_persec: Option<u64>,

/// 
    #[serde(rename = "HypervisorL1DataCacheFlushesPersec")]
    pub hypervisor_l1_data_cache_flushes_persec: Option<u64>,

/// 
    #[serde(rename = "HypervisorMicroarchitecturalBufferFlushesPersec")]
    pub hypervisor_microarchitectural_buffer_flushes_persec: Option<u64>,

/// 
    #[serde(rename = "InterProcessorInterruptsPersec")]
    pub inter_processor_interrupts_persec: Option<u64>,

/// 
    #[serde(rename = "InterProcessorInterruptsSentPersec")]
    pub inter_processor_interrupts_sent_persec: Option<u64>,

/// 
    #[serde(rename = "MonitorTransitionCost")]
    pub monitor_transition_cost: Option<u64>,

/// 
    #[serde(rename = "PercentC1Time")]
    pub percent_c1_time: Option<u64>,

/// 
    #[serde(rename = "PercentC2Time")]
    pub percent_c2_time: Option<u64>,

/// 
    #[serde(rename = "PercentC3Time")]
    pub percent_c3_time: Option<u64>,

/// 
    #[serde(rename = "PercentGuestRunTime")]
    pub percent_guest_run_time: Option<u64>,

/// 
    #[serde(rename = "PercentHypervisorRunTime")]
    pub percent_hypervisor_run_time: Option<u64>,

/// 
    #[serde(rename = "PercentIdleTime")]
    pub percent_idle_time: Option<u64>,

/// 
    #[serde(rename = "PercentTotalRunTime")]
    pub percent_total_run_time: Option<u64>,

/// 
    #[serde(rename = "PerformanceMonitoringInterruptsPersec")]
    pub performance_monitoring_interrupts_persec: Option<u64>,

/// 
    #[serde(rename = "PostedInterruptNotificationsPersec")]
    pub posted_interrupt_notifications_persec: Option<u64>,

/// 
    #[serde(rename = "ReserveGroupId")]
    pub reserve_group_id: Option<u64>,

/// 
    #[serde(rename = "RootVpIndex")]
    pub root_vp_index: Option<u64>,

/// 
    #[serde(rename = "RunningPriority")]
    pub running_priority: Option<u64>,

/// 
    #[serde(rename = "SchedulerInterruptsPersec")]
    pub scheduler_interrupts_persec: Option<u64>,

/// 
    #[serde(rename = "SchedulerLocalRunListSize")]
    pub scheduler_local_run_list_size: Option<u64>,

/// 
    #[serde(rename = "TimerInterruptsPersec")]
    pub timer_interrupts_persec: Option<u64>,

/// 
    #[serde(rename = "TotalInterruptsPersec")]
    pub total_interrupts_persec: Option<u64>,
}

impl Win32_PerfFormattedData_HvStats_HyperVHypervisorLogicalProcessor {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            c1_transitions_persec: None,
            c2_transitions_persec: None,
            c3_transitions_persec: None,
            context_switches_persec: None,
            cppcrequest_context_switches_persec: None,
            hardware_interrupts_persec: None,
            hypervisor_branch_predictor_flushes_persec: None,
            hypervisor_immediate_l1_data_cache_flushes_persec: None,
            hypervisor_l1_data_cache_flushes_persec: None,
            hypervisor_microarchitectural_buffer_flushes_persec: None,
            inter_processor_interrupts_persec: None,
            inter_processor_interrupts_sent_persec: None,
            monitor_transition_cost: None,
            percent_c1_time: None,
            percent_c2_time: None,
            percent_c3_time: None,
            percent_guest_run_time: None,
            percent_hypervisor_run_time: None,
            percent_idle_time: None,
            percent_total_run_time: None,
            performance_monitoring_interrupts_persec: None,
            posted_interrupt_notifications_persec: None,
            reserve_group_id: None,
            root_vp_index: None,
            running_priority: None,
            scheduler_interrupts_persec: None,
            scheduler_local_run_list_size: None,
            timer_interrupts_persec: None,
            total_interrupts_persec: None,
        }
    }


    /// Sets the value of C1TransitionsPersec
    pub fn set_c1_transitions_persec(&mut self, value: u64) {
        self.c1_transitions_persec = Some(value);
    }

    /// Gets the value of C1TransitionsPersec
    pub fn get_c1_transitions_persec(&self) -> Option<&u64> {
        self.c1_transitions_persec.as_ref()
    }

    /// Sets the value of C2TransitionsPersec
    pub fn set_c2_transitions_persec(&mut self, value: u64) {
        self.c2_transitions_persec = Some(value);
    }

    /// Gets the value of C2TransitionsPersec
    pub fn get_c2_transitions_persec(&self) -> Option<&u64> {
        self.c2_transitions_persec.as_ref()
    }

    /// Sets the value of C3TransitionsPersec
    pub fn set_c3_transitions_persec(&mut self, value: u64) {
        self.c3_transitions_persec = Some(value);
    }

    /// Gets the value of C3TransitionsPersec
    pub fn get_c3_transitions_persec(&self) -> Option<&u64> {
        self.c3_transitions_persec.as_ref()
    }

    /// Sets the value of ContextSwitchesPersec
    pub fn set_context_switches_persec(&mut self, value: u64) {
        self.context_switches_persec = Some(value);
    }

    /// Gets the value of ContextSwitchesPersec
    pub fn get_context_switches_persec(&self) -> Option<&u64> {
        self.context_switches_persec.as_ref()
    }

    /// Sets the value of CPPCRequestContextSwitchesPersec
    pub fn set_cppcrequest_context_switches_persec(&mut self, value: u64) {
        self.cppcrequest_context_switches_persec = Some(value);
    }

    /// Gets the value of CPPCRequestContextSwitchesPersec
    pub fn get_cppcrequest_context_switches_persec(&self) -> Option<&u64> {
        self.cppcrequest_context_switches_persec.as_ref()
    }

    /// Sets the value of HardwareInterruptsPersec
    pub fn set_hardware_interrupts_persec(&mut self, value: u64) {
        self.hardware_interrupts_persec = Some(value);
    }

    /// Gets the value of HardwareInterruptsPersec
    pub fn get_hardware_interrupts_persec(&self) -> Option<&u64> {
        self.hardware_interrupts_persec.as_ref()
    }

    /// Sets the value of HypervisorBranchPredictorFlushesPersec
    pub fn set_hypervisor_branch_predictor_flushes_persec(&mut self, value: u64) {
        self.hypervisor_branch_predictor_flushes_persec = Some(value);
    }

    /// Gets the value of HypervisorBranchPredictorFlushesPersec
    pub fn get_hypervisor_branch_predictor_flushes_persec(&self) -> Option<&u64> {
        self.hypervisor_branch_predictor_flushes_persec.as_ref()
    }

    /// Sets the value of HypervisorImmediateL1DataCacheFlushesPersec
    pub fn set_hypervisor_immediate_l1_data_cache_flushes_persec(&mut self, value: u64) {
        self.hypervisor_immediate_l1_data_cache_flushes_persec = Some(value);
    }

    /// Gets the value of HypervisorImmediateL1DataCacheFlushesPersec
    pub fn get_hypervisor_immediate_l1_data_cache_flushes_persec(&self) -> Option<&u64> {
        self.hypervisor_immediate_l1_data_cache_flushes_persec.as_ref()
    }

    /// Sets the value of HypervisorL1DataCacheFlushesPersec
    pub fn set_hypervisor_l1_data_cache_flushes_persec(&mut self, value: u64) {
        self.hypervisor_l1_data_cache_flushes_persec = Some(value);
    }

    /// Gets the value of HypervisorL1DataCacheFlushesPersec
    pub fn get_hypervisor_l1_data_cache_flushes_persec(&self) -> Option<&u64> {
        self.hypervisor_l1_data_cache_flushes_persec.as_ref()
    }

    /// Sets the value of HypervisorMicroarchitecturalBufferFlushesPersec
    pub fn set_hypervisor_microarchitectural_buffer_flushes_persec(&mut self, value: u64) {
        self.hypervisor_microarchitectural_buffer_flushes_persec = Some(value);
    }

    /// Gets the value of HypervisorMicroarchitecturalBufferFlushesPersec
    pub fn get_hypervisor_microarchitectural_buffer_flushes_persec(&self) -> Option<&u64> {
        self.hypervisor_microarchitectural_buffer_flushes_persec.as_ref()
    }

    /// Sets the value of InterProcessorInterruptsPersec
    pub fn set_inter_processor_interrupts_persec(&mut self, value: u64) {
        self.inter_processor_interrupts_persec = Some(value);
    }

    /// Gets the value of InterProcessorInterruptsPersec
    pub fn get_inter_processor_interrupts_persec(&self) -> Option<&u64> {
        self.inter_processor_interrupts_persec.as_ref()
    }

    /// Sets the value of InterProcessorInterruptsSentPersec
    pub fn set_inter_processor_interrupts_sent_persec(&mut self, value: u64) {
        self.inter_processor_interrupts_sent_persec = Some(value);
    }

    /// Gets the value of InterProcessorInterruptsSentPersec
    pub fn get_inter_processor_interrupts_sent_persec(&self) -> Option<&u64> {
        self.inter_processor_interrupts_sent_persec.as_ref()
    }

    /// Sets the value of MonitorTransitionCost
    pub fn set_monitor_transition_cost(&mut self, value: u64) {
        self.monitor_transition_cost = Some(value);
    }

    /// Gets the value of MonitorTransitionCost
    pub fn get_monitor_transition_cost(&self) -> Option<&u64> {
        self.monitor_transition_cost.as_ref()
    }

    /// Sets the value of PercentC1Time
    pub fn set_percent_c1_time(&mut self, value: u64) {
        self.percent_c1_time = Some(value);
    }

    /// Gets the value of PercentC1Time
    pub fn get_percent_c1_time(&self) -> Option<&u64> {
        self.percent_c1_time.as_ref()
    }

    /// Sets the value of PercentC2Time
    pub fn set_percent_c2_time(&mut self, value: u64) {
        self.percent_c2_time = Some(value);
    }

    /// Gets the value of PercentC2Time
    pub fn get_percent_c2_time(&self) -> Option<&u64> {
        self.percent_c2_time.as_ref()
    }

    /// Sets the value of PercentC3Time
    pub fn set_percent_c3_time(&mut self, value: u64) {
        self.percent_c3_time = Some(value);
    }

    /// Gets the value of PercentC3Time
    pub fn get_percent_c3_time(&self) -> Option<&u64> {
        self.percent_c3_time.as_ref()
    }

    /// Sets the value of PercentGuestRunTime
    pub fn set_percent_guest_run_time(&mut self, value: u64) {
        self.percent_guest_run_time = Some(value);
    }

    /// Gets the value of PercentGuestRunTime
    pub fn get_percent_guest_run_time(&self) -> Option<&u64> {
        self.percent_guest_run_time.as_ref()
    }

    /// Sets the value of PercentHypervisorRunTime
    pub fn set_percent_hypervisor_run_time(&mut self, value: u64) {
        self.percent_hypervisor_run_time = Some(value);
    }

    /// Gets the value of PercentHypervisorRunTime
    pub fn get_percent_hypervisor_run_time(&self) -> Option<&u64> {
        self.percent_hypervisor_run_time.as_ref()
    }

    /// Sets the value of PercentIdleTime
    pub fn set_percent_idle_time(&mut self, value: u64) {
        self.percent_idle_time = Some(value);
    }

    /// Gets the value of PercentIdleTime
    pub fn get_percent_idle_time(&self) -> Option<&u64> {
        self.percent_idle_time.as_ref()
    }

    /// Sets the value of PercentTotalRunTime
    pub fn set_percent_total_run_time(&mut self, value: u64) {
        self.percent_total_run_time = Some(value);
    }

    /// Gets the value of PercentTotalRunTime
    pub fn get_percent_total_run_time(&self) -> Option<&u64> {
        self.percent_total_run_time.as_ref()
    }

    /// Sets the value of PerformanceMonitoringInterruptsPersec
    pub fn set_performance_monitoring_interrupts_persec(&mut self, value: u64) {
        self.performance_monitoring_interrupts_persec = Some(value);
    }

    /// Gets the value of PerformanceMonitoringInterruptsPersec
    pub fn get_performance_monitoring_interrupts_persec(&self) -> Option<&u64> {
        self.performance_monitoring_interrupts_persec.as_ref()
    }

    /// Sets the value of PostedInterruptNotificationsPersec
    pub fn set_posted_interrupt_notifications_persec(&mut self, value: u64) {
        self.posted_interrupt_notifications_persec = Some(value);
    }

    /// Gets the value of PostedInterruptNotificationsPersec
    pub fn get_posted_interrupt_notifications_persec(&self) -> Option<&u64> {
        self.posted_interrupt_notifications_persec.as_ref()
    }

    /// Sets the value of ReserveGroupId
    pub fn set_reserve_group_id(&mut self, value: u64) {
        self.reserve_group_id = Some(value);
    }

    /// Gets the value of ReserveGroupId
    pub fn get_reserve_group_id(&self) -> Option<&u64> {
        self.reserve_group_id.as_ref()
    }

    /// Sets the value of RootVpIndex
    pub fn set_root_vp_index(&mut self, value: u64) {
        self.root_vp_index = Some(value);
    }

    /// Gets the value of RootVpIndex
    pub fn get_root_vp_index(&self) -> Option<&u64> {
        self.root_vp_index.as_ref()
    }

    /// Sets the value of RunningPriority
    pub fn set_running_priority(&mut self, value: u64) {
        self.running_priority = Some(value);
    }

    /// Gets the value of RunningPriority
    pub fn get_running_priority(&self) -> Option<&u64> {
        self.running_priority.as_ref()
    }

    /// Sets the value of SchedulerInterruptsPersec
    pub fn set_scheduler_interrupts_persec(&mut self, value: u64) {
        self.scheduler_interrupts_persec = Some(value);
    }

    /// Gets the value of SchedulerInterruptsPersec
    pub fn get_scheduler_interrupts_persec(&self) -> Option<&u64> {
        self.scheduler_interrupts_persec.as_ref()
    }

    /// Sets the value of SchedulerLocalRunListSize
    pub fn set_scheduler_local_run_list_size(&mut self, value: u64) {
        self.scheduler_local_run_list_size = Some(value);
    }

    /// Gets the value of SchedulerLocalRunListSize
    pub fn get_scheduler_local_run_list_size(&self) -> Option<&u64> {
        self.scheduler_local_run_list_size.as_ref()
    }

    /// Sets the value of TimerInterruptsPersec
    pub fn set_timer_interrupts_persec(&mut self, value: u64) {
        self.timer_interrupts_persec = Some(value);
    }

    /// Gets the value of TimerInterruptsPersec
    pub fn get_timer_interrupts_persec(&self) -> Option<&u64> {
        self.timer_interrupts_persec.as_ref()
    }

    /// Sets the value of TotalInterruptsPersec
    pub fn set_total_interrupts_persec(&mut self, value: u64) {
        self.total_interrupts_persec = Some(value);
    }

    /// Gets the value of TotalInterruptsPersec
    pub fn get_total_interrupts_persec(&self) -> Option<&u64> {
        self.total_interrupts_persec.as_ref()
    }
}

