//! Platform-specific memory measurement utilities
//!
//! This module provides cross-platform memory measurement capabilities
//! for performance benchmarks. It uses platform-specific APIs to measure
//! peak and current memory usage with graceful degradation on unsupported platforms.

use std::fmt;

/// Error types for memory measurement operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryMeasurementError {
    /// Platform is not supported for memory measurement
    UnsupportedPlatform,
    /// Measurement operation failed with a specific error message
    MeasurementFailed(String),
    /// Platform-specific error occurred
    PlatformError(String),
}

impl fmt::Display for MemoryMeasurementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryMeasurementError::UnsupportedPlatform => {
                write!(f, "Memory measurement is not supported on this platform")
            }
            MemoryMeasurementError::MeasurementFailed(msg) => {
                write!(f, "Memory measurement failed: {}", msg)
            }
            MemoryMeasurementError::PlatformError(msg) => {
                write!(f, "Platform error during memory measurement: {}", msg)
            }
        }
    }
}

impl std::error::Error for MemoryMeasurementError {}

/// Trait for platform-specific memory measurement implementations
pub trait MemoryMeasurer {
    /// Measure peak memory usage in MB
    fn measure_peak_memory_mb() -> Result<u64, MemoryMeasurementError>;
    
    /// Measure current memory usage in MB
    fn measure_current_memory_mb() -> Result<u64, MemoryMeasurementError>;
}

/// Linux memory measurer using /proc/self/status
#[cfg(target_os = "linux")]
pub struct LinuxMemoryMeasurer;

#[cfg(target_os = "linux")]
impl MemoryMeasurer for LinuxMemoryMeasurer {
    fn measure_peak_memory_mb() -> Result<u64, MemoryMeasurementError> {
        use std::fs;
        
        let status = fs::read_to_string("/proc/self/status")
            .map_err(|e| MemoryMeasurementError::PlatformError(format!("Failed to read /proc/self/status: {}", e)))?;
        
        for line in status.lines() {
            if line.starts_with("VmPeak:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let value: u64 = parts[1]
                        .parse()
                        .map_err(|e| MemoryMeasurementError::MeasurementFailed(format!("Failed to parse VmPeak: {}", e)))?;
                    // VmPeak is in KB, convert to MB
                    return Ok(value / 1024);
                }
            }
        }
        
        Err(MemoryMeasurementError::MeasurementFailed(
            "VmPeak not found in /proc/self/status".to_string()
        ))
    }
    
    fn measure_current_memory_mb() -> Result<u64, MemoryMeasurementError> {
        use std::fs;
        
        let status = fs::read_to_string("/proc/self/status")
            .map_err(|e| MemoryMeasurementError::PlatformError(format!("Failed to read /proc/self/status: {}", e)))?;
        
        for line in status.lines() {
            if line.starts_with("VmRSS:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let value: u64 = parts[1]
                        .parse()
                        .map_err(|e| MemoryMeasurementError::MeasurementFailed(format!("Failed to parse VmRSS: {}", e)))?;
                    // VmRSS is in KB, convert to MB
                    return Ok(value / 1024);
                }
            }
        }
        
        Err(MemoryMeasurementError::MeasurementFailed(
            "VmRSS not found in /proc/self/status".to_string()
        ))
    }
}

/// macOS memory measurer using mach_task_basic_info via libc
#[cfg(target_os = "macos")]
pub struct MacOSMemoryMeasurer;

#[cfg(target_os = "macos")]
impl MemoryMeasurer for MacOSMemoryMeasurer {
    fn measure_peak_memory_mb() -> Result<u64, MemoryMeasurementError> {
        use libc::{mach_task_self, task_info, KERN_SUCCESS};
        
        // TASK_BASIC_INFO constant value (from mach/task_info.h)
        const TASK_BASIC_INFO: u32 = 5;
        
        // task_basic_info structure (simplified - we only need virtual_size and resident_size)
        #[repr(C)]
        struct TaskBasicInfo {
            suspend_count: libc::integer_t,
            virtual_size: libc::vm_size_t,
            resident_size: libc::vm_size_t,
            user_time: libc::time_value_t,
            system_time: libc::time_value_t,
        }
        
        unsafe {
            let mut info: TaskBasicInfo = std::mem::zeroed();
            let mut count = (std::mem::size_of::<TaskBasicInfo>() / std::mem::size_of::<libc::natural_t>()) as libc::mach_msg_type_number_t;
            
            let result = task_info(
                mach_task_self(),
                TASK_BASIC_INFO,
                &mut info as *mut _ as *mut libc::integer_t,
                &mut count,
            );
            
            if result != KERN_SUCCESS {
                return Err(MemoryMeasurementError::PlatformError(
                    format!("task_info failed with error code: {}", result)
                ));
            }
            
            // virtual_size is the total virtual memory, use resident_size for peak
            // Note: macOS doesn't track peak memory separately, so we use virtual_size
            // as an approximation. For more accurate peak measurement, we'd need
            // to track it ourselves over time.
            let memory_bytes = info.virtual_size as u64;
            Ok(memory_bytes / (1024 * 1024))
        }
    }
    
