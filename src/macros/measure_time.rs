#[macro_export]
macro_rules! measure_time {
    ($func:block, $day:expr, $part:expr) => {{
        let start = Instant::now();
        let result = $func;
        let duration = start.elapsed();

        println!(
            "Day {} - part {} executed in {:?}. Result: {:?}",
            $day, $part, duration, result
        );

        result
    }};
}
