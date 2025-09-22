#!/usr/bin/env python3
"""
Comprehensive test suite for SentryStr Python bindings
Includes both encrypted (to specific npub) and public events
"""

import sys
import os
import time
import json
from datetime import datetime, timedelta

# Add the built module to the path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'python'))

try:
    import sentrystr
    from key_generator import generate_test_keys, get_target_pubkey
    print("✓ Successfully imported sentrystr and key generator")
except ImportError as e:
    print(f"❌ Failed to import: {e}")
    print("Make sure to build the Python bindings first with: maturin develop")
    sys.exit(1)

# Configuration
TARGET_NPUB = "npub18kpn83drge7x9vz4cuhh7xta79sl4tfq55se4e554yj90s8y3f7qa49nps"
TARGET_PUBKEY = get_target_pubkey()  # Use npub directly

# Generate dynamic keys for this test run
DYNAMIC_KEYS = generate_test_keys()
SENDER_PRIVATE_KEY = DYNAMIC_KEYS['private_key']
SENDER_PUBLIC_KEY = DYNAMIC_KEYS['public_key_hex']

RELAYS = [
    "wss://relay.damus.io",
    "wss://nos.lol",
    "wss://relay.snort.social",
    "wss://nostr.wine",
    "wss://relay.nostr.band",
    "wss://nostr.chaima.info"
]

