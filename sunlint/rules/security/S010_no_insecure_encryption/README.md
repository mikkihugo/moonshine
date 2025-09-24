# S010 - Must use cryptographically secure random number generators (CSPRNG)

## Overview
Quy tắc này phát hiện việc sử dụng các hàm tạo số ngẫu nhiên không an toàn (như `Math.random()`) cho các mục đích bảo mật, có thể dẫn đến các lỗ hổng bảo mật nghiêm trọng do tính có thể dự đoán được.

## OWASP Classification
- **Category**: A02:2021 - Cryptographic Failures
- **CWE**: CWE-338 - Use of Cryptographically Weak Pseudo-Random Number Generator (PRNG)
- **Severity**: Error
- **Impact**: High (Predictable tokens, weak encryption keys, authentication bypass)

## Vấn đề
Khi sử dụng các hàm tạo số ngẫu nhiên không an toàn cho mục đích bảo mật:

1. **Tính dự đoán**: `Math.random()` và các hàm tương tự có thể được dự đoán bởi kẻ tấn công
2. **Weak entropy**: Không đủ entropy để tạo ra các giá trị thực sự ngẫu nhiên
3. **Seed attacks**: Có thể bị tấn công thông qua việc dự đoán seed
4. **Brute force**: Dễ dàng bị brute force do không gian giá trị hạn chế

## Các trường hợp vi phạm

### 1. Sử dụng Math.random() cho security tokens
```javascript
// ❌ Vi phạm - Math.random() không an toàn cho security
const sessionToken = Math.random().toString(36).substring(2);
const apiKey = Math.floor(Math.random() * 1000000);
const resetCode = Math.random().toString().substring(2, 8);

// ❌ Vi phạm - tạo password reset token
function generateResetToken() {
  return Math.random().toString(36).substring(2, 15) + 
         Math.random().toString(36).substring(2, 15);
}

// ❌ Vi phạm - tạo JWT secret
const jwtSecret = Math.random().toString(36);
```

### 2. Sử dụng timestamp cho random generation
```javascript
// ❌ Vi phạm - timestamp có thể dự đoán được
const userId = Date.now().toString();
const sessionId = new Date().getTime().toString();
const nonce = performance.now().toString();

// ❌ Vi phạm - kết hợp timestamp với Math.random()
const token = Date.now() + '-' + Math.random().toString(36);
```

### 3. Sử dụng cho mã hóa và authentication
```javascript
// ❌ Vi phạm - tạo encryption key không an toàn
const encryptionKey = Math.random().toString(36).repeat(3);

// ❌ Vi phạm - tạo salt cho password hashing
const salt = Math.random().toString(36).substring(2);
bcrypt.hash(password, salt); // Nguy hiểm!

// ❌ Vi phạm - tạo CSRF token
const csrfToken = Math.floor(Math.random() * 1000000000);
```

### 4. Tạo unique identifiers không an toàn
```javascript
// ❌ Vi phạm - tạo user ID
const generateUserId = () => Math.random().toString(36).substring(2);

// ❌ Vi phạm - tạo file upload path
const uploadPath = `/uploads/${Math.random().toString(36)}/file.jpg`;

// ❌ Vi phạm - tạo temporary filename
const tempFile = `temp_${Date.now()}_${Math.random()}.tmp`;
```

## Giải pháp an toàn

### 1. Sử dụng crypto.randomBytes()
```javascript
// ✅ An toàn - sử dụng crypto.randomBytes()
const crypto = require('crypto');

const sessionToken = crypto.randomBytes(32).toString('hex');
const apiKey = crypto.randomBytes(16).toString('base64');
const resetCode = crypto.randomBytes(3).toString('hex'); // 6 hex chars
```

### 2. Sử dụng crypto.randomUUID()
```javascript
// ✅ An toàn - sử dụng crypto.randomUUID()
const sessionId = crypto.randomUUID();
const userId = crypto.randomUUID();
const requestId = crypto.randomUUID();
```

### 3. Sử dụng crypto.randomInt()
```javascript
// ✅ An toàn - sử dụng crypto.randomInt()
const otpCode = crypto.randomInt(100000, 999999); // 6-digit OTP
const randomPort = crypto.randomInt(3000, 9000);
const challengeNumber = crypto.randomInt(1, 1000000);
```

