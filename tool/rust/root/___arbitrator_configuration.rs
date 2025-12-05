// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __ArbitratorConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __ArbitratorConfiguration {
    #[serde(flatten)]
    pub base: __SystemClass,

/// 
    #[serde(rename = "OutstandingTasksPerUser")]
    pub outstanding_tasks_per_user: Option<u32>,

/// 
    #[serde(rename = "OutstandingTasksTotal")]
    pub outstanding_tasks_total: Option<u32>,

/// 
    #[serde(rename = "PermanentSubscriptionsPerUser")]
    pub permanent_subscriptions_per_user: Option<u32>,

/// 
    #[serde(rename = "PermanentSubscriptionsTotal")]
    pub permanent_subscriptions_total: Option<u32>,

/// 
    #[serde(rename = "PollingInstructionsPerUser")]
    pub polling_instructions_per_user: Option<u32>,

/// 
    #[serde(rename = "PollingInstructionsTotal")]
    pub polling_instructions_total: Option<u32>,

/// 
    #[serde(rename = "PollingMemoryPerUser")]
    pub polling_memory_per_user: Option<u32>,

/// 
    #[serde(rename = "PollingMemoryTotal")]
    pub polling_memory_total: Option<u32>,

/// 
    #[serde(rename = "QuotaRetryCount")]
    pub quota_retry_count: Option<u32>,

/// 
    #[serde(rename = "QuotaRetryWaitInterval")]
    pub quota_retry_wait_interval: Option<u32>,

/// 
    #[serde(rename = "TaskThreadsPerUser")]
    pub task_threads_per_user: Option<u32>,

/// 
    #[serde(rename = "TaskThreadsTotal")]
    pub task_threads_total: Option<u32>,

/// 
    #[serde(rename = "TemporarySubscriptionsPerUser")]
    pub temporary_subscriptions_per_user: Option<u32>,

/// 
    #[serde(rename = "TemporarySubscriptionsTotal")]
    pub temporary_subscriptions_total: Option<u32>,

/// 
    #[serde(rename = "TotalCacheDisk")]
    pub total_cache_disk: Option<u32>,

/// 
    #[serde(rename = "TotalCacheDiskPerTask")]
    pub total_cache_disk_per_task: Option<u32>,

/// 
    #[serde(rename = "TotalCacheDiskPerUser")]
    pub total_cache_disk_per_user: Option<u32>,

/// 
    #[serde(rename = "TotalCacheMemory")]
    pub total_cache_memory: Option<u32>,

/// 
    #[serde(rename = "TotalCacheMemoryPerTask")]
    pub total_cache_memory_per_task: Option<u32>,

/// 
    #[serde(rename = "TotalCacheMemoryPerUser")]
    pub total_cache_memory_per_user: Option<u32>,

/// 
    #[serde(rename = "TotalUsers")]
    pub total_users: Option<u32>,
}

