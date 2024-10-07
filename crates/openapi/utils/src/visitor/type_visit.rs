use super::VisitorOutcome;
use openapiv3::{
    AdditionalProperties, AnySchema, ArrayType, BooleanType, Discriminator, ExternalDocumentation,
    IntegerFormat, IntegerType, NumberFormat, NumberType, ObjectType, ReferenceOr,
    ReferenceOr::Item, Schema, SchemaData, SchemaKind, StringFormat, StringType, Type,
    VariantOrUnknownOrEmpty,
};

pub type Ext = indexmap::IndexMap<String, serde_json::Value>;

pub trait TypeVisit {
    type Outcome: VisitorOutcome;

    fn visit_reference_or_schema(&mut self, node: ReferenceOr<Schema>) -> Self::Outcome {
        visit_reference_or_schema(self, node)
    }
    fn visit_schema(&mut self, node: Schema) -> Self::Outcome {
        visit_schema(self, node)
    }
    fn visit_schema_data(&mut self, node: SchemaData) -> Self::Outcome {
        visit_schema_data(self, node)
    }
    fn visit_discriminator(&mut self, node: Discriminator) -> Self::Outcome {
        visit_discriminator(self, node)
    }
    fn visit_schema_kind(&mut self, node: SchemaKind) -> Self::Outcome {
        visit_schema_kind(self, node)
    }
    fn visit_any_schema(&mut self, node: AnySchema) -> Self::Outcome {
        visit_any_schema(self, node)
    }
    fn visit_type(&mut self, node: Type) -> Self::Outcome {
        visit_type(self, node)
    }
    fn visit_boolean_type(&mut self, node: BooleanType) -> Self::Outcome {
        visit_boolean_type(self, node)
    }
    fn visit_array_type(&mut self, node: ArrayType) -> Self::Outcome {
        visit_array_type(self, node)
    }
    fn visit_object_type(&mut self, node: ObjectType) -> Self::Outcome {
        visit_object_type(self, node)
    }
    fn visit_reference_or_box_schema(&mut self, node: ReferenceOr<Box<Schema>>) -> Self::Outcome {
        visit_reference_or_box_schema(self, node)
    }
    fn visit_additional_properties(&mut self, node: AdditionalProperties) -> Self::Outcome {
        visit_additional_properties(self, node)
    }
    fn visit_integer_type(&mut self, node: IntegerType) -> Self::Outcome {
        visit_integer_type(self, node)
    }
    fn visit_variant_or_unknown_or_empty_integer_format(
        &mut self,
        node: VariantOrUnknownOrEmpty<IntegerFormat>,
    ) -> Self::Outcome {
        visit_variant_or_unknown_or_empty_integer_format(self, node)
    }
    fn visit_integer_format(&mut self, node: IntegerFormat) -> Self::Outcome {
        visit_integer_format(self, node)
    }
    fn visit_number_type(&mut self, node: NumberType) -> Self::Outcome {
        visit_number_type(self, node)
    }
    fn visit_variant_or_unknown_or_empty_number_format(
        &mut self,
        node: VariantOrUnknownOrEmpty<NumberFormat>,
    ) -> Self::Outcome {
        visit_variant_or_unknown_or_empty_number_format(self, node)
    }
    fn visit_number_format(&mut self, node: NumberFormat) -> Self::Outcome {
        visit_number_format(self, node)
    }
    fn visit_string_type(&mut self, node: StringType) -> Self::Outcome {
        visit_string_type(self, node)
    }
    fn visit_variant_or_unknown_or_empty_string_format(
        &mut self,
        node: VariantOrUnknownOrEmpty<StringFormat>,
    ) -> Self::Outcome {
        visit_variant_or_unknown_or_empty_string_format(self, node)
    }
    fn visit_string_format(&mut self, node: StringFormat) -> Self::Outcome {
        visit_string_format(self, node)
    }
    fn visit_external_documentation(&mut self, node: ExternalDocumentation) -> Self::Outcome {
        visit_external_documentation(self, node)
    }
    fn visit_extensions(&mut self, node: Ext) -> Self::Outcome {
        visit_extensions(self, node)
    }
}