class SentryStrTestSuite:
    def __init__(self):
        self.test_counter = 0
        self.passed_tests = 0
        self.failed_tests = 0

    def log_test(self, test_name, success=True):
        self.test_counter += 1
        if success:
            self.passed_tests += 1
            print(f"✅ Test {self.test_counter}: {test_name} - PASSED")
        else:
            self.failed_tests += 1
            print(f"❌ Test {self.test_counter}: {test_name} - FAILED")

    def create_public_client(self):
        """Create client for public events"""
        config = sentrystr.Config(SENDER_PRIVATE_KEY, RELAYS)
        return sentrystr.NostrSentryClient(config)

    def create_encrypted_client(self):
        """Create client for encrypted events to target npub"""
        config = sentrystr.Config(SENDER_PRIVATE_KEY, RELAYS)
        config.with_encryption(TARGET_PUBKEY)
        return sentrystr.NostrSentryClient(config)

    def test_basic_functionality(self):
        """Test basic client creation and simple messages"""
        print("\n🧪 Testing Basic Functionality")

        try:
            # Public client
            public_client = self.create_public_client()
            public_client.capture_message("Public test message from SentryStr Python bindings")
            self.log_test("Public message sending")

            # Encrypted client
            encrypted_client = self.create_encrypted_client()
            encrypted_client.capture_message("Encrypted test message to specific npub")
            self.log_test("Encrypted message sending")

            return True
        except Exception as e:
            print(f"Error in basic functionality test: {e}")
            self.log_test("Basic functionality", False)
            return False

    def test_error_levels(self):
        """Test different error levels"""
        print("\n🧪 Testing Error Levels")

        try:
            client = self.create_public_client()

            levels = [
                ("debug", "Debug: User session initialized"),
                ("info", "Info: API request successful"),
                ("warning", "Warning: High memory usage detected"),
                ("error", "Error: Database query failed"),
                ("fatal", "Fatal: System crash imminent")
            ]

            for level_name, message in levels:
                event = sentrystr.Event()
                event.with_message(f"[{level_name.upper()}] {message}")
                event.with_level(sentrystr.Level(level_name))
                event.with_tag("test_type", "level_testing")
                event.with_tag("severity", level_name)

                client.capture_event(event)
                self.log_test(f"Level {level_name} event")
                time.sleep(0.5)

            return True
        except Exception as e:
            print(f"Error in error levels test: {e}")
            self.log_test("Error levels", False)
            return False

    def test_user_context(self):
        """Test user context in events"""
        print("\n🧪 Testing User Context")

        try:
            client = self.create_encrypted_client()  # Send to target npub

            # Create user with full context
            user = sentrystr.User()
            user.with_id(f"dynamic_user_{int(time.time())}")
            user.with_email("alice@example.com")
            user.with_username("alice_developer")

            event = sentrystr.Event()
            event.with_message("User performed unauthorized action")
            event.with_level(sentrystr.Level("warning"))
            event.with_user(user)
            event.with_tag("security", "unauthorized_access")
            event.with_tag("action", "file_download")
            event.with_tag("test_type", "dynamic_keys")
            event.add_extra("file_path", "/sensitive/data.txt")
            event.add_extra("user_ip", "192.168.1.100")
            event.add_extra("timestamp", datetime.now().isoformat())
            event.add_extra("sender_pubkey", SENDER_PUBLIC_KEY)

            client.capture_event(event)
            self.log_test("User context in event")

            return True
        except Exception as e:
            print(f"Error in user context test: {e}")
            self.log_test("User context", False)
            return False

    def test_exception_handling(self):
        """Test exception reporting with stack traces"""
        print("\n🧪 Testing Exception Handling")

        try:
            client = self.create_public_client()

            # Create a realistic stack trace
            frames = []

            # Main entry point
            main_frame = sentrystr.Frame("main.py")
            main_frame.with_function("main")
            main_frame.with_lineno(15)
            frames.append(main_frame)

            # Service layer
            service_frame = sentrystr.Frame("services/user_service.py")
            service_frame.with_function("authenticate_user")
            service_frame.with_lineno(67)
            frames.append(service_frame)

            # Repository layer
            repo_frame = sentrystr.Frame("repositories/user_repository.py")
            repo_frame.with_function("find_user_by_email")
            repo_frame.with_lineno(89)
            frames.append(repo_frame)

            # Database layer (where error occurred)
            db_frame = sentrystr.Frame("database/connection.py")
            db_frame.with_function("execute_query")
            db_frame.with_lineno(134)
            frames.append(db_frame)

            # Create stacktrace and exception
            stacktrace = sentrystr.Stacktrace(frames)
            exception = sentrystr.Exception("PostgreSQLError", "relation 'users' does not exist")
            exception.with_stacktrace(stacktrace)

            # Create event
            event = sentrystr.Event()
            event.with_message("Database table missing during user authentication")
            event.with_level(sentrystr.Level("error"))
            event.with_exception(exception)
            event.with_tag("database", "postgresql")
            event.with_tag("operation", "user_auth")
            event.add_extra("query", "SELECT * FROM users WHERE email = $1")
            event.add_extra("database_name", "production_db")

            client.capture_event(event)
            self.log_test("Exception with stack trace")

            return True
        except Exception as e:
            print(f"Error in exception handling test: {e}")
            self.log_test("Exception handling", False)
            return False

    def test_performance_monitoring(self):
        """Test performance monitoring events"""
        print("\n🧪 Testing Performance Monitoring")

        try:
            client = self.create_encrypted_client()  # Send to target npub

            # Simulate various performance scenarios
            scenarios = [
                {
                    "operation": "api_endpoint",
                    "endpoint": "/api/v1/users/profile",
                    "method": "GET",
                    "duration": 2.45,
                    "status_code": 200,
                    "level": "warning"  # Slow but successful
                },
                {
                    "operation": "database_query",
                    "query": "SELECT COUNT(*) FROM orders WHERE created_at > ?",
                    "duration": 8.92,
                    "rows_affected": 156789,
                    "level": "error"  # Very slow
                },
                {
                    "operation": "cache_operation",
                    "cache_key": "user:profile:12345",
                    "operation_type": "miss",
                    "duration": 0.023,
                    "level": "info"  # Normal cache miss
                },
                {
                    "operation": "external_api",
                    "service": "payment_processor",
                    "endpoint": "https://api.stripe.com/charges",
                    "duration": 15.67,
                    "timeout": 30.0,
                    "level": "error"  # Slow external service
                }
            ]

            for scenario in scenarios:
                event = sentrystr.Event()
                event.with_message(f"Performance: {scenario['operation']} took {scenario['duration']}s")
                event.with_level(sentrystr.Level(scenario['level']))
                event.with_tag("monitoring", "performance")
                event.with_tag("operation_type", scenario['operation'])

                # Add all scenario data as extra
                for key, value in scenario.items():
                    if key != 'level':
                        event.add_extra(key, value)

                # Add threshold analysis
                if scenario['duration'] > 5.0:
                    event.add_extra("performance_issue", "slow_operation")
                    event.with_tag("alert", "performance_degradation")

                client.capture_event(event)
                self.log_test(f"Performance monitoring - {scenario['operation']}")
                time.sleep(0.5)

            return True
        except Exception as e:
            print(f"Error in performance monitoring test: {e}")
            self.log_test("Performance monitoring", False)
            return False

    def test_security_events(self):
        """Test security-related events"""
        print("\n🧪 Testing Security Events")

        try:
            client = self.create_encrypted_client()  # Security events should be encrypted

            security_events = [
                {
                    "event_type": "failed_login",
                    "message": "Multiple failed login attempts detected",
                    "level": "warning",
                    "details": {
                        "ip_address": "10.0.0.45",
                        "username": "admin",
                        "attempts": 5,
                        "time_window": "5 minutes"
                    }
                },
                {
                    "event_type": "privilege_escalation",
                    "message": "User attempted to access admin resources",
                    "level": "error",
                    "details": {
                        "user_id": "user_789",
                        "requested_resource": "/admin/users",
                        "user_role": "regular_user",
                        "ip_address": "192.168.1.50"
                    }
                },
                {
                    "event_type": "suspicious_activity",
                    "message": "Unusual data access pattern detected",
                    "level": "warning",
                    "details": {
                        "user_id": "user_456",
                        "data_accessed": "customer_database",
                        "records_accessed": 10000,
                        "normal_average": 50,
                        "time_of_day": "3:00 AM"
                    }
                }
            ]

            for sec_event in security_events:
                event = sentrystr.Event()
                event.with_message(sec_event['message'])
                event.with_level(sentrystr.Level(sec_event['level']))
                event.with_tag("category", "security")
                event.with_tag("event_type", sec_event['event_type'])
                event.with_tag("requires_investigation", "true")

                # Add security details
                for key, value in sec_event['details'].items():
                    event.add_extra(key, value)

                event.add_extra("detection_time", datetime.now().isoformat())
                event.add_extra("alert_priority", "high" if sec_event['level'] == "error" else "medium")

                client.capture_event(event)
                self.log_test(f"Security event - {sec_event['event_type']}")
                time.sleep(1.0)

            return True
        except Exception as e:
            print(f"Error in security events test: {e}")
            self.log_test("Security events", False)
            return False

    def test_business_metrics(self):
        """Test business metrics and KPI events"""
        print("\n🧪 Testing Business Metrics")

        try:
            client = self.create_public_client()  # Business metrics can be public

            # Simulate business events
            business_events = [
                {
                    "metric": "daily_revenue",
                    "value": 45678.90,
                    "currency": "USD",
                    "target": 50000.00,
                    "level": "warning"  # Below target
                },
                {
                    "metric": "user_signups",
                    "value": 234,
                    "period": "hourly",
                    "previous_hour": 189,
                    "level": "info"  # Good growth
                },
                {
                    "metric": "system_load",
                    "value": 0.85,
                    "threshold": 0.80,
                    "level": "warning"  # High load
                },
                {
                    "metric": "conversion_rate",
                    "value": 0.0234,  # 2.34%
                    "previous_rate": 0.0278,
                    "level": "error"  # Significant drop
                }
            ]

            for metric_event in business_events:
                event = sentrystr.Event()
                event.with_message(f"Business metric alert: {metric_event['metric']}")
                event.with_level(sentrystr.Level(metric_event['level']))
                event.with_tag("category", "business_intelligence")
                event.with_tag("metric_type", metric_event['metric'])
                event.with_tag("monitoring", "automated")

                # Add metric data
                for key, value in metric_event.items():
                    if key != 'level':
                        event.add_extra(key, value)

                event.add_extra("measurement_time", datetime.now().isoformat())
                event.add_extra("dashboard_link", f"https://dashboard.example.com/metrics/{metric_event['metric']}")

                client.capture_event(event)
                self.log_test(f"Business metric - {metric_event['metric']}")
                time.sleep(0.8)

            return True
        except Exception as e:
            print(f"Error in business metrics test: {e}")
            self.log_test("Business metrics", False)
            return False

    def run_comprehensive_tests(self):
        """Run all test suites"""
        print("🚀 Starting Comprehensive SentryStr Python Bindings Test Suite (Dynamic Keys)")
        print(f"🎯 Target npub: {TARGET_NPUB}")
        print(f"🆔 Generated sender pubkey: {SENDER_PUBLIC_KEY[:20]}...")
        print(f"🔑 Generated sender private key: {SENDER_PRIVATE_KEY[:20]}...")
        print(f"📡 Relays: {', '.join(RELAYS)}")
        print(f"⏰ Test started at: {datetime.now().isoformat()}")
        print("=" * 80)

        # Run all test suites
        test_suites = [
            ("Basic Functionality", self.test_basic_functionality),
            ("Error Levels", self.test_error_levels),
            ("User Context", self.test_user_context),
            ("Exception Handling", self.test_exception_handling),
            ("Performance Monitoring", self.test_performance_monitoring),
            ("Security Events", self.test_security_events),
            ("Business Metrics", self.test_business_metrics)
        ]

        start_time = time.time()

        for suite_name, test_method in test_suites:
            print(f"\n{'='*20} {suite_name} {'='*20}")
            try:
                test_method()
                time.sleep(2)  # Delay between test suites
            except Exception as e:
                print(f"❌ Test suite '{suite_name}' failed with exception: {e}")
                self.log_test(f"{suite_name} suite", False)

        end_time = time.time()
        duration = end_time - start_time

        # Print final results
        print("\n" + "="*80)
        print("📊 COMPREHENSIVE TEST RESULTS")
        print("="*80)
        print(f"✅ Passed: {self.passed_tests}")
        print(f"❌ Failed: {self.failed_tests}")
        print(f"📈 Success Rate: {(self.passed_tests/(self.passed_tests+self.failed_tests)*100):.1f}%")
        print(f"⏱️  Duration: {duration:.2f} seconds")
        print(f"📅 Completed: {datetime.now().isoformat()}")

        if self.failed_tests == 0:
            print("\n🎉 ALL TESTS PASSED! 🎉")
            print(f"📱 Check your Nostr client for events sent to {TARGET_NPUB}")
            print(f"🆔 Events sent from dynamically generated key: {SENDER_PUBLIC_KEY[:20]}...")
            print("🔐 Some events are encrypted (sent to target npub)")
            print("🌍 Some events are public (visible to all)")
            print("⏰ Events may take a moment to propagate across relays")
            print("🔄 Run this test again for different keys each time!")
        else:
            print(f"\n⚠️  {self.failed_tests} tests failed. Check output above for details.")

        return self.failed_tests == 0

if __name__ == "__main__":
    test_suite = SentryStrTestSuite()
    success = test_suite.run_comprehensive_tests()
    sys.exit(0 if success else 1)