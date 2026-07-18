use field_basis::{FieldRecipe, ONE, Term, cache_key, philox4x32_10, recipe_fingerprint, sample};

fn hex(bytes: impl AsRef<[u8]>) -> String {
    bytes
        .as_ref()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn main() {
    let recipe = FieldRecipe::new(
        vec![
            Term::ValueLattice2 {
                frequency: 2,
                amplitude: ONE,
                component: 7,
            },
            Term::Ridged { input: 0 },
        ],
        1,
    )
    .expect("fixed receipt recipe must be valid");
    let stream_key = [3_u8; 32];
    let reconstruction = [9_u8; 32];

    println!("{{");
    println!("  \"schema_version\": 1,");
    println!("  \"receipt_scope\": \"same-platform second-language\",");
    println!(
        "  \"philox_zero\": [{}, {}, {}, {}],",
        philox4x32_10([0; 4], [0; 2])[0],
        philox4x32_10([0; 4], [0; 2])[1],
        philox4x32_10([0; 4], [0; 2])[2],
        philox4x32_10([0; 4], [0; 2])[3]
    );
    println!(
        "  \"philox_mapped\": {:?},",
        philox4x32_10([1, 4, 7, 0], [0x03030303, 0x03030303])
    );
    println!(
        "  \"recipe_bytes_hex\": \"{}\",",
        hex(recipe.encode_canonical().unwrap())
    );
    println!(
        "  \"recipe_fingerprint_hex\": \"{}\",",
        hex(recipe_fingerprint(&recipe).unwrap())
    );
    println!(
        "  \"cache_key_hex\": \"{}\",",
        hex(cache_key(&recipe, reconstruction, b"second-language-receipt").unwrap())
    );
    println!("  \"samples\": [");
    for (index, (x, y)) in [
        (0_i64, 0_i64),
        (1_i64 << 32, 0),
        (123_456_789, -987_654_321),
        (-(1_i64 << 32), (1_i64 << 31) + 17),
    ]
    .into_iter()
    .enumerate()
    {
        let comma = if index == 3 { "" } else { "," };
        println!(
            "    {{\"x_q32_32\": {x}, \"y_q32_32\": {y}, \"value_q16_48\": {}}}{comma}",
            sample(&recipe, stream_key, x, y).unwrap()
        );
    }
    println!("  ],");
    println!(
        "  \"limitations\": [\"same Windows host\", \"not a second-platform receipt\", \"not reference_proven\"]"
    );
    println!("}}");
}