pub fn visit_reference_or_schema<V>(visitor: &mut V, node: ReferenceOr<Schema>) -> V::Outcome
where
    V: TypeVisit + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_schema(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_schema<V>(visitor: &mut V, node: Schema) -> V::Outcome
where
    V: TypeVisit + ?Sized,
{
    let Schema {
        schema_data,
        schema_kind,
    } = node;
    let outcome = visitor.visit_schema_data(schema_data);
    let other = visitor.visit_schema_kind(schema_kind);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_schema_data<V>(visitor: &mut V, node: SchemaData) -> V::Outcome
where
    V: TypeVisit + ?Sized,
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
    if let Some(node) = external_docs {
        let other = visitor.visit_external_documentation(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = discriminator {
        let other = visitor.visit_discriminator(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_discriminator<V>(visitor: &mut V, node: Discriminator) -> V::Outcome
where
    V: TypeVisit + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_schema_kind<V>(visitor: &mut V, node: SchemaKind) -> V::Outcome
where
    V: TypeVisit + ?Sized,
{
    match node {
        SchemaKind::Type(node) => visitor.visit_type(node),
        SchemaKind::OneOf { one_of } => {
            let mut outcome = V::Outcome::new();
            for node in one_of {
                let other = visitor.visit_reference_or_schema(node);
                outcome = V::Outcome::reduce(outcome, other);
            }
            outcome
        }
        SchemaKind::AllOf { all_of } => {
            let mut outcome = V::Outcome::new();
            for node in all_of {
                let other = visitor.visit_reference_or_schema(node);
                outcome = V::Outcome::reduce(outcome, other);
            }
            outcome
        }
        SchemaKind::AnyOf { any_of } => {
            let mut outcome = V::Outcome::new();
            for node in any_of {
                let other = visitor.visit_reference_or_schema(node);
                outcome = V::Outcome::reduce(outcome, other);
            }
            outcome
        }
        SchemaKind::Not { not } => visitor.visit_reference_or_schema(*(not)),
        SchemaKind::Any(node) => visitor.visit_any_schema(node),
    }
}

pub fn visit_any_schema<V>(visitor: &mut V, node: AnySchema) -> V::Outcome
where
    V: TypeVisit + ?Sized,
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
        let other = visitor.visit_reference_or_box_schema(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = additional_properties {
        let other = visitor.visit_additional_properties(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = items {
        let other = visitor.visit_reference_or_box_schema(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for node in one_of {
        let other = visitor.visit_reference_or_schema(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for node in all_of {
        let other = visitor.visit_reference_or_schema(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for node in any_of {
        let other = visitor.visit_reference_or_schema(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = not {
        let other = visitor.visit_reference_or_schema(*(node));
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_type<V>(visitor: &mut V, node: Type) -> V::Outcome
where
    V: TypeVisit + ?Sized,
{
    match node {
        Type::String(node) => visitor.visit_string_type(node),
        Type::Number(node) => visitor.visit_number_type(node),
        Type::Integer(node) => visitor.visit_integer_type(node),
        Type::Object(node) => visitor.visit_object_type(node),
        Type::Array(node) => visitor.visit_array_type(node),
        Type::Boolean(node) => visitor.visit_boolean_type(node),
    }
}

pub fn visit_boolean_type<V>(visitor: &mut V, node: BooleanType) -> V::Outcome
where
    V: TypeVisit + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_array_type<V>(visitor: &mut V, node: ArrayType) -> V::Outcome
where
    V: TypeVisit + ?Sized,
{
    let ArrayType {
        items,
        min_items: _,
        max_items: _,
        unique_items: _,
    } = node;
    let mut outcome = V::Outcome::new();
    if let Some(node) = items {
        let other = visitor.visit_reference_or_box_schema(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_object_type<V>(visitor: &mut V, node: ObjectType) -> V::Outcome
where
    V: TypeVisit + ?Sized,
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
        let other = visitor.visit_reference_or_box_schema(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = additional_properties {
        let other = visitor.visit_additional_properties(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_reference_or_box_schema<V>(
    visitor: &mut V,
    node: ReferenceOr<Box<Schema>>,
) -> V::Outcome
where
    V: TypeVisit + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_schema(*(node));
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_additional_properties<V>(visitor: &mut V, node: AdditionalProperties) -> V::Outcome
where
    V: TypeVisit + ?Sized,
{
    match node {
        AdditionalProperties::Any(_) => V::Outcome::new(),
        AdditionalProperties::Schema(node) => visitor.visit_reference_or_schema(*(node)),
    }
}

pub fn visit_integer_type<V>(visitor: &mut V, node: IntegerType) -> V::Outcome
where
    V: TypeVisit + ?Sized,
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
    visitor.visit_variant_or_unknown_or_empty_integer_format(format)
}

pub fn visit_variant_or_unknown_or_empty_integer_format<V>(
    visitor: &mut V,
    node: VariantOrUnknownOrEmpty<IntegerFormat>,
) -> V::Outcome
where
    V: TypeVisit + ?Sized,
{
    match node {
        VariantOrUnknownOrEmpty::Item(node) => visitor.visit_integer_format(node),
        VariantOrUnknownOrEmpty::Unknown(_) => V::Outcome::new(),
        VariantOrUnknownOrEmpty::Empty => V::Outcome::new(),
    }
}

pub fn visit_integer_format<V>(visitor: &mut V, node: IntegerFormat) -> V::Outcome
where
    V: TypeVisit + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_number_type<V>(visitor: &mut V, node: NumberType) -> V::Outcome
where
    V: TypeVisit + ?Sized,
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
    visitor.visit_variant_or_unknown_or_empty_number_format(format)
}

pub fn visit_variant_or_unknown_or_empty_number_format<V>(
    visitor: &mut V,
    node: VariantOrUnknownOrEmpty<NumberFormat>,
) -> V::Outcome
where
    V: TypeVisit + ?Sized,
{
    match node {
        VariantOrUnknownOrEmpty::Item(node) => visitor.visit_number_format(node),
        VariantOrUnknownOrEmpty::Unknown(_) => V::Outcome::new(),
        VariantOrUnknownOrEmpty::Empty => V::Outcome::new(),
    }
}

pub fn visit_number_format<V>(visitor: &mut V, node: NumberFormat) -> V::Outcome
where
    V: TypeVisit + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_string_type<V>(visitor: &mut V, node: StringType) -> V::Outcome
where
    V: TypeVisit + ?Sized,
{
    let StringType {
        format,
        pattern: _,
        enumeration: _,
        min_length: _,
        max_length: _,
    } = node;
    visitor.visit_variant_or_unknown_or_empty_string_format(format)
}

pub fn visit_variant_or_unknown_or_empty_string_format<V>(
    visitor: &mut V,
    node: VariantOrUnknownOrEmpty<StringFormat>,
) -> V::Outcome
where
    V: TypeVisit + ?Sized,
{
    match node {
        VariantOrUnknownOrEmpty::Item(node) => visitor.visit_string_format(node),
        VariantOrUnknownOrEmpty::Unknown(_) => V::Outcome::new(),
        VariantOrUnknownOrEmpty::Empty => V::Outcome::new(),
    }
}

pub fn visit_string_format<V>(visitor: &mut V, node: StringFormat) -> V::Outcome
where
    V: TypeVisit + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_external_documentation<V>(visitor: &mut V, node: ExternalDocumentation) -> V::Outcome
where
    V: TypeVisit + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_extensions<V>(visitor: &mut V, node: Ext) -> V::Outcome
where
    V: TypeVisit + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}
