use arwa::dom::Name;

pub trait Attributes: Clone + Default {
    const OBSERVED: &'static [Name];

    fn update(&mut self, name: &Name, value: Option<String>);
}

impl Attributes for () {
    const OBSERVED: &'static [Name] = &[];

    fn update(&mut self, _name: &Name, _value: Option<String>) {
        ()
    }
}

pub trait Attribute {
    fn update(&mut self, value: Option<String>);
}

impl Attribute for Option<String> {
    fn update(&mut self, value: Option<String>) {
        *self = value;
    }
}