### 4. Sử dụng cho mã hóa an toàn
```javascript
// ✅ An toàn - tạo encryption key
const encryptionKey = crypto.randomBytes(32); // 256-bit key

// ✅ An toàn - tạo salt cho password hashing
const saltRounds = 12;
const hashedPassword = await bcrypt.hash(password, saltRounds);

// ✅ An toàn - tạo IV cho encryption
const iv = crypto.randomBytes(16);
const cipher = crypto.createCipher('aes-256-cbc', key, iv);
```

### 5. Sử dụng thư viện an toàn
```javascript
// ✅ An toàn - sử dụng nanoid
const { nanoid } = require('nanoid');
const id = nanoid(); // URL-safe, secure ID

// ✅ An toàn - sử dụng uuid v4
const { v4: uuidv4 } = require('uuid');
const userId = uuidv4();

// ✅ An toàn - sử dụng secure-random
const secureRandom = require('secure-random');
const randomBytes = secureRandom(32, { type: 'Buffer' });
```

### 6. Express.js session security
```javascript
// ✅ An toàn - cấu hình Express session
const session = require('express-session');

app.use(session({
  genid: () => crypto.randomUUID(), // Secure session ID
  secret: crypto.randomBytes(64).toString('hex'), // Secure secret
  resave: false,
  saveUninitialized: false,
  cookie: {
    secure: true, // HTTPS only
    httpOnly: true,
    maxAge: 1800000 // 30 minutes
  }
}));
```

### 7. JWT token generation
```javascript
// ✅ An toàn - tạo JWT với secure secret
const jwtSecret = crypto.randomBytes(64).toString('hex');

const token = jwt.sign(
  { userId, exp: Math.floor(Date.now() / 1000) + 3600 },
  jwtSecret,
  { algorithm: 'HS256' }
);
```

## Phương pháp phát hiện

Rule này sử dụng heuristic analysis để phát hiện:

1. **Pattern matching**: Tìm kiếm các pattern như `Math.random()`, `Date.now()`, `performance.now()`
2. **Context analysis**: Phân tích ngữ cảnh để xác định có phải mục đích bảo mật không
3. **Function analysis**: Kiểm tra tên function có chứa từ khóa bảo mật
4. **Variable analysis**: Phân tích tên biến có liên quan đến bảo mật
5. **Surrounding context**: Kiểm tra các dòng xung quanh để xác định context

## Cấu hình

```json
{
  "S010": {
    "enabled": true,
    "severity": "error",
    "excludePatterns": [
      "test/**",
      "**/*.test.js",
      "**/*.spec.js",
      "**/demo/**",
      "**/examples/**"
    ]
  }
}
```

## Best Practices

1. **Luôn sử dụng CSPRNG**: Sử dụng `crypto.randomBytes()`, `crypto.randomUUID()`, `crypto.randomInt()` cho mục đích bảo mật
2. **Đủ entropy**: Đảm bảo đủ entropy cho các giá trị random (ít nhất 128 bits cho tokens)
3. **Secure storage**: Lưu trữ các giá trị random một cách an toàn
4. **Time-limited**: Sử dụng expiration time cho các tokens
5. **Proper seeding**: Đảm bảo CSPRNG được seed đúng cách
6. **Regular rotation**: Thường xuyên rotate các keys và secrets

## Các trường hợp ngoại lệ

Rule này sẽ KHÔNG báo lỗi trong các trường hợp sau:

```javascript
// ✅ Không báo lỗi - sử dụng cho UI/animation
const animationDelay = Math.random() * 1000;
const chartColor = `hsl(${Math.random() * 360}, 50%, 50%)`;

// ✅ Không báo lỗi - game logic
const diceRoll = Math.floor(Math.random() * 6) + 1;
const enemySpawn = Math.random() < 0.3;

// ✅ Không báo lỗi - demo/test data
const mockData = Array.from({length: 10}, () => ({
  value: Math.random() * 100
}));
```

## Tài liệu tham khảo

- [OWASP A02:2021 - Cryptographic Failures](https://owasp.org/Top10/A02_2021-Cryptographic_Failures/)
- [CWE-338: Use of Cryptographically Weak Pseudo-Random Number Generator](https://cwe.mitre.org/data/definitions/338.html)
- [Node.js Crypto Documentation](https://nodejs.org/api/crypto.html)
- [NIST Guidelines for Random Number Generation](https://csrc.nist.gov/publications/detail/sp/800-90a/rev-1/final)
- [Secure Random Number Generation Best Practices](https://cheatsheetseries.owasp.org/cheatsheets/Cryptographic_Storage_Cheat_Sheet.html)
