#![no_main]

use libfuzzer_sys::fuzz_target;

// The whole native reader (archive, object stream, model) and the preview, on arbitrary bytes.
fuzz_target!(|bytes: &[u8]| {
    let _ = localvector_affinity::container::header(bytes);
    let _ = localvector_affinity::preview(bytes);
    let _ = localvector_affinity::read(bytes, localvector_affinity::Limits { max_entry: 16 << 20, max_total: 64 << 20 });
});
