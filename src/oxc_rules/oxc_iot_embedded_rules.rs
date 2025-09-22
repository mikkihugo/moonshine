//! # IoT & Embedded Systems Rules
//!
//! Comprehensive rules for IoT device development, embedded JavaScript,
//! edge computing, and resource-constrained environments.
//!
//! ## Rule Categories:
//! - **Resource Management**: Memory optimization, CPU usage, battery efficiency
//! - **Network Optimization**: Bandwidth conservation, offline resilience, data compression
//! - **Real-time Constraints**: Timing requirements, interrupt handling, deadline management
//! - **Hardware Abstraction**: GPIO management, sensor interfacing, actuator control
//! - **Edge Computing**: Local processing, data filtering, reduced cloud dependency
//!
//! ## Examples:
//! ```javascript
//! // ✅ Good: Efficient sensor data processing
//! const sensorBuffer = new Float32Array(100);
//! function processSensorData(data) {
//!   // Local filtering to reduce network traffic
//!   return data.filter(value => value > threshold);
//! }
//!
//! // ❌ Bad: Inefficient memory usage
//! const allData = [];
//! setInterval(() => {
//!   allData.push(readSensor()); // Memory leak on IoT device
//! }, 100);
//! ```

use serde::{Deserialize, Serialize};

use crate::unified_rule_registry::{RuleSettings, WasmRuleCategory, WasmFixStatus};

/// Trait for basic WASM-compatible rule implementation
pub trait WasmRule {
    const NAME: &'static str;
    const CATEGORY: WasmRuleCategory;
    const FIX_STATUS: WasmFixStatus;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic>;
}

/// Enhanced trait for AI-powered rule suggestions
pub trait EnhancedWasmRule: WasmRule {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmRuleDiagnostic {
    pub rule_name: String,
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub severity: String,
    pub fix_suggestion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSuggestion {
    pub suggestion_type: String,
    pub confidence: f64,
    pub description: String,
    pub code_example: Option<String>,
}

/// Rule: require-memory-constraints
/// Enforces memory usage constraints for resource-limited IoT devices
#[derive(Clone)]
pub struct RequireMemoryConstraints;

impl RequireMemoryConstraints {
    pub const NAME: &'static str = "require-memory-constraints";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireMemoryConstraints {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for unbounded arrays that could exhaust memory
        if code.contains("push(") && !code.contains("shift()") && !code.contains("splice(") {
            diagnostics.push(create_memory_constraints_diagnostic(
                0, 0,
                "Unbounded arrays can exhaust memory on IoT devices - implement circular buffers"
            ));
        }

        // Check for large object allocations
        if code.contains("new Array(") && (code.contains("1000") || code.contains("10000")) {
            diagnostics.push(create_memory_constraints_diagnostic(
                0, 0,
                "Large array allocations should be avoided on memory-constrained devices"
            ));
        }

        // Check for memory-intensive operations
        if code.contains("JSON.stringify") && !code.contains("streaming") {
            diagnostics.push(create_memory_constraints_diagnostic(
                0, 0,
                "Large JSON operations should use streaming to conserve memory"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireMemoryConstraints {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.91,
            suggestion: "Implement circular buffer: `class CircularBuffer { constructor(size) { this.buffer = new Array(size); this.index = 0; } add(item) { this.buffer[this.index] = item; this.index = (this.index + 1) % this.buffer.length; } }`".to_string(),
            fix_code: Some("// Replace unbounded array with circular buffer\nconst buffer = new CircularBuffer(100);\nsetInterval(() => {\n  buffer.add(readSensor());\n}, 1000);".to_string()),
        }).collect()
    }
}

/// Rule: require-power-optimization
/// Enforces power-efficient coding patterns for battery-powered devices
#[derive(Clone)]
pub struct RequirePowerOptimization;

impl RequirePowerOptimization {
    pub const NAME: &'static str = "require-power-optimization";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequirePowerOptimization {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for high-frequency timers that drain battery
        if code.contains("setInterval") && (code.contains("10") || code.contains("50") || code.contains("100")) {
            diagnostics.push(create_power_optimization_diagnostic(
                0, 0,
                "High-frequency timers drain battery - use adaptive sampling rates"
            ));
        }

