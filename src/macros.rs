#[macro_export]
macro_rules! timed_run {
    ($name:expr, $code:block) => {{
        let start = std::time::Instant::now();
        $code;
        let elapsed = start.elapsed();
        println!(
            "{} in: {}.{:03}s",
            $name,
            elapsed.as_secs(),
            elapsed.subsec_millis()
        );
    }};
}
#[macro_export]
macro_rules! either {
    ($test:expr => $true_expr:expr; $false_expr:expr) => {
        if $test {
            $true_expr
        } else {
            $false_expr
        }
    };
}
