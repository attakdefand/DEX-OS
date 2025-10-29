# Android Testing Framework for DEX-OS

## Unit Tests

Unit tests for DEX-OS on Android use the standard Android testing framework:

- JUnit 4 for unit tests
- Mockito for mocking dependencies
- Robolectric for Android framework mocking

## Instrumentation Tests

Instrumentation tests use:

- AndroidX Test libraries
- Espresso for UI testing
- UI Automator for cross-app testing

## Running Tests

To run unit tests:
```
./gradlew test
```

To run instrumentation tests:
```
./gradlew connectedAndroidTest
```

## Test Structure

```
src/
├── main/
│   └── java/
├── test/
│   └── java/
└── androidTest/
    └── java/
```

## Continuous Integration

Tests are automatically run on every commit through GitHub Actions with Firebase Test Lab integration.