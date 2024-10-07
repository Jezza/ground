use super::VisitorOutcome;
use openapiv3::{
    AdditionalProperties, AnySchema, ArrayType, BooleanType, Discriminator, ExternalDocumentation,
    IntegerFormat, IntegerType, NumberFormat, NumberType, ObjectType, ReferenceOr,
    ReferenceOr::Item, Schema, SchemaData, SchemaKind, StringFormat, StringType, Type,
    VariantOrUnknownOrEmpty,
};

pub type Ext = indexmap::IndexMap<String, serde_json::Value>;

pub trait TypeVisitMut<'openapi> {
    type Outcome: VisitorOutcome;

    fn visit_reference_or_schema_mut(
        &mut self,
        node: &'openapi mut ReferenceOr<Schema>,
    ) -> Self::Outcome {
        visit_reference_or_schema_mut(self, node)
    }
    fn visit_schema_mut(&mut self, node: &'openapi mut Schema) -> Self::Outcome {
        visit_schema_mut(self, node)
    }
    fn visit_schema_data_mut(&mut self, node: &'openapi mut SchemaData) -> Self::Outcome {
        visit_schema_data_mut(self, node)
    }
    fn visit_discriminator_mut(&mut self, node: &'openapi mut Discriminator) -> Self::Outcome {
        visit_discriminator_mut(self, node)
    }
    fn visit_schema_kind_mut(&mut self, node: &'openapi mut SchemaKind) -> Self::Outcome {
        visit_schema_kind_mut(self, node)
    }
    fn visit_any_schema_mut(&mut self, node: &'openapi mut AnySchema) -> Self::Outcome {
        visit_any_schema_mut(self, node)
    }
    fn visit_type_mut(&mut self, node: &'openapi mut Type) -> Self::Outcome {
        visit_type_mut(self, node)
    }
    fn visit_boolean_type_mut(&mut self, node: &'openapi mut BooleanType) -> Self::Outcome {
        visit_boolean_type_mut(self, node)
    }
    fn visit_array_type_mut(&mut self, node: &'openapi mut ArrayType) -> Self::Outcome {
        visit_array_type_mut(self, node)
    }
    fn visit_object_type_mut(&mut self, node: &'openapi mut ObjectType) -> Self::Outcome {
        visit_object_type_mut(self, node)
    }
    fn visit_reference_or_box_schema_mut(
        &mut self,
        node: &'openapi mut ReferenceOr<Box<Schema>>,
    ) -> Self::Outcome {
        visit_reference_or_box_schema_mut(self, node)
    }
    fn visit_additional_properties_mut(
        &mut self,
        node: &'openapi mut AdditionalProperties,
    ) -> Self::Outcome {
        visit_additional_properties_mut(self, node)
    }
    fn visit_integer_type_mut(&mut self, node: &'openapi mut IntegerType) -> Self::Outcome {
        visit_integer_type_mut(self, node)
    }
    fn visit_variant_or_unknown_or_empty_integer_format_mut(
        &mut self,
        node: &'openapi mut VariantOrUnknownOrEmpty<IntegerFormat>,
    ) -> Self::Outcome {
        visit_variant_or_unknown_or_empty_integer_format_mut(self, node)
    }
    fn visit_integer_format_mut(&mut self, node: &'openapi mut IntegerFormat) -> Self::Outcome {
        visit_integer_format_mut(self, node)
    }
    fn visit_number_type_mut(&mut self, node: &'openapi mut NumberType) -> Self::Outcome {
        visit_number_type_mut(self, node)
    }
    fn visit_variant_or_unknown_or_empty_number_format_mut(
        &mut self,
        node: &'openapi mut VariantOrUnknownOrEmpty<NumberFormat>,
    ) -> Self::Outcome {
        visit_variant_or_unknown_or_empty_number_format_mut(self, node)
    }
    fn visit_number_format_mut(&mut self, node: &'openapi mut NumberFormat) -> Self::Outcome {
        visit_number_format_mut(self, node)
    }
    fn visit_string_type_mut(&mut self, node: &'openapi mut StringType) -> Self::Outcome {
        visit_string_type_mut(self, node)
    }
    fn visit_variant_or_unknown_or_empty_string_format_mut(
        &mut self,
        node: &'openapi mut VariantOrUnknownOrEmpty<StringFormat>,
    ) -> Self::Outcome {
        visit_variant_or_unknown_or_empty_string_format_mut(self, node)
    }
    fn visit_string_format_mut(&mut self, node: &'openapi mut StringFormat) -> Self::Outcome {
        visit_string_format_mut(self, node)
    }
    fn visit_external_documentation_mut(
        &mut self,
        node: &'openapi mut ExternalDocumentation,
    ) -> Self::Outcome {
        visit_external_documentation_mut(self, node)
    }
    fn visit_extensions_mut(&mut self, node: &'openapi mut Ext) -> Self::Outcome {
        visit_extensions_mut(self, node)
    }
}

