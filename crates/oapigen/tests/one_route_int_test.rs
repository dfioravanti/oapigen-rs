use insta;
use oapigen;
use oapigen::generating::merges::merge_schemas;
use oapigen::parsing::specs::parse_specs;

#[test]
fn one_route_int_test() {
    let f = std::fs::File::open("fixtures/one_route_int.yaml").unwrap();
    let spec: oas3::Spec = serde_yaml::from_reader(f).unwrap();

    let parsed_schemas = parse_specs(spec).unwrap();
    let got = merge_schemas(parsed_schemas);

    insta::assert_yaml_snapshot!(got.to_string());
}
