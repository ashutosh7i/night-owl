# Night Owl 🦉

A versatile, tag-based logging system designed for cross-platform applications. Night Owl provides a straightforward way to collect, store, and retrieve logs with custom tags, built with performance and simplicity in mind.

## 🎯 The Problem

While developing applications across multiple platforms, I needed a straightforward, efficient logging service that could collect logs with tags from various sources. Existing solutions like Sentry and Datadog were either not suitable for simple log lookup, too expensive, or overly complex for my needs.

## 💡 The Solution

Night Owl 🦉 is a versatile, tag-based logging system designed to be simple, fast, and platform-independent. Built with C++ and SQLite at its core, it allows developers to define custom tags and easily send logs to a server for later retrieval based on these tags.

## ✨ Key Features

- 🏷️ Custom tag definition
- 📝 Two components for each log entry: message and data
- 📌 Support for predefined tags like User, System, Other
- 🔄 Fallback type for unknown tags
- ⚡ Fast and efficient C++ core implementation
- 💾 SQLite database for robust storage and quick retrieval
- 🐳 Docker image for easy deployment
- 📦 SDKs for multiple platforms (Flutter, JavaScript)
- 🔌 Extensibility to support most programming language

## 🚀 How It Works

1. Deploy Night Owl using the provided Docker image
2. Define your tags in a configuration file
3. Send logs from your application using the platform-specific SDK
4. Logs are stored in the SQLite database with their associated tags
5. Retrieve logs later using tag-based queries

## 📝 Example Usage

### Sending Logs

```javascript
// Sending a log with user tag
NightOwl.log("User", "user registered", {
  "userId": "12345",
  "email": "user@example.com"
});

// Sending a log with system tag
NightOwl.log("System", "Application startup", {
  "version": "1.0.0",
  "buildNumber": "42"
});
```

### Retrieving Logs

```javascript
// Get all logs with a specific tag
const userLogs = await NightOwl.getLogs("User");

// Get logs within a time range
const recentLogs = await NightOwl.getLogs(
  "System",
  {
    startTime: new Date(Date.now() - 24 * 60 * 60 * 1000),
    endTime: new Date()
  }
);

// Search logs with specific criteria
const filteredLogs = await NightOwl.searchLogs({
  tags: ["User", "System"],
  query: "error",
  limit: 100
});
```

## ⚙️ Custom Tag Configuration

Create a `log_config.json` file in your project:

```json
{
  "tags": {
    "User": {
      "retention": "30d",
      "priority": "normal"
    },
    "System": {
      "retention": "90d",
      "priority": "high"
    },
    "Debug": {
      "retention": "7d",
      "priority": "low"
    }
  }
}
```

## 🏗️ Architecture

Night Owl consists of three main components:

1. **Core Engine (C++)**
   - Handles log processing and storage
   - Optimized for performance
   - Platform-independent implementation

2. **SQLite Database**
   - Manages persistent storage
   - Efficient query execution
   - Robust data integrity

3. **Platform SDKs**
   - Native implementations for each platform
   - Consistent API across languages
   - Optimized for each environment

## 🔍 API Reference

| Method | Description |
|--------|-------------|
| `log(String tag, String message, Object data)` | Logs a new entry |
| `getLogs(String tag, Options options)` | Retrieves logs by tag |
| `searchLogs(SearchOptions options)` | Searches logs with criteria |
| `clearLogs(String tag)` | Clears logs for a specific tag |

## 🎯 Getting Started

Coming soon...

## 📦 Available SDKs

- Flutter: [Link to Flutter package] soon
- JavaScript: [Link to npm package] soon
- Python: Coming soon
- Java: Coming soon
- Go: Coming soon

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

Made with ❤️ 
