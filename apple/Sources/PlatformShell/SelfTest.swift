// The stub app's one behavior (plan step P0.13.5): a self-test line that proves a built, signed binary launches.
public enum SelfTest {
    public static let version = "0.0.0"

    /// The line to print and the exit code for the arguments after the program name: --self-test alone prints the
    /// name and version and exits 0; anything else prints nothing and exits 2.
    public static func answer(to arguments: [String]) -> (line: String?, code: Int32) {
        arguments == ["--self-test"] ? ("Shellborne \(version)", 0) : (nil, 2)
    }
}
