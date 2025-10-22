pub struct Macro {
    pub import: TokenStream,
    pub actual_macro: TokenStream,
}

pub struct MacroConfiguration {
    pub skip_defaults: bool,
    pub macro_to_add: Vec<Macro>,
}

pub struct MacrosConfiguration {
    global: MacroConfiguration,
    by_type: Map<String, MacroConfiguration>,
    by_name: Map<String, MacroConfiguration>,
}