    fn measure_current_memory_mb() -> Result<u64, MemoryMeasurementError> {
        use libc::{mach_task_self, task_info, KERN_SUCCESS};
        
        // TASK_BASIC_INFO constant value (from mach/task_info.h)
        const TASK_BASIC_INFO: u32 = 5;
        
        // task_basic_info structure (simplified - we only need virtual_size and resident_size)
        #[repr(C)]
        struct TaskBasicInfo {
            suspend_count: libc::integer_t,
            virtual_size: libc::vm_size_t,
            resident_size: libc::vm_size_t,
            user_time: libc::time_value_t,
            system_time: libc::time_value_t,
        }
        
        unsafe {
            let mut info: TaskBasicInfo = std::mem::zeroed();
            let mut count = (std::mem::size_of::<TaskBasicInfo>() / std::mem::size_of::<libc::natural_t>()) as libc::mach_msg_type_number_t;
            
            let result = task_info(
                mach_task_self(),
                TASK_BASIC_INFO,
                &mut info as *mut _ as *mut libc::integer_t,
                &mut count,
            );
            
            if result != KERN_SUCCESS {
                return Err(MemoryMeasurementError::PlatformError(
                    format!("task_info failed with error code: {}", result)
                ));
            }
            
            // resident_size is the current resident set size
            let memory_bytes = info.resident_size as u64;
            Ok(memory_bytes / (1024 * 1024))
        }
    }
}

/// Windows memory measurer using GetProcessMemoryInfo from winapi
#[cfg(target_os = "windows")]
pub struct WindowsMemoryMeasurer;

#[cfg(target_os = "windows")]
impl MemoryMeasurer for WindowsMemoryMeasurer {
    fn measure_peak_memory_mb() -> Result<u64, MemoryMeasurementError> {
        use winapi::um::processthreadsapi::GetCurrentProcess;
        use winapi::um::psapi::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};
        use winapi::um::winnt::HANDLE;
        
        unsafe {
            let process: HANDLE = GetCurrentProcess();
            let mut pmc: PROCESS_MEMORY_COUNTERS = std::mem::zeroed();
            
            let result = GetProcessMemoryInfo(
                process,
                &mut pmc,
                std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
            );
            
            if result == 0 {
                return Err(MemoryMeasurementError::PlatformError(
                    "GetProcessMemoryInfo failed".to_string()
                ));
            }
            
            // PeakWorkingSetSize is in bytes, convert to MB
            Ok(pmc.PeakWorkingSetSize as u64 / (1024 * 1024))
        }
    }
    
    fn measure_current_memory_mb() -> Result<u64, MemoryMeasurementError> {
        use winapi::um::processthreadsapi::GetCurrentProcess;
        use winapi::um::psapi::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};
        use winapi::um::winnt::HANDLE;
        
        unsafe {
            let process: HANDLE = GetCurrentProcess();
            let mut pmc: PROCESS_MEMORY_COUNTERS = std::mem::zeroed();
            
            let result = GetProcessMemoryInfo(
                process,
                &mut pmc,
                std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
            );
            
            if result == 0 {
                return Err(MemoryMeasurementError::PlatformError(
                    "GetProcessMemoryInfo failed".to_string()
                ));
            }
            
            // WorkingSetSize is in bytes, convert to MB
            Ok(pmc.WorkingSetSize as u64 / (1024 * 1024))
        }
    }
}

/// Unsupported platform memory measurer (graceful degradation)
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
pub struct UnsupportedMemoryMeasurer;

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
impl MemoryMeasurer for UnsupportedMemoryMeasurer {
    fn measure_peak_memory_mb() -> Result<u64, MemoryMeasurementError> {
        Err(MemoryMeasurementError::UnsupportedPlatform)
    }
    
    fn measure_current_memory_mb() -> Result<u64, MemoryMeasurementError> {
        Err(MemoryMeasurementError::UnsupportedPlatform)
    }
}


/// Measure memory usage using platform-specific implementation
/// Returns (peak_mb, current_mb) with graceful degradation on unsupported platforms
pub fn measure_memory_usage_platform() -> (u64, u64) {
    #[cfg(target_os = "linux")]
    {
        match (LinuxMemoryMeasurer::measure_peak_memory_mb(), LinuxMemoryMeasurer::measure_current_memory_mb()) {
            (Ok(peak), Ok(current)) => (peak, current),
            _ => (0, 0),
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        match (MacOSMemoryMeasurer::measure_peak_memory_mb(), MacOSMemoryMeasurer::measure_current_memory_mb()) {
            (Ok(peak), Ok(current)) => (peak, current),
            _ => (0, 0),
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        match (WindowsMemoryMeasurer::measure_peak_memory_mb(), WindowsMemoryMeasurer::measure_current_memory_mb()) {
            (Ok(peak), Ok(current)) => (peak, current),
            _ => (0, 0),
        }
    }
    
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        (0, 0)
    }
}

