use indexmap::IndexMap;
use openapiv3::{Components, Operation, PathItem, ReferenceOr};

pub mod visitor;

pub type Obj<T> = IndexMap<String, ReferenceOr<T>>;
pub type Ext = IndexMap<String, serde_json::Value>;

pub fn check_ext(extensions: &Ext, key: &str) -> bool {
    matches!(
        extensions.get(key),
        Some(serde_json::Value::Bool(value)) if *value
    )
}

pub fn ext_str<'a>(extensions: &'a Ext, key: &str) -> Option<&'a str> {
    extensions.get(key)?.as_str()
}

pub fn visit_op_mut(item: &mut PathItem, mut func: impl FnMut(&'static str, &mut Operation)) {
    macro_rules! visit {
        (
            $($ident:ident),*$(,)?
        ) => {
            $(
                if let Some(value) = item.$ident.as_mut() {
                    func(stringify!($ident), value);
                }
            )*
        };
    }

    visit! {
        get,
        put,
        post,
        delete,
        options,
        head,
        patch,
        trace,
    }
}

//  = "#/components/requestBodies/"
// fn resolve_ref<'a>(components: &'a Components, reference: &str) -> &'a ReferenceOr<Self>;

pub trait ReferenceType: Sized {
    const REF: &'static str;

    fn components(components: &Components) -> &IndexMap<String, ReferenceOr<Self>>;
}

pub fn resolve_ref<'a, T: ReferenceType + 'a>(
    components: &'a Components,
    reference: &'a ReferenceOr<T>,
) -> &'a T {
    let reference = match reference {
        ReferenceOr::Item(item) => {
            return item;
        }
        ReferenceOr::Reference { reference } => &**reference,
    };

    let reference = reference
        .strip_prefix(T::REF)
        .expect("Reference type doesn't match the path contained.");

    let comps = T::components(components);
    let reference = comps
        .get(reference)
        .expect(&format!("Key is missing: {}", reference));

    resolve_ref(components, reference)
}

impl ReferenceType for openapiv3::Parameter {
    const REF: &'static str = "#/components/parameters/";

    fn components(components: &Components) -> &IndexMap<String, ReferenceOr<Self>> {
        &components.parameters
    }
}
impl ReferenceType for openapiv3::RequestBody {
    const REF: &'static str = "#/components/requestBodies/";

    fn components(components: &Components) -> &IndexMap<String, ReferenceOr<Self>> {
        &components.request_bodies
    }
}
impl ReferenceType for openapiv3::Response {
    const REF: &'static str = "#/components/responses/";

    fn components(components: &Components) -> &IndexMap<String, ReferenceOr<Self>> {
        &components.responses
    }
}
impl ReferenceType for openapiv3::Schema {
    const REF: &'static str = "#/components/ schemas/";

    fn components(components: &Components) -> &IndexMap<String, ReferenceOr<Self>> {
        &components.schemas
    }
}