        // Check for continuous network operations
        if code.contains("fetch") && code.contains("setInterval") {
            diagnostics.push(create_power_optimization_diagnostic(
                0, 0,
                "Continuous network requests consume power - implement batching and sleep modes"
            ));
        }

        // Check for missing sleep/idle states
        if code.contains("while(true)") && !code.contains("sleep") && !code.contains("idle") {
            diagnostics.push(create_power_optimization_diagnostic(
                0, 0,
                "Continuous loops should implement sleep states to conserve power"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequirePowerOptimization {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.88,
            suggestion: "Implement adaptive sampling: `let interval = batteryLevel > 0.5 ? 1000 : 5000; setTimeout(nextSample, interval);`".to_string(),
            fix_code: Some("class PowerManager {\n  getOptimalInterval() {\n    const battery = navigator.getBattery();\n    return battery.level > 0.5 ? 1000 : 5000;\n  }\n  scheduleNextTask() {\n    setTimeout(() => {\n      this.performTask();\n      this.scheduleNextTask();\n    }, this.getOptimalInterval());\n  }\n}".to_string()),
        }).collect()
    }
}

/// Rule: require-offline-resilience
/// Enforces offline-first patterns for unreliable network environments
#[derive(Clone)]
pub struct RequireOfflineResilience;

impl RequireOfflineResilience {
    pub const NAME: &'static str = "require-offline-resilience";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireOfflineResilience {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for network operations without offline handling
        if code.contains("fetch(") && !code.contains("navigator.onLine") && !code.contains("catch") {
            diagnostics.push(create_offline_resilience_diagnostic(
                0, 0,
                "Network operations should handle offline scenarios gracefully"
            ));
        }

        // Check for missing local storage/caching
        if code.contains("api/") && !code.contains("localStorage") && !code.contains("cache") {
            diagnostics.push(create_offline_resilience_diagnostic(
                0, 0,
                "API data should be cached locally for offline access"
            ));
        }

        // Check for missing data synchronization
        if code.contains("POST") && !code.contains("queue") && !code.contains("sync") {
            diagnostics.push(create_offline_resilience_diagnostic(
                0, 0,
                "Data modifications should be queued for synchronization when online"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireOfflineResilience {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.90,
            suggestion: "Implement offline-first pattern: `if (!navigator.onLine) { return getCachedData(); } try { const data = await fetch(url); cacheData(data); return data; } catch { return getCachedData(); }`".to_string(),
            fix_code: Some("class OfflineManager {\n  async fetchWithFallback(url) {\n    const cached = this.getFromCache(url);\n    if (!navigator.onLine) return cached;\n    \n    try {\n      const response = await fetch(url);\n      const data = await response.json();\n      this.saveToCache(url, data);\n      return data;\n    } catch (error) {\n      return cached || { error: 'No cached data available' };\n    }\n  }\n}".to_string()),
        }).collect()
    }
}

/// Rule: require-sensor-data-validation
/// Enforces validation and filtering of sensor data
#[derive(Clone)]
pub struct RequireSensorDataValidation;

impl RequireSensorDataValidation {
    pub const NAME: &'static str = "require-sensor-data-validation";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireSensorDataValidation {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for raw sensor data usage without validation
        if code.contains("sensor.") && !code.contains("validate") && !code.contains("filter") {
            diagnostics.push(create_sensor_data_validation_diagnostic(
                0, 0,
                "Sensor data should be validated and filtered before processing"
            ));
        }

        // Check for missing range checks
        if code.contains("temperature") && !code.contains("range") && !code.contains("bounds") {
            diagnostics.push(create_sensor_data_validation_diagnostic(
                0, 0,
                "Sensor readings should include range validation to detect faulty sensors"
            ));
        }

