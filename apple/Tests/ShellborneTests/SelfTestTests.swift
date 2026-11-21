// Unit tests of the stub's self-test answer (plan step P0.13.5).
import PlatformShell
import Testing

@MainActor @Test func selfTestPrintsTheNameAndVersion() {
    let answer = SelfTest.answer(to: ["--self-test"])
    #expect(answer.line == "Shellborne \(SelfTest.version)")
    #expect(answer.code == 0)
}

@MainActor @Test func anyOtherArgumentsPrintNothingAndExitTwo() {
    for arguments in [[], ["--anything"], ["--self-test", "extra"]] {
        let answer = SelfTest.answer(to: arguments)
        #expect(answer.line == nil)
        #expect(answer.code == 2)
    }
}
