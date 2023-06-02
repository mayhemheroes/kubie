use honggfuzz::fuzz;
use kubie::settings::expanduser;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            // Convert the input to a string
            if let Ok(path) = std::str::from_utf8(data) {
                // Call the expanduser function with the fuzzed input
                let _ = expanduser(path);
            }
        });
    }
}
