use enum_variant_enumerator_derive::VariantEnumerator;

#[derive(VariantEnumerator, Debug, Clone, Copy, PartialEq, Eq)]
enum GrammarElements {
    Expr,
    Factor,
    Term,
}

fn main() -> Result<(), String> {
    for variant in GrammarElements::enumerate_variants() {
        println!("{:?}", variant);
    }

    Ok(())
}
