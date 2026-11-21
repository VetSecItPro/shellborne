// swift-tools-version: 6.2
// The Shellborne Swift package: five one-way layers, checked by tools/gates/boundaries.
// Settings: Swift 6 language mode and all warnings as errors for every target, strict memory safety for
// EngineSurface, and main-actor default isolation for the four UI targets. tools/gates/unsafe-code checks them
// through swift package dump-package (plan step P0.10.3).
import PackageDescription

let warningsAsErrors: [SwiftSetting] = [.treatAllWarnings(as: .error)]
let uiTargetSettings: [SwiftSetting] = warningsAsErrors + [.defaultIsolation(MainActor.self)]

let package = Package(
    name: "Shellborne",
    platforms: [.macOS("26.0")],
    targets: [
        .target(name: "EngineSurface", dependencies: [], swiftSettings: warningsAsErrors + [.strictMemorySafety()]),
        .target(name: "TerminalView", dependencies: ["EngineSurface"], swiftSettings: uiTargetSettings),
        .target(name: "CoreClient", dependencies: [], swiftSettings: warningsAsErrors),
        .target(name: "DesignSystem", dependencies: [], swiftSettings: uiTargetSettings),
        .target(name: "FeaturePlaceholder", dependencies: ["CoreClient", "DesignSystem"], swiftSettings: uiTargetSettings),
        .target(name: "PlatformShell", dependencies: ["TerminalView", "CoreClient", "DesignSystem", "FeaturePlaceholder"],
                swiftSettings: uiTargetSettings),
        .executableTarget(name: "Shellborne", dependencies: ["PlatformShell"], swiftSettings: uiTargetSettings),
        .testTarget(name: "ShellborneTests", dependencies: ["PlatformShell"], swiftSettings: uiTargetSettings),
    ],
    swiftLanguageModes: [.v6]
)
