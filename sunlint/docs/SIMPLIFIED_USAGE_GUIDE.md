# 🚀 SunLint Performance - Simplified Usage Guide

## 🎯 **TÓM TẮT: 3 Commands Duy Nhất Bạn Cần Biết**

### **1. 🏃‍♂️ Quick Start (90% use cases)**
```bash
sunlint --all --input=src
```
✅ **Auto-detects** project size và chọn settings tối ưu  
✅ **Zero configuration** - chỉ cần chỉ định input folder  
✅ **Works everywhere** - small projects đến enterprise  

### **2. ⚡ Performance Modes (khi cần tùy chỉnh)**
```bash
# Fast scan (for testing/development)
sunlint --all --input=src --performance=fast

# Thorough analysis (for CI/CD)  
sunlint --all --input=src --performance=careful
```

### **3. 🛠️ Custom Timeout (khi project rất lớn)**
```bash
sunlint --all --input=src --timeout=120000  # 2 minutes
```

---

## 🤖 **Auto Performance Detection**

SunLint **tự động phát hiện** project size và chọn settings tối ưu:

| **Project Size** | **Files** | **Auto Settings** | **Timeout** |
|------------------|-----------|-------------------|-------------|
| **Small** | < 100 | Fast analysis | 30s |
| **Medium** | 100-500 | Balanced | 60s |
| **Large** | 500-1000 | Careful + progressive | 120s |
| **Enterprise** | 1000+ | Conservative + streaming | 300s |

### **Auto-Detection Logic**
```bash
# ✅ SunLint tự động:
# - Đếm số files trong input folder
# - Phát hiện TypeScript, Node.js project
# - Chọn timeout và batch size phù hợp
# - Bật progressive results cho large projects

sunlint --all --input=src  # Làm tất cả tự động!
```

---

## 📋 **Common Usage Patterns**

### **Development (hàng ngày)**
```bash
# Quick feedback loop
sunlint --rules=C019,C041,S027 --input=src

# Check specific files
sunlint --all --input=src/components --performance=fast
```

### **Code Review/PR**
```bash
# Check changed files only
sunlint --all --changed-files

# Quick but comprehensive
sunlint --all --input=src --performance=fast --verbose
```

### **CI/CD Pipeline**
```bash
# Thorough analysis with auto-optimization
sunlint --all --input=src --format=json --output=results.json

# For large projects in CI
sunlint --all --input=src --performance=careful --quiet
```

### **Weekly Code Quality Review**
```bash
# Full analysis with detailed reporting
sunlint --all --input=src --verbose --format=table
```

---

## 🚨 **Troubleshooting Simplified**

### **❌ Getting Timeouts?**
```bash
# Try longer timeout
sunlint --all --input=src --timeout=120000

# Or limit files
sunlint --all --input=src --max-files=500
```

### **❌ Taking Too Long?**
```bash
# Use fast mode
sunlint --all --input=src --performance=fast

# Or check specific rules
sunlint --rules=C002,C019,S027 --input=src
```

### **❌ Memory Issues?**
```bash
# Automatic handling - just use auto mode
sunlint --all --input=src --performance=auto
```

---

## 🎛️ **Migration from Complex Commands**

### **BEFORE (v3.x - Complex)**
```bash
# ❌ Too many options to remember
sunlint --all --input=src \
  --performance-profile=balanced \
  --adaptive-timeout \
  --max-memory=2GB \
  --batch-size=10 \
  --progressive-results \
  --verbose
```

### **AFTER (v4.x - Simplified)**
```bash
# ✅ Simple and effective
sunlint --all --input=src --verbose
```

### **Advanced Users Can Still Customize**
```bash
# For power users who need control
sunlint --all --input=src --performance=careful --timeout=180000
```

---

## 📊 **Performance Comparison**

| **Command** | **Small Project** | **Large Project** | **Enterprise** |
|-------------|-------------------|-------------------|----------------|
| `--performance=auto` | ~10s | ~60s | ~120s |
| `--performance=fast` | ~5s | ~30s | ~60s |
| `--performance=careful` | ~15s | ~90s | ~180s |

---

## ✅ **Best Practices**

### **🎯 DO (Recommended)**
```bash
✅ sunlint --all --input=src                    # Let auto-detection work
✅ sunlint --all --input=src --verbose          # See what's happening  
✅ sunlint --quality --input=src --performance=fast  # Quick quality check
✅ sunlint --all --changed-files                # Only check changes
```

### **❌ DON'T (Avoid)**
```bash
❌ sunlint --all --input=src --performance-profile=conservative --batch-size=5 --streaming-analysis
   # Too complex - just use --performance=careful

❌ sunlint --all --input=src --timeout=5000     
   # Too short - let auto-detection choose

❌ sunlint --all --input=huge-project           
   # Missing performance hint - add --performance=careful
```

---

## 🏆 **Success Metrics**

### **✅ Simplified CLI Achieved**
- **3 main commands** cover 90% of use cases
- **Auto-detection** eliminates guesswork  
- **Zero configuration** for most projects
- **Predictable performance** across project sizes

### **✅ Backward Compatibility**
- Old commands still work but show deprecation warnings
- Gradual migration path for existing users
- Advanced options available for power users

---

## 🚀 **Quick Start Checklist**

- [ ] **Update to SunLint v4.x** with auto-performance
- [ ] **Use basic command**: `sunlint --all --input=src`
- [ ] **Add --verbose** if you want to see progress
- [ ] **Use --performance=fast** for quick checks
- [ ] **Use --performance=careful** for thorough analysis
- [ ] **Test with your project** to validate performance

---

**🎯 Bottom Line: Chỉ cần nhớ `sunlint --all --input=src` - mọi thứ khác được tự động optimize!**

*🚀 Simple • ⚡ Fast • 🎯 Effective*
