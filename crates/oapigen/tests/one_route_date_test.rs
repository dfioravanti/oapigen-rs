use rstest::rstest;

#[rstest]
#[case("chrono", oapigen::DateTimeLibraries::Chrono)]
#[case("jiff", oapigen::DateTimeLibraries::Jiff)]
fn one_route_date_test(#[case] name: &str, #[case] library: oapigen::DateTimeLibraries) {
    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_suffix(name);

    let mut config = oapigen::Config::default();
    config.libraries.datetime = library;

    let f = std::fs::File::open("fixtures/one_route_date.yaml").unwrap();
    let spec: oas3::Spec = serde_yaml::from_reader(f).unwrap();

    let got = oapigen::spec_to_rust(&config, spec).unwrap();

    settings.bind(|| {
        insta::assert_snapshot!(got.to_string());
    });
}
