işlev say(n: u64) -> u64 {
    say(n + 1)
}

işlev ana() {
    yazdır!("{}", say(0));
}
