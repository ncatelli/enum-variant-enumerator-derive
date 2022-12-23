use enum_variant_enumerator_derive::VariantEnumerator;

#[derive(VariantEnumerator, Debug, Clone, Copy, PartialEq, Eq)]
enum GrammarElements {
    Expr,
    Factor,
    Term,
}

fn main() {
    assert_eq!(
        GrammarElements::enumerate_variants().collect::<Vec<_>>(),
        vec![
            GrammarElements::Expr,
            GrammarElements::Factor,
            GrammarElements::Term,
        ]
    )
}
