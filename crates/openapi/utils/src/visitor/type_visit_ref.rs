use super::VisitorOutcome;
use openapiv3::{
    AdditionalProperties, AnySchema, ArrayType, BooleanType, Discriminator, ExternalDocumentation,
    IntegerFormat, IntegerType, NumberFormat, NumberType, ObjectType, ReferenceOr,
    ReferenceOr::Item, Schema, SchemaData, SchemaKind, StringFormat, StringType, Type,
    VariantOrUnknownOrEmpty,
};

pub type Ext = indexmap::IndexMap<String, serde_json::Value>;

pub trait TypeVisitRef<'openapi> {
    type Outcome: VisitorOutcome;

    fn visit_reference_or_schema_ref(
        &mut self,
        node: &'openapi ReferenceOr<Schema>,
    ) -> Self::Outcome {
        visit_reference_or_schema_ref(self, node)
    }
    fn visit_schema_ref(&mut self, node: &'openapi Schema) -> Self::Outcome {
        visit_schema_ref(self, node)
    }
    fn visit_schema_data_ref(&mut self, node: &'openapi SchemaData) -> Self::Outcome {
        visit_schema_data_ref(self, node)
    }
    fn visit_discriminator_ref(&mut self, node: &'openapi Discriminator) -> Self::Outcome {
        visit_discriminator_ref(self, node)
    }
    fn visit_schema_kind_ref(&mut self, node: &'openapi SchemaKind) -> Self::Outcome {
        visit_schema_kind_ref(self, node)
    }
    fn visit_any_schema_ref(&mut self, node: &'openapi AnySchema) -> Self::Outcome {
        visit_any_schema_ref(self, node)
    }
    fn visit_type_ref(&mut self, node: &'openapi Type) -> Self::Outcome {
        visit_type_ref(self, node)
    }
    fn visit_boolean_type_ref(&mut self, node: &'openapi BooleanType) -> Self::Outcome {
        visit_boolean_type_ref(self, node)
    }
    fn visit_array_type_ref(&mut self, node: &'openapi ArrayType) -> Self::Outcome {
        visit_array_type_ref(self, node)
    }
    fn visit_object_type_ref(&mut self, node: &'openapi ObjectType) -> Self::Outcome {
        visit_object_type_ref(self, node)
    }
    fn visit_reference_or_box_schema_ref(
        &mut self,
        node: &'openapi ReferenceOr<Box<Schema>>,
    ) -> Self::Outcome {
        visit_reference_or_box_schema_ref(self, node)
    }
    fn visit_additional_properties_ref(
        &mut self,
        node: &'openapi AdditionalProperties,
    ) -> Self::Outcome {
        visit_additional_properties_ref(self, node)
    }
    fn visit_integer_type_ref(&mut self, node: &'openapi IntegerType) -> Self::Outcome {
        visit_integer_type_ref(self, node)
    }
    fn visit_variant_or_unknown_or_empty_integer_format_ref(
        &mut self,
        node: &'openapi VariantOrUnknownOrEmpty<IntegerFormat>,
    ) -> Self::Outcome {
        visit_variant_or_unknown_or_empty_integer_format_ref(self, node)
    }
    fn visit_integer_format_ref(&mut self, node: &'openapi IntegerFormat) -> Self::Outcome {
        visit_integer_format_ref(self, node)
    }
    fn visit_number_type_ref(&mut self, node: &'openapi NumberType) -> Self::Outcome {
        visit_number_type_ref(self, node)
    }
    fn visit_variant_or_unknown_or_empty_number_format_ref(
        &mut self,
        node: &'openapi VariantOrUnknownOrEmpty<NumberFormat>,
    ) -> Self::Outcome {
        visit_variant_or_unknown_or_empty_number_format_ref(self, node)
    }
    fn visit_number_format_ref(&mut self, node: &'openapi NumberFormat) -> Self::Outcome {
        visit_number_format_ref(self, node)
    }
    fn visit_string_type_ref(&mut self, node: &'openapi StringType) -> Self::Outcome {
        visit_string_type_ref(self, node)
    }
    fn visit_variant_or_unknown_or_empty_string_format_ref(
        &mut self,
        node: &'openapi VariantOrUnknownOrEmpty<StringFormat>,
    ) -> Self::Outcome {
        visit_variant_or_unknown_or_empty_string_format_ref(self, node)
    }
    fn visit_string_format_ref(&mut self, node: &'openapi StringFormat) -> Self::Outcome {
        visit_string_format_ref(self, node)
    }
    fn visit_external_documentation_ref(
        &mut self,
        node: &'openapi ExternalDocumentation,
    ) -> Self::Outcome {
        visit_external_documentation_ref(self, node)
    }
    fn visit_extensions_ref(&mut self, node: &'openapi Ext) -> Self::Outcome {
        visit_extensions_ref(self, node)
    }
}