        // Check for missing noise filtering
        if code.contains("accelerometer") && !code.contains("smooth") && !code.contains("average") {
            diagnostics.push(create_sensor_data_validation_diagnostic(
                0, 0,
                "Accelerometer data should be smoothed to filter noise"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireSensorDataValidation {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.93,
            suggestion: "Add sensor validation: `function validateSensorData(value, min, max) { return value >= min && value <= max && !isNaN(value); }`".to_string(),
            fix_code: Some("class SensorValidator {\n  validate(reading, type) {\n    const ranges = {\n      temperature: { min: -40, max: 85 },\n      humidity: { min: 0, max: 100 },\n      pressure: { min: 300, max: 1100 }\n    };\n    \n    const range = ranges[type];\n    return reading >= range.min && reading <= range.max && !isNaN(reading);\n  }\n  \n  smooth(readings) {\n    return readings.reduce((sum, val) => sum + val, 0) / readings.length;\n  }\n}".to_string()),
        }).collect()
    }
}

/// Rule: require-real-time-constraints
/// Enforces real-time deadline management for time-critical operations
#[derive(Clone)]
pub struct RequireRealTimeConstraints;

impl RequireRealTimeConstraints {
    pub const NAME: &'static str = "require-real-time-constraints";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireRealTimeConstraints {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for time-critical operations without deadline management
        if code.contains("control") && code.contains("motor") && !code.contains("deadline") {
            diagnostics.push(create_real_time_constraints_diagnostic(
                0, 0,
                "Motor control operations should enforce real-time deadlines"
            ));
        }

        // Check for interrupt handlers without timing constraints
        if code.contains("interrupt") && !code.contains("timeout") && !code.contains("deadline") {
            diagnostics.push(create_real_time_constraints_diagnostic(
                0, 0,
                "Interrupt handlers should have execution time constraints"
            ));
        }

        // Check for blocking operations in real-time code
        if code.contains("critical") && code.contains("await") && !code.contains("timeout") {
            diagnostics.push(create_real_time_constraints_diagnostic(
                0, 0,
                "Critical sections should avoid blocking operations or use timeouts"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireRealTimeConstraints {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.89,
            suggestion: "Implement deadline monitoring: `class DeadlineManager { scheduleWithDeadline(task, deadlineMs) { const start = Date.now(); task(); const elapsed = Date.now() - start; if (elapsed > deadlineMs) console.warn('Deadline missed'); } }`".to_string(),
            fix_code: Some("class RealTimeScheduler {\n  constructor() {\n    this.tasks = [];\n    this.deadlines = new Map();\n  }\n  \n  schedule(task, priority, deadlineMs) {\n    const deadline = Date.now() + deadlineMs;\n    this.deadlines.set(task, deadline);\n    this.tasks.push({ task, priority, deadline });\n    this.sortByDeadline();\n  }\n  \n  execute() {\n    if (this.tasks.length === 0) return;\n    \n    const { task, deadline } = this.tasks.shift();\n    const start = Date.now();\n    \n    try {\n      task();\n    } finally {\n      const elapsed = Date.now() - start;\n      if (Date.now() > deadline) {\n        console.warn(`Task missed deadline by ${Date.now() - deadline}ms`);\n      }\n    }\n  }\n}".to_string()),
        }).collect()
    }
}

/// Rule: require-hardware-abstraction
/// Enforces hardware abstraction layer patterns for device portability
#[derive(Clone)]
pub struct RequireHardwareAbstraction;

impl RequireHardwareAbstraction {
    pub const NAME: &'static str = "require-hardware-abstraction";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireHardwareAbstraction {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for direct hardware access without abstraction
        if code.contains("GPIO") && !code.contains("interface") && !code.contains("HAL") {
            diagnostics.push(create_hardware_abstraction_diagnostic(
                0, 0,
                "Direct GPIO access should use hardware abstraction layer for portability"
            ));
        }

        // Check for hardcoded pin numbers
        if code.contains("pin") && (code.contains("13") || code.contains("5") || code.contains("21")) {
            diagnostics.push(create_hardware_abstraction_diagnostic(
                0, 0,
                "Pin numbers should be defined in configuration, not hardcoded"
            ));
        }

