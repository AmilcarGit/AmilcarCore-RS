use std::hint::black_box;
use std::time::Instant;

fn main() {
    let text = "AmilcarCore-RS procesa datos localmente.\n".repeat(25_000);
    let json = format!(
        "{{\"items\":[{}]}}",
        (0..10_000)
            .map(|index| format!("{{\"id\":{index},\"active\":true}}"))
            .collect::<Vec<_>>()
            .join(",")
    );
    let csv = format!(
        "id,name\n{}",
        (0..10_000)
            .map(|index| format!("{index},item-{index}"))
            .collect::<Vec<_>>()
            .join("\n")
    );

    measure("Text statistics", 20, || {
        black_box(amilcar_data::text::stats(black_box(&text)));
    });
    measure("JSON formatting", 10, || {
        black_box(amilcar_data::json::format(black_box(&json)).expect("JSON should format"));
    });
    measure("CSV validation", 10, || {
        black_box(amilcar_data::csv::validate(black_box(&csv)).expect("CSV should validate"));
    });
}

fn measure(name: &str, iterations: u32, mut operation: impl FnMut()) {
    let start = Instant::now();
    for _ in 0..iterations {
        operation();
    }
    println!(
        "{name}: {iterations} iterations in {:.3} s",
        start.elapsed().as_secs_f64()
    );
}
