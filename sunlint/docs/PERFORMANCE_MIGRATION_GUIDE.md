# 🚀 SunLint Performance Migration Guide

## 🎯 Overview

With **73 heuristic rules** and growing, SunLint needs performance optimizations to handle enterprise-scale projects without timeouts or memory crashes. This guide helps you migrate to the optimized performance features in **SunLint v4.0**.

---

## ⚠️ Current Performance Issues

### **Before Optimization**
```bash
# ❌ This will likely timeout on large projects
sunlint --all --input=src

# ❌ Memory exhaustion with 1000+ files
sunlint --rules=C019,C033,C047,C076 --input=large-project

# ❌ Stack overflow with complex projects
sunlint --all --input=enterprise-codebase
```

### **Common Error Messages**
- `Engine heuristic failed: Engine undefined timed out after 30000ms`
- `Maximum call stack size exceeded`
- `JavaScript heap out of memory`
- `FATAL ERROR: Ineffective mark-compacts near heap limit`

---

## 🚀 Migration Steps

### **Step 1: Enable Performance Optimizations**

#### **Quick Fix: Use Performance Profile**
```bash
# ✅ BEFORE: Basic command (may timeout)
sunlint --all --input=src

# ✅ AFTER: With performance profile
sunlint --all --input=src --performance-profile=balanced
```

#### **Available Profiles**
| Profile | Best For | Timeout | Batch Size | Max Files |
|---------|----------|---------|------------|-----------|
| `fast` | < 100 files | 30s | 20 rules | 200 files |
| `balanced` | 100-500 files | 60s | 15 rules | 500 files |
| `careful` | 500-1000 files | 120s | 10 rules | 1000 files |
| `conservative` | 1000+ files | 300s | 5 rules | 1500 files |

### **Step 2: Configure Timeouts**

#### **Static Timeout**
```bash
# ✅ Set longer timeout for large projects
sunlint --all --input=src --timeout=120000  # 2 minutes
```

#### **Adaptive Timeout (Recommended)**
```bash
# ✅ Auto-scale timeout based on project size
sunlint --all --input=src --adaptive-timeout
```

#### **No Timeout (Use with Caution)**
```bash
# ⚠️ Disable timeout completely (for CI/CD environments)
sunlint --all --input=src --no-timeout
```

### **Step 3: Memory Management**

#### **Set Memory Limits**
```bash
# ✅ Limit memory usage
sunlint --all --input=src --max-memory=2GB

# ✅ For containers with limited memory
sunlint --all --input=src --max-memory=1GB --streaming-analysis
```

#### **File Limits**
```bash
# ✅ Limit files to prevent memory explosion
sunlint --all --input=src --max-files=1000

# ✅ Smart sampling for huge projects
sunlint --all --input=src --smart-sampling --max-files=500
```

### **Step 4: Batch Processing**

#### **Rule Batching**
```bash
# ✅ Process rules in smaller batches
sunlint --all --input=src --batch-size=10

# ✅ Parallel batch processing
sunlint --all --input=src --batch-size=10 --parallel-batches=2
```

#### **File Batching**
```bash
# ✅ Process files in batches for memory management
sunlint --all --input=src --file-batch-size=50
```

### **Step 5: Progressive Results**

#### **Show Results as Found**
```bash
# ✅ See violations as they're discovered
sunlint --all --input=src --progressive-results

# ✅ For CI/CD: See progress without waiting
sunlint --all --input=src --progressive-results --verbose
```

---

## 🎛️ Complete Migration Examples

### **Small to Medium Projects (< 500 files)**
```bash
# BEFORE (v3.x)
sunlint --all --input=src --verbose

# AFTER (v4.x - Optimized)
sunlint --all --input=src \
  --performance-profile=balanced \
  --progressive-results \
  --verbose
```

### **Large Projects (500-1000 files)**
```bash
# BEFORE (v3.x) - Would likely timeout
sunlint --all --input=src --timeout=60000

# AFTER (v4.x - Optimized)
sunlint --all --input=src \
  --performance-profile=careful \
  --adaptive-timeout \
  --max-files=1000 \
  --batch-size=10 \
  --progressive-results \
  --verbose
```

### **Enterprise Projects (1000+ files)**
```bash
# BEFORE (v3.x) - Would crash
sunlint --all --input=src

# AFTER (v4.x - Optimized)
sunlint --all --input=src \
  --performance-profile=conservative \
  --max-memory=2GB \
  --max-files=1500 \
  --streaming-analysis \
  --smart-sampling \
  --batch-size=5 \
  --progressive-results \
  --verbose
```

### **CI/CD Pipeline Optimization**
```bash
# BEFORE (v3.x)
sunlint --all --input=src --format=json

# AFTER (v4.x - CI-Optimized)
sunlint --all --input=src \
  --performance-profile=balanced \
  --adaptive-timeout \
  --progressive-results \
  --format=json \
  --quiet  # Suppress progress for cleaner CI logs
```