        // Check for platform-specific code without interfaces
        if code.contains("esp32") || code.contains("arduino") {
            if !code.contains("interface") && !code.contains("abstract") {
                diagnostics.push(create_hardware_abstraction_diagnostic(
                    0, 0,
                    "Platform-specific code should implement common interfaces"
                ));
            }
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireHardwareAbstraction {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.86,
            suggestion: "Create hardware abstraction: `interface GPIOInterface { digitalWrite(pin: number, value: boolean): void; digitalRead(pin: number): boolean; } class ESP32GPIO implements GPIOInterface { ... }`".to_string(),
            fix_code: Some("// Hardware abstraction layer\nabstract class HardwareAbstractionLayer {\n  abstract digitalWrite(pin: string, value: boolean): void;\n  abstract digitalRead(pin: string): boolean;\n  abstract analogRead(pin: string): number;\n}\n\nclass ESP32HAL extends HardwareAbstractionLayer {\n  private pinConfig: Record<string, number>;\n  \n  constructor(config: Record<string, number>) {\n    super();\n    this.pinConfig = config;\n  }\n  \n  digitalWrite(pin: string, value: boolean): void {\n    const pinNumber = this.pinConfig[pin];\n    // ESP32-specific implementation\n  }\n}".to_string()),
        }).collect()
    }
}

/// Rule: require-edge-data-processing
/// Enforces local data processing to reduce network dependency
#[derive(Clone)]
pub struct RequireEdgeDataProcessing;

impl RequireEdgeDataProcessing {
    pub const NAME: &'static str = "require-edge-data-processing";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireEdgeDataProcessing {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for raw data transmission without local processing
        if code.contains("send(") && code.contains("sensor") && !code.contains("process") {
            diagnostics.push(create_edge_data_processing_diagnostic(
                0, 0,
                "Sensor data should be processed locally before transmission to reduce bandwidth"
            ));
        }

        // Check for missing data aggregation
        if code.contains("setInterval") && code.contains("transmit") && !code.contains("aggregate") {
            diagnostics.push(create_edge_data_processing_diagnostic(
                0, 0,
                "Frequent transmissions should aggregate data to reduce network usage"
            ));
        }

        // Check for cloud-dependent operations that could run locally
        if code.contains("api.analyze") && !code.contains("local") && !code.contains("edge") {
            diagnostics.push(create_edge_data_processing_diagnostic(
                0, 0,
                "Simple analysis operations should run locally when possible"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireEdgeDataProcessing {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.87,
            suggestion: "Implement edge processing: `const processed = sensorData.filter(d => d.anomaly).reduce((acc, d) => ({ ...acc, [d.type]: d.value }), {}); if (Object.keys(processed).length > 0) transmit(processed);`".to_string(),
            fix_code: Some("class EdgeProcessor {\n  private buffer: SensorReading[] = [];\n  private aggregationInterval = 60000; // 1 minute\n  \n  process(reading: SensorReading) {\n    // Local filtering\n    if (this.isAnomalous(reading)) {\n      this.transmitImmediate(reading);\n      return;\n    }\n    \n    // Buffer for aggregation\n    this.buffer.push(reading);\n    \n    // Aggregate and transmit periodically\n    if (this.buffer.length >= 10) {\n      const aggregated = this.aggregate(this.buffer);\n      this.transmit(aggregated);\n      this.buffer = [];\n    }\n  }\n  \n  private isAnomalous(reading: SensorReading): boolean {\n    return reading.value > this.threshold || reading.confidence < 0.8;\n  }\n  \n  private aggregate(readings: SensorReading[]) {\n    return {\n      avg: readings.reduce((sum, r) => sum + r.value, 0) / readings.length,\n      min: Math.min(...readings.map(r => r.value)),\n      max: Math.max(...readings.map(r => r.value)),\n      count: readings.length,\n      timestamp: Date.now()\n    };\n  }\n}".to_string()),
        }).collect()
    }
}

/// Rule: require-device-health-monitoring
/// Enforces device health monitoring and diagnostics
#[derive(Clone)]
pub struct RequireDeviceHealthMonitoring;

impl RequireDeviceHealthMonitoring {
    pub const NAME: &'static str = "require-device-health-monitoring";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireDeviceHealthMonitoring {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for IoT device code without health monitoring
        if code.contains("device") && code.contains("sensor") && !code.contains("health") {
            diagnostics.push(create_device_health_monitoring_diagnostic(
                0, 0,
                "IoT devices should implement health monitoring and diagnostics"
            ));
        }

