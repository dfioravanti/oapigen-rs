use indoc::indoc;
use rstest::fixture;

#[fixture]
fn twenty_one() -> &'static str {
    indoc! {r#"
        def hello():
            print("Hello, world!")

        hello()
    "#}
}
