// The stub app (plan step P0.13.5): it proves the bundle builds, signs and launches. It opens no window yet.
import Foundation
import PlatformShell

let answer = SelfTest.answer(to: Array(CommandLine.arguments.dropFirst()))
if let line = answer.line {
    print(line)
}
exit(answer.code)