        // Check for missing battery level monitoring
        if code.contains("battery") && !code.contains("level") && !code.contains("voltage") {
            diagnostics.push(create_device_health_monitoring_diagnostic(
                0, 0,
                "Battery-powered devices should monitor battery level and voltage"
            ));
        }

        // Check for missing temperature monitoring
        if code.contains("cpu") && !code.contains("temperature") && !code.contains("thermal") {
            diagnostics.push(create_device_health_monitoring_diagnostic(
                0, 0,
                "Devices should monitor CPU temperature to prevent overheating"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireDeviceHealthMonitoring {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.88,
            suggestion: "Add health monitoring: `class DeviceHealth { monitor() { return { battery: this.getBatteryLevel(), temperature: this.getCPUTemp(), memory: this.getMemoryUsage(), uptime: process.uptime() }; } }`".to_string(),
            fix_code: Some("class DeviceHealthMonitor {\n  private healthMetrics = {\n    battery: 0,\n    temperature: 0,\n    memory: 0,\n    uptime: 0,\n    errors: 0\n  };\n  \n  startMonitoring() {\n    setInterval(() => {\n      this.updateHealth();\n      this.checkThresholds();\n    }, 30000); // Every 30 seconds\n  }\n  \n  private updateHealth() {\n    this.healthMetrics = {\n      battery: this.getBatteryLevel(),\n      temperature: this.getCPUTemperature(),\n      memory: this.getMemoryUsage(),\n      uptime: process.uptime(),\n      errors: this.getErrorCount()\n    };\n  }\n  \n  private checkThresholds() {\n    if (this.healthMetrics.battery < 0.2) {\n      this.triggerAlert('Low battery warning');\n    }\n    if (this.healthMetrics.temperature > 80) {\n      this.triggerAlert('High temperature warning');\n    }\n  }\n}".to_string()),
        }).collect()
    }
}

/// Rule: require-secure-firmware-updates
/// Enforces secure over-the-air firmware update patterns
#[derive(Clone)]
pub struct RequireSecureFirmwareUpdates;

impl RequireSecureFirmwareUpdates {
    pub const NAME: &'static str = "require-secure-firmware-updates";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireSecureFirmwareUpdates {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for firmware updates without signature verification
        if code.contains("firmware") && code.contains("update") && !code.contains("signature") {
            diagnostics.push(create_secure_firmware_updates_diagnostic(
                0, 0,
                "Firmware updates should verify digital signatures for security"
            ));
        }

        // Check for OTA updates without rollback capability
        if code.contains("OTA") && !code.contains("rollback") && !code.contains("backup") {
            diagnostics.push(create_secure_firmware_updates_diagnostic(
                0, 0,
                "OTA updates should implement rollback capability for failed updates"
            ));
        }

