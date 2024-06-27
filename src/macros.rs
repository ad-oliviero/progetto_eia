#[macro_export]
macro_rules! timed_run {
    ($code:block) => {{
        let start = std::time::Instant::now();
        $code;
        let elapsed = start.elapsed();
        elapsed
    }};
}