pub fn visit_reference_or_schema_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut ReferenceOr<Schema>,
) -> V::Outcome
where
    V: TypeVisitMut<'openapi> + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_schema_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_schema_mut<'openapi, V>(visitor: &mut V, node: &'openapi mut Schema) -> V::Outcome
where
    V: TypeVisitMut<'openapi> + ?Sized,
{
    let Schema {
        schema_data,
        schema_kind,
    } = node;
    let outcome = visitor.visit_schema_data_mut(schema_data);
    let other = visitor.visit_schema_kind_mut(schema_kind);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_schema_data_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut SchemaData,
) -> V::Outcome
where
    V: TypeVisitMut<'openapi> + ?Sized,
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
    if let Some(node) = external_docs.as_mut() {
        let other = visitor.visit_external_documentation_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = discriminator.as_mut() {
        let other = visitor.visit_discriminator_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_mut(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_discriminator_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut Discriminator,
) -> V::Outcome
where
    V: TypeVisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_schema_kind_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut SchemaKind,
) -> V::Outcome
where
    V: TypeVisitMut<'openapi> + ?Sized,
{
    match node {
        SchemaKind::Type(node) => visitor.visit_type_mut(node),
        SchemaKind::OneOf { one_of } => {
            let mut outcome = V::Outcome::new();
            for node in one_of {
                let other = visitor.visit_reference_or_schema_mut(node);
                outcome = V::Outcome::reduce(outcome, other);
            }
            outcome
        }
        SchemaKind::AllOf { all_of } => {
            let mut outcome = V::Outcome::new();
            for node in all_of {
                let other = visitor.visit_reference_or_schema_mut(node);
                outcome = V::Outcome::reduce(outcome, other);
            }
            outcome
        }
        SchemaKind::AnyOf { any_of } => {
            let mut outcome = V::Outcome::new();
            for node in any_of {
                let other = visitor.visit_reference_or_schema_mut(node);
                outcome = V::Outcome::reduce(outcome, other);
            }
            outcome
        }
        SchemaKind::Not { not } => {
            visitor.visit_reference_or_schema_mut(<_ as AsMut<_>>::as_mut(not))
        }
        SchemaKind::Any(node) => visitor.visit_any_schema_mut(node),
    }
}

pub fn visit_any_schema_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut AnySchema,
) -> V::Outcome
where
    V: TypeVisitMut<'openapi> + ?Sized,
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
        let other = visitor.visit_reference_or_box_schema_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = additional_properties.as_mut() {
        let other = visitor.visit_additional_properties_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = items.as_mut() {
        let other = visitor.visit_reference_or_box_schema_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for node in one_of {
        let other = visitor.visit_reference_or_schema_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for node in all_of {
        let other = visitor.visit_reference_or_schema_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for node in any_of {
        let other = visitor.visit_reference_or_schema_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = not.as_mut() {
        let other = visitor.visit_reference_or_schema_mut(<_ as AsMut<_>>::as_mut(node));
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_type_mut<'openapi, V>(visitor: &mut V, node: &'openapi mut Type) -> V::Outcome
where
    V: TypeVisitMut<'openapi> + ?Sized,
{
    match node {
        Type::String(node) => visitor.visit_string_type_mut(node),
        Type::Number(node) => visitor.visit_number_type_mut(node),
        Type::Integer(node) => visitor.visit_integer_type_mut(node),
        Type::Object(node) => visitor.visit_object_type_mut(node),
        Type::Array(node) => visitor.visit_array_type_mut(node),
        Type::Boolean(node) => visitor.visit_boolean_type_mut(node),
    }
}

pub fn visit_boolean_type_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut BooleanType,
) -> V::Outcome
where
    V: TypeVisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_array_type_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut ArrayType,
) -> V::Outcome
where
    V: TypeVisitMut<'openapi> + ?Sized,
{
    let ArrayType {
        items,
        min_items: _,
        max_items: _,
        unique_items: _,
    } = node;
    let mut outcome = V::Outcome::new();
    if let Some(node) = items.as_mut() {
        let other = visitor.visit_reference_or_box_schema_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_object_type_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut ObjectType,
) -> V::Outcome
where
    V: TypeVisitMut<'openapi> + ?Sized,
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
        let other = visitor.visit_reference_or_box_schema_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = additional_properties.as_mut() {
        let other = visitor.visit_additional_properties_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_reference_or_box_schema_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut ReferenceOr<Box<Schema>>,
) -> V::Outcome
where
    V: TypeVisitMut<'openapi> + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_schema_mut(<_ as AsMut<_>>::as_mut(node));
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_additional_properties_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut AdditionalProperties,
) -> V::Outcome
where
    V: TypeVisitMut<'openapi> + ?Sized,
{
    match node {
        AdditionalProperties::Any(_) => V::Outcome::new(),
        AdditionalProperties::Schema(node) => {
            visitor.visit_reference_or_schema_mut(<_ as AsMut<_>>::as_mut(node))
        }
    }
}

pub fn visit_integer_type_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut IntegerType,
) -> V::Outcome
where
    V: TypeVisitMut<'openapi> + ?Sized,
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
    visitor.visit_variant_or_unknown_or_empty_integer_format_mut(format)
}

pub fn visit_variant_or_unknown_or_empty_integer_format_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut VariantOrUnknownOrEmpty<IntegerFormat>,
) -> V::Outcome
where
    V: TypeVisitMut<'openapi> + ?Sized,
{
    match node {
        VariantOrUnknownOrEmpty::Item(node) => visitor.visit_integer_format_mut(node),
        VariantOrUnknownOrEmpty::Unknown(_) => V::Outcome::new(),
        VariantOrUnknownOrEmpty::Empty => V::Outcome::new(),
    }
}

pub fn visit_integer_format_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut IntegerFormat,
) -> V::Outcome
where
    V: TypeVisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_number_type_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut NumberType,
) -> V::Outcome
where
    V: TypeVisitMut<'openapi> + ?Sized,
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
    visitor.visit_variant_or_unknown_or_empty_number_format_mut(format)
}