pub fn visit_reference_or_schema_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi ReferenceOr<Schema>,
) -> V::Outcome
where
    V: TypeVisitRef<'openapi> + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_schema_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_schema_ref<'openapi, V>(visitor: &mut V, node: &'openapi Schema) -> V::Outcome
where
    V: TypeVisitRef<'openapi> + ?Sized,
{
    let Schema {
        schema_data,
        schema_kind,
    } = node;
    let outcome = visitor.visit_schema_data_ref(schema_data);
    let other = visitor.visit_schema_kind_ref(schema_kind);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_schema_data_ref<'openapi, V>(visitor: &mut V, node: &'openapi SchemaData) -> V::Outcome
where
    V: TypeVisitRef<'openapi> + ?Sized,
{
    let SchemaData {
        nullable: _,
        read_only: _,
        write_only: _,
        deprecated: _,
        external_docs,
        example: _,
        title: _,
        description: _,
        discriminator,
        default: _,
        extensions,
    } = node;
    let mut outcome = V::Outcome::new();
    if let Some(node) = external_docs.as_ref() {
        let other = visitor.visit_external_documentation_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = discriminator.as_ref() {
        let other = visitor.visit_discriminator_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_ref(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_discriminator_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi Discriminator,
) -> V::Outcome
where
    V: TypeVisitRef<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_schema_kind_ref<'openapi, V>(visitor: &mut V, node: &'openapi SchemaKind) -> V::Outcome
where
    V: TypeVisitRef<'openapi> + ?Sized,
{
    match node {
        SchemaKind::Type(node) => visitor.visit_type_ref(node),
        SchemaKind::OneOf { one_of } => {
            let mut outcome = V::Outcome::new();
            for node in one_of {
                let other = visitor.visit_reference_or_schema_ref(node);
                outcome = V::Outcome::reduce(outcome, other);
            }
            outcome
        }
        SchemaKind::AllOf { all_of } => {
            let mut outcome = V::Outcome::new();
            for node in all_of {
                let other = visitor.visit_reference_or_schema_ref(node);
                outcome = V::Outcome::reduce(outcome, other);
            }
            outcome
        }
        SchemaKind::AnyOf { any_of } => {
            let mut outcome = V::Outcome::new();
            for node in any_of {
                let other = visitor.visit_reference_or_schema_ref(node);
                outcome = V::Outcome::reduce(outcome, other);
            }
            outcome
        }
        SchemaKind::Not { not } => {
            visitor.visit_reference_or_schema_ref(<_ as AsRef<_>>::as_ref(not))
        }
        SchemaKind::Any(node) => visitor.visit_any_schema_ref(node),
    }
}

pub fn visit_any_schema_ref<'openapi, V>(visitor: &mut V, node: &'openapi AnySchema) -> V::Outcome
where
    V: TypeVisitRef<'openapi> + ?Sized,
{
    let AnySchema {
        typ: _,
        pattern: _,
        multiple_of: _,
        exclusive_minimum: _,
        exclusive_maximum: _,
        minimum: _,
        maximum: _,
        properties,
        required: _,
        additional_properties,
        min_properties: _,
        max_properties: _,
        items,
        min_items: _,
        max_items: _,
        unique_items: _,
        enumeration: _,
        format: _,
        min_length: _,
        max_length: _,
        one_of,
        all_of,
        any_of,
        not,
    } = node;
    let mut outcome = V::Outcome::new();
    for (_, node) in properties {
        let other = visitor.visit_reference_or_box_schema_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = additional_properties.as_ref() {
        let other = visitor.visit_additional_properties_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = items.as_ref() {
        let other = visitor.visit_reference_or_box_schema_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for node in one_of {
        let other = visitor.visit_reference_or_schema_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for node in all_of {
        let other = visitor.visit_reference_or_schema_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for node in any_of {
        let other = visitor.visit_reference_or_schema_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = not.as_ref() {
        let other = visitor.visit_reference_or_schema_ref(<_ as AsRef<_>>::as_ref(node));
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_type_ref<'openapi, V>(visitor: &mut V, node: &'openapi Type) -> V::Outcome
where
    V: TypeVisitRef<'openapi> + ?Sized,
{
    match node {
        Type::String(node) => visitor.visit_string_type_ref(node),
        Type::Number(node) => visitor.visit_number_type_ref(node),
        Type::Integer(node) => visitor.visit_integer_type_ref(node),
        Type::Object(node) => visitor.visit_object_type_ref(node),
        Type::Array(node) => visitor.visit_array_type_ref(node),
        Type::Boolean(node) => visitor.visit_boolean_type_ref(node),
    }
}

pub fn visit_boolean_type_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi BooleanType,
) -> V::Outcome
where
    V: TypeVisitRef<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_array_type_ref<'openapi, V>(visitor: &mut V, node: &'openapi ArrayType) -> V::Outcome
where
    V: TypeVisitRef<'openapi> + ?Sized,
{
    let ArrayType {
        items,
        min_items: _,
        max_items: _,
        unique_items: _,
    } = node;
    let mut outcome = V::Outcome::new();
    if let Some(node) = items.as_ref() {
        let other = visitor.visit_reference_or_box_schema_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_object_type_ref<'openapi, V>(visitor: &mut V, node: &'openapi ObjectType) -> V::Outcome
where
    V: TypeVisitRef<'openapi> + ?Sized,
{
    let ObjectType {
        properties,
        required: _,
        additional_properties,
        min_properties: _,
        max_properties: _,
    } = node;
    let mut outcome = V::Outcome::new();
    for (_, node) in properties {
        let other = visitor.visit_reference_or_box_schema_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = additional_properties.as_ref() {
        let other = visitor.visit_additional_properties_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_reference_or_box_schema_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi ReferenceOr<Box<Schema>>,
) -> V::Outcome
where
    V: TypeVisitRef<'openapi> + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_schema_ref(<_ as AsRef<_>>::as_ref(node));
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_additional_properties_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi AdditionalProperties,
) -> V::Outcome
where
    V: TypeVisitRef<'openapi> + ?Sized,
{
    match node {
        AdditionalProperties::Any(_) => V::Outcome::new(),
        AdditionalProperties::Schema(node) => {
            visitor.visit_reference_or_schema_ref(<_ as AsRef<_>>::as_ref(node))
        }
    }
}

pub fn visit_integer_type_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi IntegerType,
) -> V::Outcome
where
    V: TypeVisitRef<'openapi> + ?Sized,
{
    let IntegerType {
        format,
        multiple_of: _,
        exclusive_minimum: _,
        exclusive_maximum: _,
        minimum: _,
        maximum: _,
        enumeration: _,
    } = node;
    visitor.visit_variant_or_unknown_or_empty_integer_format_ref(format)
}

pub fn visit_variant_or_unknown_or_empty_integer_format_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi VariantOrUnknownOrEmpty<IntegerFormat>,
) -> V::Outcome
where
    V: TypeVisitRef<'openapi> + ?Sized,
{
    match node {
        VariantOrUnknownOrEmpty::Item(node) => visitor.visit_integer_format_ref(node),
        VariantOrUnknownOrEmpty::Unknown(_) => V::Outcome::new(),
        VariantOrUnknownOrEmpty::Empty => V::Outcome::new(),
    }
}

pub fn visit_integer_format_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi IntegerFormat,
) -> V::Outcome
where
    V: TypeVisitRef<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_number_type_ref<'openapi, V>(visitor: &mut V, node: &'openapi NumberType) -> V::Outcome
where
    V: TypeVisitRef<'openapi> + ?Sized,
{
    let NumberType {
        format,
        multiple_of: _,
        exclusive_minimum: _,
        exclusive_maximum: _,
        minimum: _,
        maximum: _,
        enumeration: _,
    } = node;
    visitor.visit_variant_or_unknown_or_empty_number_format_ref(format)
}

pub fn visit_variant_or_unknown_or_empty_number_format_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi VariantOrUnknownOrEmpty<NumberFormat>,
) -> V::Outcome
where
    V: TypeVisitRef<'openapi> + ?Sized,
{
    match node {
        VariantOrUnknownOrEmpty::Item(node) => visitor.visit_number_format_ref(node),
        VariantOrUnknownOrEmpty::Unknown(_) => V::Outcome::new(),
        VariantOrUnknownOrEmpty::Empty => V::Outcome::new(),
    }
}

pub fn visit_number_format_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi NumberFormat,
) -> V::Outcome
where
    V: TypeVisitRef<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_string_type_ref<'openapi, V>(visitor: &mut V, node: &'openapi StringType) -> V::Outcome
where
    V: TypeVisitRef<'openapi> + ?Sized,
{
    let StringType {
        format,
        pattern: _,
        enumeration: _,
        min_length: _,
        max_length: _,
    } = node;
    visitor.visit_variant_or_unknown_or_empty_string_format_ref(format)
}

pub fn visit_variant_or_unknown_or_empty_string_format_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi VariantOrUnknownOrEmpty<StringFormat>,
) -> V::Outcome
where
    V: TypeVisitRef<'openapi> + ?Sized,
{
    match node {
        VariantOrUnknownOrEmpty::Item(node) => visitor.visit_string_format_ref(node),
        VariantOrUnknownOrEmpty::Unknown(_) => V::Outcome::new(),
        VariantOrUnknownOrEmpty::Empty => V::Outcome::new(),
    }
}

pub fn visit_string_format_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi StringFormat,
) -> V::Outcome
where
    V: TypeVisitRef<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_external_documentation_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi ExternalDocumentation,
) -> V::Outcome
where
    V: TypeVisitRef<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_extensions_ref<'openapi, V>(visitor: &mut V, node: &'openapi Ext) -> V::Outcome
where
    V: TypeVisitRef<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}
