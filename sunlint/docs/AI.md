# 🤖 AI-Powered Analysis

Sunlint supports AI-powered code analysis alongside traditional pattern-based analysis for more intelligent and context-aware rule checking.

## Overview

- **🎯 Smart Analysis**: Uses AI to understand code context and intent
- **🔄 Fallback Strategy**: Automatically falls back to pattern analysis if AI fails
- **⚡ Performance**: AI analysis runs per-file with caching support
- **🔧 Configurable**: Multiple AI providers and models supported

## Configuration

### In `.sunlint.json`:

```json
{
  "ai": {
    "enabled": true,
    "provider": "openai",
    "model": "gpt-4o-mini",
    "apiKey": "${OPENAI_API_KEY}",
    "fallbackToPattern": true
  }
}
```

### Environment Variables:

```bash
export OPENAI_API_KEY="your-openai-api-key"
```

## Supported Providers

### OpenAI
- **Models**: `gpt-4`, `gpt-4o-mini`, `gpt-3.5-turbo`
- **API Key**: Required via `OPENAI_API_KEY` environment variable
- **Cost**: Pay-per-use based on OpenAI pricing

### GitHub Copilot (Planned)
- **Integration**: VS Code extension integration
- **Models**: GitHub Copilot models
- **Authentication**: VS Code Copilot session

## AI-Enhanced Rules

### C019 - Log Level Usage
✅ **AI-Enabled**: Understands code context to determine appropriate log levels

**AI Analysis Features:**
- **Context Understanding**: Analyzes surrounding code to determine error criticality
- **Intent Recognition**: Understands whether errors are expected or exceptional
- **Semantic Analysis**: Goes beyond pattern matching to understand meaning

**Example:**
```typescript
// AI understands this is a validation error, suggests warn level
if (!user.email) {
  console.error('Missing email'); // AI: Should use console.warn()
}

// AI understands this is a critical system error, keeps error level
try {
  await database.connect();
} catch (error) {
  console.error('Database connection failed:', error); // AI: Appropriate error level
}
```

## Usage

### CLI Commands

**Enable AI for specific rule:**
```bash
sunlint --rule=C019 --input=src --ai
```

**Enable AI for all rules:**
```bash
sunlint --quality --input=src --ai
```

**Debug AI analysis:**
```bash
sunlint --rule=C019 --input=src --ai --verbose
```

### VS Code Integration

1. **Debug Configuration**: Use "Debug Sunlint - AI Analysis"
2. **Task**: Run "Sunlint: AI Analysis Test"
3. **Set API Key**: Configure `OPENAI_API_KEY` in environment

## Output Differences

### Pattern Analysis Output:
```
WARNING: Error log level used for non-critical issue - should use warn/info level
  at src/user.ts:15:5 (C019)
```

### AI Analysis Output:
```
WARNING: Email validation error should use console.warn() - this is user input validation, not a system error
  at src/user.ts:15:5 (C019)
  Suggestion: Change to console.warn('User email validation failed:', email)
```

## Performance Considerations

- **Caching**: AI responses are cached per file content hash
- **Concurrency**: AI calls are made concurrently with rate limiting
- **Timeout**: 30-second timeout per AI request
- **Cost**: Monitor API usage in OpenAI dashboard

## Troubleshooting

### Common Issues

**API Key Not Found:**
```bash
⚠️  AI API key not found, falling back to pattern analysis
```
Solution: Set `OPENAI_API_KEY` environment variable

**API Rate Limit:**
```bash
AI Analysis failed: OpenAI API error: 429 Too Many Requests
```
Solution: Reduce `maxConcurrentRules` in config or wait

**Network Issues:**
```bash
AI Analysis failed: OpenAI API error: Network timeout
```
Solution: Check internet connection, increase `timeoutMs`

### Debug AI Issues

1. **Enable verbose mode**: `--verbose`
2. **Check API key**: `echo $OPENAI_API_KEY`
3. **Test connection**: Use debug configuration
4. **Check API quota**: Visit OpenAI dashboard

## Future Enhancements

- **🔄 GitHub Copilot Integration**: Direct integration with VS Code Copilot
- **📊 Custom Models**: Support for fine-tuned models
- **🎯 Rule-Specific Prompts**: Specialized prompts per rule type
- **💾 Smart Caching**: Semantic caching across similar code patterns
- **📈 Analytics**: AI vs Pattern analysis effectiveness metrics

## Cost Estimation

**OpenAI API Costs** (approximate):
- **gpt-4o-mini**: ~$0.001 per 1K tokens
- **gpt-4**: ~$0.03 per 1K tokens
- **Average file**: ~500 tokens
- **1000 files with gpt-4o-mini**: ~$0.50

**Recommendation**: Start with `gpt-4o-mini` for cost-effectiveness.
