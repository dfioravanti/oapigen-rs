use insta;
use oapigen;

#[test]
fn one_route_int_test() {
    let config = oapigen::Config::default();
    let f = std::fs::File::open("fixtures/one_route_int.yaml").unwrap();
    let spec: oas3::Spec = serde_yaml::from_reader(f).unwrap();

    let got = oapigen::spec_to_rust(&config, spec).unwrap();

    insta::assert_snapshot!(got.to_string());
}
