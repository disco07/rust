//@compile-flags: -Zmir-opt-level=3 -Zinline-mir-hint-threshold=1000
//@normalize-stderr-test: "unsafe \{ libc::abort\(\) \}|crate::intrinsics::abort\(\);" -> "ABORT();"
//@normalize-stderr-test: "\| +\^+" -> "| ^"
//@normalize-stderr-test: "\n  +[0-9]+:[^\n]+" -> "$1"
//@normalize-stderr-test: "\n at [^\n]+" -> "$1"
//@error-in-other-file: aborted execution
// Enable MIR inlining to ensure that `TerminatorKind::UnwindTerminate` is generated
// instead of just `UnwindAction::Terminate`.

#![feature(c_unwind)]

struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {}
}

#[inline(always)]
fn has_cleanup() {
    let _f = Foo;
    panic!();
}

extern "C" fn panic_abort() {
    has_cleanup();
}

fn main() {
    panic_abort();
}