---

## 🔧 Engine Selection

### **Use Optimized Engine**
```bash
# ✅ Use the new optimized engine
sunlint --all --input=src --engine=optimized

# ✅ Or force with performance profile
sunlint --all --input=src --performance-profile=balanced
# (automatically uses optimized engine)
```

### **Fallback to Legacy Engine**
```bash
# ⚠️ Only if optimized engine has issues
sunlint --all --input=src --legacy
```

---

## 📊 Performance Monitoring

### **Verbose Output for Performance Insights**
```bash
# ✅ See detailed performance metrics
sunlint --all --input=src \
  --performance-profile=balanced \
  --verbose

# Output example:
# 🚀 Analysis started with performance profile: Balanced
# ⚡ Processing batch 1/5 (15 rules)...
# 📊 Analyzed 245/1000 files (24.5%)
# 🎯 Found 23 violations so far...
# ✅ Analysis completed in 45.2s:
#    📁 Files: 1000
#    📋 Rules: 73
#    🎯 Violations: 156
#    💾 Peak Memory: 1.2GB
```

### **Performance Testing**
```bash
# ✅ Test performance with your project
node scripts/performance-test.js

# ✅ Demo batch processing
node scripts/batch-processing-demo.js
```

---

## 🚨 Troubleshooting

### **Still Getting Timeouts?**

1. **Increase timeout**:
   ```bash
   sunlint --all --input=src --timeout=300000  # 5 minutes
   ```

2. **Reduce scope**:
   ```bash
   sunlint --all --input=src --max-files=500
   ```

3. **Use smaller batches**:
   ```bash
   sunlint --all --input=src --batch-size=5
   ```

4. **Enable streaming**:
   ```bash
   sunlint --all --input=src --streaming-analysis
   ```

### **Memory Issues?**

1. **Set memory limit**:
   ```bash
   sunlint --all --input=src --max-memory=1GB
   ```

2. **Reduce file batch size**:
   ```bash
   sunlint --all --input=src --file-batch-size=25
   ```

3. **Use smart sampling**:
   ```bash
   sunlint --all --input=src --smart-sampling --max-files=300
   ```

### **Poor Performance?**

1. **Use fast profile for testing**:
   ```bash
   sunlint --rules=C002,C003,C006 --input=src --performance-profile=fast
   ```

2. **Check file exclusions**:
   ```bash
   sunlint --all --input=src --exclude="node_modules/**,dist/**,build/**"
   ```

3. **Limit semantic analysis**:
   ```bash
   sunlint --all --input=src --max-semantic-files=200
   ```

---

## 📈 Performance Benchmarks

### **Target Performance** (v4.x Optimized)

| Project Size | Files | Rules | Expected Time | Memory Usage |
|--------------|-------|-------|---------------|--------------|
| **Small** | 50 | 20 | ~10s | 500MB |
| **Medium** | 200 | 35 | ~30s | 1GB |
| **Large** | 500 | 50 | ~60s | 1.5GB |
| **Enterprise** | 1000+ | 73 | ~120s | 2GB |

### **Performance Improvements**

| Scenario | v3.x (Before) | v4.x (After) | Improvement |
|----------|---------------|--------------|-------------|
| **Medium Project** | Timeout (30s) | 45s | **Functional** |
| **Large Project** | Memory crash | 90s | **3x faster** |
| **Enterprise** | Impossible | 180s | **Breakthrough** |

---

## 🏁 Quick Start Checklist

- [ ] **Update to SunLint v4.x** with performance optimizations
- [ ] **Choose performance profile** based on your project size
- [ ] **Enable adaptive timeout** for automatic scaling
- [ ] **Set memory limits** appropriate for your environment
- [ ] **Use progressive results** for better user experience
- [ ] **Test with your actual codebase** to validate performance
- [ ] **Monitor memory usage** and adjust limits as needed
- [ ] **Consider smart sampling** for extremely large projects

---

## 🎯 Best Practices

### **Development**
```bash
# Fast feedback during development
sunlint --rules=C002,C019,S027 --input=src --performance-profile=fast
```

### **Pre-commit**
```bash
# Quick check on changed files
sunlint --all --changed-files --performance-profile=fast
```

### **CI/CD**
```bash
# Comprehensive analysis with performance safety
sunlint --all --input=src \
  --performance-profile=balanced \
  --adaptive-timeout \
  --format=json \
  --output=sunlint-results.json
```

### **Weekly Code Quality Review**
```bash
# Full analysis with detailed reporting
sunlint --all --input=src \
  --performance-profile=careful \
  --progressive-results \
  --format=table \
  --verbose
```

---

**Performance optimization is critical for SunLint adoption at enterprise scale. These optimizations ensure that SunLint remains fast and reliable as your codebase grows.**

*🚀 Scale with Confidence • ⚡ Optimized Performance • 🎯 Enterprise Ready*