        // Check for update process without secure channels
        if code.contains("download") && code.contains("firmware") && !code.contains("https") {
            diagnostics.push(create_secure_firmware_updates_diagnostic(
                0, 0,
                "Firmware downloads should use secure HTTPS channels"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireSecureFirmwareUpdates {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.92,
            suggestion: "Implement secure updates: `async function verifyAndUpdate(firmwareUrl, signature) { const firmware = await secureDownload(firmwareUrl); if (await verifySignature(firmware, signature)) { await updateWithRollback(firmware); } }`".to_string(),
            fix_code: Some("class SecureOTAUpdater {\n  async updateFirmware(updateInfo: FirmwareUpdate) {\n    try {\n      // Download firmware securely\n      const firmware = await this.secureDownload(updateInfo.url);\n      \n      // Verify signature\n      const isValid = await this.verifySignature(firmware, updateInfo.signature);\n      if (!isValid) {\n        throw new Error('Invalid firmware signature');\n      }\n      \n      // Create backup of current firmware\n      await this.createBackup();\n      \n      // Apply update\n      await this.applyUpdate(firmware);\n      \n      // Verify successful boot\n      if (!(await this.verifyBoot())) {\n        await this.rollback();\n        throw new Error('Update verification failed, rolled back');\n      }\n      \n    } catch (error) {\n      console.error('OTA update failed:', error);\n      await this.rollback();\n    }\n  }\n}".to_string()),
        }).collect()
    }
}

// Diagnostic creation functions
fn create_memory_constraints_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireMemoryConstraints::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "error".to_string(),
        category: "Performance".to_string(),
    }
}

fn create_power_optimization_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequirePowerOptimization::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "warning".to_string(),
        category: "Performance".to_string(),
    }
}

fn create_offline_resilience_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireOfflineResilience::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "error".to_string(),
        category: "Correctness".to_string(),
    }
}

fn create_sensor_data_validation_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSensorDataValidation::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "error".to_string(),
        category: "Correctness".to_string(),
    }
}

fn create_real_time_constraints_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireRealTimeConstraints::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "warning".to_string(),
        category: "Correctness".to_string(),
    }
}

fn create_hardware_abstraction_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireHardwareAbstraction::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "info".to_string(),
        category: "Style".to_string(),
    }
}

fn create_edge_data_processing_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireEdgeDataProcessing::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "warning".to_string(),
        category: "Performance".to_string(),
    }
}

fn create_device_health_monitoring_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireDeviceHealthMonitoring::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "warning".to_string(),
        category: "Correctness".to_string(),
    }
}

fn create_secure_firmware_updates_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSecureFirmwareUpdates::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "error".to_string(),
        category: "Security".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_require_memory_constraints_violation() {
        let code = r#"
        const allData = [];
        setInterval(() => {
            allData.push(readSensor());
        }, 100);
        "#;

        let rule = RequireMemoryConstraints;
        let diagnostics = rule.run(code);

        assert!(!diagnostics.is_empty());
        assert!(diagnostics[0].message.contains("circular buffers"));
    }

    #[test]
    fn test_require_memory_constraints_compliant() {
        let code = r#"
        const buffer = new CircularBuffer(100);
        setInterval(() => {
            buffer.add(readSensor());
            if (buffer.isFull()) {
                buffer.shift();
            }
        }, 100);
        "#;

        let rule = RequireMemoryConstraints;
        let diagnostics = rule.run(code);

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_require_power_optimization_violation() {
        let code = r#"
        setInterval(() => {
            fetch('/api/data');
        }, 100);
        "#;

        let rule = RequirePowerOptimization;
        let diagnostics = rule.run(code);

        assert!(!diagnostics.is_empty());
        assert!(diagnostics[0].message.contains("power"));
    }

    #[test]
    fn test_require_offline_resilience_violation() {
        let code = r#"
        async function getData() {
            const response = await fetch('/api/data');
            return response.json();
        }
        "#;

        let rule = RequireOfflineResilience;
        let diagnostics = rule.run(code);

        assert!(!diagnostics.is_empty());
        assert!(diagnostics[0].message.contains("offline"));
    }

    #[test]
    fn test_require_sensor_data_validation_violation() {
        let code = r#"
        function processSensorData() {
            const temperature = sensor.temperature;
            console.log(temperature);
        }
        "#;

        let rule = RequireSensorDataValidation;
        let diagnostics = rule.run(code);

        assert!(!diagnostics.is_empty());
        assert!(diagnostics[0].message.contains("validated"));
    }

    #[test]
    fn test_ai_enhancement_memory_constraints() {
        let rule = RequireMemoryConstraints;
        let diagnostics = vec![create_memory_constraints_diagnostic(0, 0, "test")];
        let suggestions = rule.ai_enhance("", &diagnostics);

        assert!(!suggestions.is_empty());
        assert!(suggestions[0].confidence > 0.9);
        assert!(suggestions[0].suggestion.contains("CircularBuffer"));
    }
}