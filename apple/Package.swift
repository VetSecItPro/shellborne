// swift-tools-version: 6.0
// The Shellborne Swift package: five one-way layers, checked by tools/gates/boundaries.
import PackageDescription

let strict: [SwiftSetting] = [.unsafeFlags(["-warnings-as-errors"])]

let package = Package(
    name: "Shellborne",
    platforms: [.macOS("26.0")],
    targets: [
        .target(name: "EngineSurface", dependencies: [], swiftSettings: strict),
        .target(name: "TerminalView", dependencies: ["EngineSurface"], swiftSettings: strict),
        .target(name: "CoreClient", dependencies: [], swiftSettings: strict),
        .target(name: "DesignSystem", dependencies: [], swiftSettings: strict),
        .target(name: "FeaturePlaceholder", dependencies: ["CoreClient", "DesignSystem"], swiftSettings: strict),
        .target(name: "PlatformShell", dependencies: ["TerminalView", "CoreClient", "DesignSystem", "FeaturePlaceholder"], swiftSettings: strict),
    ],
    swiftLanguageModes: [.v6]
)
