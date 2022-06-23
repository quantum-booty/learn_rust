mod associated_types;
mod fully_qualified_syntax;
mod supertraits;
mod newtype_pattern;

fn main() {
    // specifying placeholder types in trait definitions with associated types
    associated_types::associated_types();
    // for calling methods with the same name
    fully_qualified_syntax::fully_qualified_syntax();
    // when a trait dependes on another trait
    supertraits::supertraits();
    // allows implement external traits on external types
    newtype_pattern::newtype_pattern();
}