impl __ArbitratorConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __SystemClass::new(),
            outstanding_tasks_per_user: None,
            outstanding_tasks_total: None,
            permanent_subscriptions_per_user: None,
            permanent_subscriptions_total: None,
            polling_instructions_per_user: None,
            polling_instructions_total: None,
            polling_memory_per_user: None,
            polling_memory_total: None,
            quota_retry_count: None,
            quota_retry_wait_interval: None,
            task_threads_per_user: None,
            task_threads_total: None,
            temporary_subscriptions_per_user: None,
            temporary_subscriptions_total: None,
            total_cache_disk: None,
            total_cache_disk_per_task: None,
            total_cache_disk_per_user: None,
            total_cache_memory: None,
            total_cache_memory_per_task: None,
            total_cache_memory_per_user: None,
            total_users: None,
        }
    }


    /// Sets the value of OutstandingTasksPerUser
    pub fn set_outstanding_tasks_per_user(&mut self, value: u32) {
        self.outstanding_tasks_per_user = Some(value);
    }

    /// Gets the value of OutstandingTasksPerUser
    pub fn get_outstanding_tasks_per_user(&self) -> Option<&u32> {
        self.outstanding_tasks_per_user.as_ref()
    }

    /// Sets the value of OutstandingTasksTotal
    pub fn set_outstanding_tasks_total(&mut self, value: u32) {
        self.outstanding_tasks_total = Some(value);
    }

    /// Gets the value of OutstandingTasksTotal
    pub fn get_outstanding_tasks_total(&self) -> Option<&u32> {
        self.outstanding_tasks_total.as_ref()
    }

    /// Sets the value of PermanentSubscriptionsPerUser
    pub fn set_permanent_subscriptions_per_user(&mut self, value: u32) {
        self.permanent_subscriptions_per_user = Some(value);
    }

    /// Gets the value of PermanentSubscriptionsPerUser
    pub fn get_permanent_subscriptions_per_user(&self) -> Option<&u32> {
        self.permanent_subscriptions_per_user.as_ref()
    }

    /// Sets the value of PermanentSubscriptionsTotal
    pub fn set_permanent_subscriptions_total(&mut self, value: u32) {
        self.permanent_subscriptions_total = Some(value);
    }

    /// Gets the value of PermanentSubscriptionsTotal
    pub fn get_permanent_subscriptions_total(&self) -> Option<&u32> {
        self.permanent_subscriptions_total.as_ref()
    }

    /// Sets the value of PollingInstructionsPerUser
    pub fn set_polling_instructions_per_user(&mut self, value: u32) {
        self.polling_instructions_per_user = Some(value);
    }

    /// Gets the value of PollingInstructionsPerUser
    pub fn get_polling_instructions_per_user(&self) -> Option<&u32> {
        self.polling_instructions_per_user.as_ref()
    }

    /// Sets the value of PollingInstructionsTotal
    pub fn set_polling_instructions_total(&mut self, value: u32) {
        self.polling_instructions_total = Some(value);
    }

    /// Gets the value of PollingInstructionsTotal
    pub fn get_polling_instructions_total(&self) -> Option<&u32> {
        self.polling_instructions_total.as_ref()
    }

    /// Sets the value of PollingMemoryPerUser
    pub fn set_polling_memory_per_user(&mut self, value: u32) {
        self.polling_memory_per_user = Some(value);
    }

    /// Gets the value of PollingMemoryPerUser
    pub fn get_polling_memory_per_user(&self) -> Option<&u32> {
        self.polling_memory_per_user.as_ref()
    }

    /// Sets the value of PollingMemoryTotal
    pub fn set_polling_memory_total(&mut self, value: u32) {
        self.polling_memory_total = Some(value);
    }

    /// Gets the value of PollingMemoryTotal
    pub fn get_polling_memory_total(&self) -> Option<&u32> {
        self.polling_memory_total.as_ref()
    }

    /// Sets the value of QuotaRetryCount
    pub fn set_quota_retry_count(&mut self, value: u32) {
        self.quota_retry_count = Some(value);
    }

    /// Gets the value of QuotaRetryCount
    pub fn get_quota_retry_count(&self) -> Option<&u32> {
        self.quota_retry_count.as_ref()
    }

    /// Sets the value of QuotaRetryWaitInterval
    pub fn set_quota_retry_wait_interval(&mut self, value: u32) {
        self.quota_retry_wait_interval = Some(value);
    }

    /// Gets the value of QuotaRetryWaitInterval
    pub fn get_quota_retry_wait_interval(&self) -> Option<&u32> {
        self.quota_retry_wait_interval.as_ref()
    }

    /// Sets the value of TaskThreadsPerUser
    pub fn set_task_threads_per_user(&mut self, value: u32) {
        self.task_threads_per_user = Some(value);
    }

    /// Gets the value of TaskThreadsPerUser
    pub fn get_task_threads_per_user(&self) -> Option<&u32> {
        self.task_threads_per_user.as_ref()
    }

    /// Sets the value of TaskThreadsTotal
    pub fn set_task_threads_total(&mut self, value: u32) {
        self.task_threads_total = Some(value);
    }

    /// Gets the value of TaskThreadsTotal
    pub fn get_task_threads_total(&self) -> Option<&u32> {
        self.task_threads_total.as_ref()
    }

    /// Sets the value of TemporarySubscriptionsPerUser
    pub fn set_temporary_subscriptions_per_user(&mut self, value: u32) {
        self.temporary_subscriptions_per_user = Some(value);
    }

    /// Gets the value of TemporarySubscriptionsPerUser
    pub fn get_temporary_subscriptions_per_user(&self) -> Option<&u32> {
        self.temporary_subscriptions_per_user.as_ref()
    }

    /// Sets the value of TemporarySubscriptionsTotal
    pub fn set_temporary_subscriptions_total(&mut self, value: u32) {
        self.temporary_subscriptions_total = Some(value);
    }

    /// Gets the value of TemporarySubscriptionsTotal
    pub fn get_temporary_subscriptions_total(&self) -> Option<&u32> {
        self.temporary_subscriptions_total.as_ref()
    }

    /// Sets the value of TotalCacheDisk
    pub fn set_total_cache_disk(&mut self, value: u32) {
        self.total_cache_disk = Some(value);
    }

    /// Gets the value of TotalCacheDisk
    pub fn get_total_cache_disk(&self) -> Option<&u32> {
        self.total_cache_disk.as_ref()
    }

    /// Sets the value of TotalCacheDiskPerTask
    pub fn set_total_cache_disk_per_task(&mut self, value: u32) {
        self.total_cache_disk_per_task = Some(value);
    }

    /// Gets the value of TotalCacheDiskPerTask
    pub fn get_total_cache_disk_per_task(&self) -> Option<&u32> {
        self.total_cache_disk_per_task.as_ref()
    }

    /// Sets the value of TotalCacheDiskPerUser
    pub fn set_total_cache_disk_per_user(&mut self, value: u32) {
        self.total_cache_disk_per_user = Some(value);
    }

    /// Gets the value of TotalCacheDiskPerUser
    pub fn get_total_cache_disk_per_user(&self) -> Option<&u32> {
        self.total_cache_disk_per_user.as_ref()
    }

    /// Sets the value of TotalCacheMemory
    pub fn set_total_cache_memory(&mut self, value: u32) {
        self.total_cache_memory = Some(value);
    }

    /// Gets the value of TotalCacheMemory
    pub fn get_total_cache_memory(&self) -> Option<&u32> {
        self.total_cache_memory.as_ref()
    }

    /// Sets the value of TotalCacheMemoryPerTask
    pub fn set_total_cache_memory_per_task(&mut self, value: u32) {
        self.total_cache_memory_per_task = Some(value);
    }

    /// Gets the value of TotalCacheMemoryPerTask
    pub fn get_total_cache_memory_per_task(&self) -> Option<&u32> {
        self.total_cache_memory_per_task.as_ref()
    }

    /// Sets the value of TotalCacheMemoryPerUser
    pub fn set_total_cache_memory_per_user(&mut self, value: u32) {
        self.total_cache_memory_per_user = Some(value);
    }

    /// Gets the value of TotalCacheMemoryPerUser
    pub fn get_total_cache_memory_per_user(&self) -> Option<&u32> {
        self.total_cache_memory_per_user.as_ref()
    }

    /// Sets the value of TotalUsers
    pub fn set_total_users(&mut self, value: u32) {
        self.total_users = Some(value);
    }

    /// Gets the value of TotalUsers
    pub fn get_total_users(&self) -> Option<&u32> {
        self.total_users.as_ref()
    }
}

