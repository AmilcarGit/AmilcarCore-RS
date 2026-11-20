use std::fs;
use std::hint::black_box;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

fn main() {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after the Unix epoch")
        .as_nanos();
    let root =
        std::env::temp_dir().join(format!("amilcar-file-bench-{}-{nonce}", std::process::id()));
    fs::create_dir_all(&root).expect("benchmark directory should be created");

    let left = root.join("left.bin");
    let right = root.join("right.bin");
    let payload = vec![0x5a_u8; 16 * 1024 * 1024];
    fs::write(&left, &payload).expect("left sample should be written");
    fs::write(&right, &payload).expect("right sample should be written");

    let iterations = 5_u32;
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(amilcar_fs::sha256_file(black_box(&left)).expect("hash should succeed"));
    }
    report(
        "SHA-256, 16 MiB",
        payload.len() as u64 * u64::from(iterations),
        start.elapsed(),
    );

    let start = Instant::now();
    for _ in 0..iterations {
        black_box(
            amilcar_fs::compare_files(black_box(&left), black_box(&right))
                .expect("comparison should succeed"),
        );
    }
    report(
        "File comparison, 16 MiB",
        payload.len() as u64 * u64::from(iterations) * 2,
        start.elapsed(),
    );

    fs::remove_dir_all(root).expect("benchmark files should be removed");
}

fn report(name: &str, bytes: u64, elapsed: std::time::Duration) {
    let seconds = elapsed.as_secs_f64();
    let throughput = bytes as f64 / (1024.0 * 1024.0) / seconds.max(f64::MIN_POSITIVE);
    println!("{name}: {throughput:.2} MiB/s ({seconds:.3} s total)");
}
