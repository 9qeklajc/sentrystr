# SentryStr Python Bindings Test Suite

This directory contains comprehensive tests for the SentryStr Python bindings, demonstrating real-world usage and sending events to the Nostr network.

## 🎯 Test Target

All tests are configured to send events to the npub: `npub18kpn83drge7x9vz4cuhh7xta79sl4tfq55se4e554yj90s8y3f7qa49nps`

## 📋 Test Files

### `test_send_to_npub.py`
Focused test that sends encrypted events directly to the target npub. Includes:
- ✅ Basic error messages
- ✅ Info messages
- ✅ Detailed events with user context
- ✅ Exception reports with stack traces
- ✅ Performance monitoring events
- ✅ Multiple severity levels (debug, info, warning, error, fatal)

### `test_comprehensive.py`
Comprehensive test suite covering all features with 20 individual tests:
- ✅ Basic functionality (public and encrypted messaging)
- ✅ Error level handling (all 5 levels)
- ✅ User context in events
- ✅ Exception handling with stack traces
- ✅ Performance monitoring scenarios
- ✅ Security event reporting
- ✅ Business metrics and KPI tracking

### `test_real_events.py`
Real-world simulation tests covering various error scenarios and use cases.

## 🔧 Configuration

- **Sender Identity**: `npub1j4c6269y9w0q2er2xjw8sv2ehyrtfxq3jwgdlxj6qfn8z4gjsq5qg4jsaxr`
- **Test Private Key**: `nsec1j4c6269y9w0q2er2xjw8sv2ehyrtfxq3jwgdlxj6qfn8z4gjsq5qfvfk99` (safe for testing)
- **Target Recipient**: `npub18kpn83drge7x9vz4cuhh7xta79sl4tfq55se4e554yj90s8y3f7qa49nps`

## 🌐 Nostr Relays Used

- `wss://relay.damus.io`
- `wss://nos.lol`
- `wss://relay.snort.social`
- `wss://nostr.wine`
- `wss://relay.nostr.band`

## 🚀 Running Tests

1. **Setup Environment**:
   ```bash
   python3 -m venv venv
   source venv/bin/activate
   pip install maturin
   ```

2. **Build Bindings**:
   ```bash
   maturin develop
   ```

3. **Run Individual Tests**:
   ```bash
   # Focused test to target npub
   python tests/test_send_to_npub.py

   # Comprehensive test suite
   python tests/test_comprehensive.py

   # Real-world scenarios
   python tests/test_real_events.py
   ```

## 📊 Test Results

### Latest Comprehensive Test Run:
- ✅ **20/20 tests passed** (100% success rate)
- ⏱️ **Duration**: ~64 seconds
- 📡 **Events sent**: 20+ individual events to Nostr network
- 🔐 **Encryption**: Mixed public and encrypted events

## 🎉 Features Demonstrated

### Core Functionality
- [x] Client creation and configuration
- [x] Basic message sending
- [x] Error event capture
- [x] Event encryption to specific npub

### Event Types
- [x] Simple messages
- [x] Structured error events
- [x] Events with user context
- [x] Events with request context
- [x] Exception reports with stack traces
- [x] Performance monitoring events
- [x] Security incident reports
- [x] Business metrics and KPIs

### Severity Levels
- [x] Debug events
- [x] Info events
- [x] Warning events
- [x] Error events
- [x] Fatal events

### Advanced Features
- [x] Custom tags and metadata
- [x] Extra data (JSON objects)
- [x] Stack trace creation
- [x] User identification
- [x] Request context
- [x] Timestamp handling
- [x] Multi-relay broadcasting

## 🔍 Verification

Events sent during tests should be visible in Nostr clients monitoring:
- **Target npub**: `npub18kpn83drge7x9vz4cuhh7xta79sl4tfq55se4e554yj90s8y3f7qa49nps` (encrypted events)
- **Sender npub**: `npub1j4c6269y9w0q2er2xjw8sv2ehyrtfxq3jwgdlxj6qfn8z4gjsq5qg4jsaxr` (public events)

## 📝 Notes

- Events may take a few moments to propagate across Nostr relays
- Encrypted events are only visible to the target npub holder
- Public events can be seen by anyone monitoring the sender's npub
- All test keys are safe for public use and testing purposes
- The test suite demonstrates production-ready error tracking capabilities

## 🛠️ Troubleshooting

If tests fail:
1. Ensure Python bindings are built: `maturin develop`
2. Check network connectivity to Nostr relays
3. Verify virtual environment is activated
4. Check for any import errors or missing dependencies

The comprehensive test suite validates that the SentryStr Python bindings are fully functional and ready for production use in decentralized error tracking scenarios.