pub fn visit_variant_or_unknown_or_empty_number_format_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut VariantOrUnknownOrEmpty<NumberFormat>,
) -> V::Outcome
where
    V: TypeVisitMut<'openapi> + ?Sized,
{
    match node {
        VariantOrUnknownOrEmpty::Item(node) => visitor.visit_number_format_mut(node),
        VariantOrUnknownOrEmpty::Unknown(_) => V::Outcome::new(),
        VariantOrUnknownOrEmpty::Empty => V::Outcome::new(),
    }
}

pub fn visit_number_format_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut NumberFormat,
) -> V::Outcome
where
    V: TypeVisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_string_type_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut StringType,
) -> V::Outcome
where
    V: TypeVisitMut<'openapi> + ?Sized,
{
    let StringType {
        format,
        pattern: _,
        enumeration: _,
        min_length: _,
        max_length: _,
    } = node;
    visitor.visit_variant_or_unknown_or_empty_string_format_mut(format)
}

pub fn visit_variant_or_unknown_or_empty_string_format_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut VariantOrUnknownOrEmpty<StringFormat>,
) -> V::Outcome
where
    V: TypeVisitMut<'openapi> + ?Sized,
{
    match node {
        VariantOrUnknownOrEmpty::Item(node) => visitor.visit_string_format_mut(node),
        VariantOrUnknownOrEmpty::Unknown(_) => V::Outcome::new(),
        VariantOrUnknownOrEmpty::Empty => V::Outcome::new(),
    }
}

pub fn visit_string_format_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut StringFormat,
) -> V::Outcome
where
    V: TypeVisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_external_documentation_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut ExternalDocumentation,
) -> V::Outcome
where
    V: TypeVisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_extensions_mut<'openapi, V>(visitor: &mut V, node: &'openapi mut Ext) -> V::Outcome
where
    V: TypeVisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}
