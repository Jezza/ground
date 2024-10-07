#![allow(unused)]
use super::VisitorOutcome;
use openapiv3::{
    APIKeyLocation, AdditionalProperties, AnySchema, ArrayType, AuthorizationCodeOAuth2Flow,
    BooleanType, Callback, ClientCredentialsOAuth2Flow, Components, Contact, CookieStyle,
    Discriminator, Encoding, Example, ExternalDocumentation, Header, HeaderStyle,
    ImplicitOAuth2Flow, Info, IntegerFormat, IntegerType, License, Link, LinkOperation, MediaType,
    NumberFormat, NumberType, OAuth2Flows, ObjectType, OpenAPI, Operation, Parameter,
    ParameterData, ParameterSchemaOrContent, PasswordOAuth2Flow, PathItem, PathStyle, Paths,
    QueryStyle, ReferenceOr, ReferenceOr::Item, RequestBody, Response, Responses, Schema,
    SchemaData, SchemaKind, SecurityScheme, Server, ServerVariable, StringFormat, StringType, Tag,
    Type, VariantOrUnknownOrEmpty,
};

pub trait VisitMut<'openapi>: super::type_visit_mut::TypeVisitMut<'openapi> {
    fn visit_openapi_mut(&mut self, node: &'openapi mut OpenAPI) -> Self::Outcome {
        visit_openapi_mut(self, node)
    }
    fn visit_tag_mut(&mut self, node: &'openapi mut Tag) -> Self::Outcome {
        visit_tag_mut(self, node)
    }
    fn visit_components_mut(&mut self, node: &'openapi mut Components) -> Self::Outcome {
        visit_components_mut(self, node)
    }
    fn visit_reference_or_callback_mut(
        &mut self,
        node: &'openapi mut ReferenceOr<Callback>,
    ) -> Self::Outcome {
        visit_reference_or_callback_mut(self, node)
    }
    fn visit_callback_mut(&mut self, node: &'openapi mut Callback) -> Self::Outcome {
        visit_callback_mut(self, node)
    }
    fn visit_reference_or_security_scheme_mut(
        &mut self,
        node: &'openapi mut ReferenceOr<SecurityScheme>,
    ) -> Self::Outcome {
        visit_reference_or_security_scheme_mut(self, node)
    }
    fn visit_security_scheme_mut(&mut self, node: &'openapi mut SecurityScheme) -> Self::Outcome {
        visit_security_scheme_mut(self, node)
    }
    fn visit_oauth2_flows_mut(&mut self, node: &'openapi mut OAuth2Flows) -> Self::Outcome {
        visit_oauth2_flows_mut(self, node)
    }
    fn visit_oauth2_client_credential_flow_mut(
        &mut self,
        node: &'openapi mut ClientCredentialsOAuth2Flow,
    ) -> Self::Outcome {
        visit_oauth2_client_credential_flow_mut(self, node)
    }
    fn visit_oauth2_authorization_code_flow_mut(
        &mut self,
        node: &'openapi mut AuthorizationCodeOAuth2Flow,
    ) -> Self::Outcome {
        visit_oauth2_authorization_code_flow_mut(self, node)
    }
    fn visit_oauth2_implicit_flow_mut(
        &mut self,
        node: &'openapi mut ImplicitOAuth2Flow,
    ) -> Self::Outcome {
        visit_oauth2_implicit_flow_mut(self, node)
    }
    fn visit_oauth2_password_flow_mut(
        &mut self,
        node: &'openapi mut PasswordOAuth2Flow,
    ) -> Self::Outcome {
        visit_oauth2_password_flow_mut(self, node)
    }
    fn visit_api_key_location_mut(&mut self, node: &'openapi mut APIKeyLocation) -> Self::Outcome {
        visit_api_key_location_mut(self, node)
    }
    fn visit_paths_mut(&mut self, node: &'openapi mut Paths) -> Self::Outcome {
        visit_paths_mut(self, node)
    }
    fn visit_reference_or_path_item_mut(
        &mut self,
        node: &'openapi mut ReferenceOr<PathItem>,
    ) -> Self::Outcome {
        visit_reference_or_path_item_mut(self, node)
    }
    fn visit_path_item_mut(&mut self, node: &'openapi mut PathItem) -> Self::Outcome {
        visit_path_item_mut(self, node)
    }
    fn visit_operation_mut(&mut self, node: &'openapi mut Operation) -> Self::Outcome {
        visit_operation_mut(self, node)
    }
    fn visit_responses_mut(&mut self, node: &'openapi mut Responses) -> Self::Outcome {
        visit_responses_mut(self, node)
    }
    fn visit_reference_or_response_mut(
        &mut self,
        node: &'openapi mut ReferenceOr<Response>,
    ) -> Self::Outcome {
        visit_reference_or_response_mut(self, node)
    }
    fn visit_response_mut(&mut self, node: &'openapi mut Response) -> Self::Outcome {
        visit_response_mut(self, node)
    }
    fn visit_reference_or_link_mut(
        &mut self,
        node: &'openapi mut ReferenceOr<Link>,
    ) -> Self::Outcome {
        visit_reference_or_link_mut(self, node)
    }
    fn visit_link_mut(&mut self, node: &'openapi mut Link) -> Self::Outcome {
        visit_link_mut(self, node)
    }
    fn visit_link_operation_mut(&mut self, node: &'openapi mut LinkOperation) -> Self::Outcome {
        visit_link_operation_mut(self, node)
    }
    fn visit_reference_or_request_body_mut(
        &mut self,
        node: &'openapi mut ReferenceOr<RequestBody>,
    ) -> Self::Outcome {
        visit_reference_or_request_body_mut(self, node)
    }
    fn visit_request_body_mut(&mut self, node: &'openapi mut RequestBody) -> Self::Outcome {
        visit_request_body_mut(self, node)
    }
    fn visit_reference_or_parameter_mut(
        &mut self,
        node: &'openapi mut ReferenceOr<Parameter>,
    ) -> Self::Outcome {
        visit_reference_or_parameter_mut(self, node)
    }
    fn visit_parameter_mut(&mut self, node: &'openapi mut Parameter) -> Self::Outcome {
        visit_parameter_mut(self, node)
    }
    fn visit_cookie_style_mut(&mut self, node: &'openapi mut CookieStyle) -> Self::Outcome {
        visit_cookie_style_mut(self, node)
    }
    fn visit_path_style_mut(&mut self, node: &'openapi mut PathStyle) -> Self::Outcome {
        visit_path_style_mut(self, node)
    }
    fn visit_parameter_data_mut(&mut self, node: &'openapi mut ParameterData) -> Self::Outcome {
        visit_parameter_data_mut(self, node)
    }
    fn visit_parameter_schema_or_content_mut(
        &mut self,
        node: &'openapi mut ParameterSchemaOrContent,
    ) -> Self::Outcome {
        visit_parameter_schema_or_content_mut(self, node)
    }
    fn visit_media_type_mut(&mut self, node: &'openapi mut MediaType) -> Self::Outcome {
        visit_media_type_mut(self, node)
    }
    fn visit_encoding_mut(&mut self, node: &'openapi mut Encoding) -> Self::Outcome {
        visit_encoding_mut(self, node)
    }
    fn visit_query_style_mut(&mut self, node: &'openapi mut QueryStyle) -> Self::Outcome {
        visit_query_style_mut(self, node)
    }
    fn visit_reference_or_header_mut(
        &mut self,
        node: &'openapi mut ReferenceOr<Header>,
    ) -> Self::Outcome {
        visit_reference_or_header_mut(self, node)
    }
    fn visit_header_mut(&mut self, node: &'openapi mut Header) -> Self::Outcome {
        visit_header_mut(self, node)
    }
    fn visit_header_style_mut(&mut self, node: &'openapi mut HeaderStyle) -> Self::Outcome {
        visit_header_style_mut(self, node)
    }
    fn visit_reference_or_example_mut(
        &mut self,
        node: &'openapi mut ReferenceOr<Example>,
    ) -> Self::Outcome {
        visit_reference_or_example_mut(self, node)
    }
    fn visit_example_mut(&mut self, node: &'openapi mut Example) -> Self::Outcome {
        visit_example_mut(self, node)
    }
    fn visit_server_mut(&mut self, node: &'openapi mut Server) -> Self::Outcome {
        visit_server_mut(self, node)
    }
    fn visit_server_variable_mut(&mut self, node: &'openapi mut ServerVariable) -> Self::Outcome {
        visit_server_variable_mut(self, node)
    }
    fn visit_info_mut(&mut self, node: &'openapi mut Info) -> Self::Outcome {
        visit_info_mut(self, node)
    }
    fn visit_contact_mut(&mut self, node: &'openapi mut Contact) -> Self::Outcome {
        visit_contact_mut(self, node)
    }
    fn visit_license_mut(&mut self, node: &'openapi mut License) -> Self::Outcome {
        visit_license_mut(self, node)
    }
}

pub fn visit_openapi_mut<'openapi, V>(visitor: &mut V, node: &'openapi mut OpenAPI) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let OpenAPI {
        openapi: _,
        info,
        servers,
        paths,
        components,
        security: _,
        tags,
        external_docs,
        extensions,
    } = node;
    let mut outcome = visitor.visit_info_mut(info);
    for node in servers {
        let other = visitor.visit_server_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_paths_mut(paths);
    outcome = V::Outcome::reduce(outcome, other);
    if let Some(node) = components.as_mut() {
        let other = visitor.visit_components_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for node in tags {
        let other = visitor.visit_tag_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = external_docs.as_mut() {
        let other = visitor.visit_external_documentation_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_mut(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_tag_mut<'openapi, V>(visitor: &mut V, node: &'openapi mut Tag) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let Tag {
        name: _,
        description: _,
        external_docs,
        extensions,
    } = node;
    let mut outcome = V::Outcome::new();
    if let Some(node) = external_docs.as_mut() {
        let other = visitor.visit_external_documentation_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_mut(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_components_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut Components,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let Components {
        security_schemes,
        responses,
        parameters,
        examples,
        request_bodies,
        headers,
        schemas,
        links,
        callbacks,
        extensions,
    } = node;
    let mut outcome = V::Outcome::new();
    for (_, node) in security_schemes {
        let other = visitor.visit_reference_or_security_scheme_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in responses {
        let other = visitor.visit_reference_or_response_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in parameters {
        let other = visitor.visit_reference_or_parameter_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in examples {
        let other = visitor.visit_reference_or_example_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in request_bodies {
        let other = visitor.visit_reference_or_request_body_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in headers {
        let other = visitor.visit_reference_or_header_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in schemas {
        let other = visitor.visit_reference_or_schema_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in links {
        let other = visitor.visit_reference_or_link_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in callbacks {
        let other = visitor.visit_reference_or_callback_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_mut(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_reference_or_callback_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut ReferenceOr<Callback>,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_callback_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_callback_mut<'openapi, V>(visitor: &mut V, node: &'openapi mut Callback) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let mut outcome = V::Outcome::new();
    for (_, node) in node {
        let other = visitor.visit_path_item_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_reference_or_security_scheme_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut ReferenceOr<SecurityScheme>,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_security_scheme_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_security_scheme_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut SecurityScheme,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    match node {
        SecurityScheme::APIKey {
            location,
            name: _,
            description: _,
            extensions,
        } => {
            let outcome = visitor.visit_api_key_location_mut(location);
            let other = visitor.visit_extensions_mut(extensions);
            V::Outcome::reduce(outcome, other)
        }
        SecurityScheme::HTTP {
            scheme: _,
            bearer_format: _,
            description: _,
            extensions,
        } => visitor.visit_extensions_mut(extensions),
        SecurityScheme::OAuth2 {
            flows,
            description: _,
            extensions,
        } => {
            let outcome = visitor.visit_oauth2_flows_mut(flows);
            let other = visitor.visit_extensions_mut(extensions);
            V::Outcome::reduce(outcome, other)
        }
        SecurityScheme::OpenIDConnect {
            open_id_connect_url: _,
            description: _,
            extensions,
        } => visitor.visit_extensions_mut(extensions),
    }
}

pub fn visit_oauth2_flows_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut OAuth2Flows,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let OAuth2Flows {
        implicit,
        password,
        client_credentials,
        authorization_code,
        extensions,
    } = node;
    let mut outcome = V::Outcome::new();
    if let Some(node) = implicit.as_mut() {
        let other = visitor.visit_oauth2_implicit_flow_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = password.as_mut() {
        let other = visitor.visit_oauth2_password_flow_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = client_credentials.as_mut() {
        let other = visitor.visit_oauth2_client_credential_flow_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = authorization_code.as_mut() {
        let other = visitor.visit_oauth2_authorization_code_flow_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_mut(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_oauth2_client_credential_flow_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut ClientCredentialsOAuth2Flow,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_oauth2_authorization_code_flow_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut AuthorizationCodeOAuth2Flow,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_oauth2_implicit_flow_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut ImplicitOAuth2Flow,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_oauth2_password_flow_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut PasswordOAuth2Flow,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_api_key_location_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut APIKeyLocation,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_paths_mut<'openapi, V>(visitor: &mut V, node: &'openapi mut Paths) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let Paths { paths, extensions } = node;
    let mut outcome = V::Outcome::new();
    for (_, node) in paths {
        let other = visitor.visit_reference_or_path_item_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_mut(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_reference_or_path_item_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut ReferenceOr<PathItem>,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_path_item_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_path_item_mut<'openapi, V>(visitor: &mut V, node: &'openapi mut PathItem) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let PathItem {
        summary: _,
        description: _,
        get,
        put,
        post,
        delete,
        options,
        head,
        patch,
        trace,
        servers,
        parameters,
        extensions,
    } = node;
    let mut outcome = V::Outcome::new();
    if let Some(node) = get.as_mut() {
        let other = visitor.visit_operation_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = put.as_mut() {
        let other = visitor.visit_operation_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = post.as_mut() {
        let other = visitor.visit_operation_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = delete.as_mut() {
        let other = visitor.visit_operation_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = options.as_mut() {
        let other = visitor.visit_operation_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = head.as_mut() {
        let other = visitor.visit_operation_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = patch.as_mut() {
        let other = visitor.visit_operation_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = trace.as_mut() {
        let other = visitor.visit_operation_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for node in servers {
        let other = visitor.visit_server_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for node in parameters {
        let other = visitor.visit_reference_or_parameter_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_mut(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_operation_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut Operation,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let Operation {
        tags: _,
        summary: _,
        description: _,
        external_docs,
        operation_id: _,
        parameters,
        request_body,
        responses,
        callbacks,
        deprecated: _,
        security: _,
        servers,
        extensions,
    } = node;
    let mut outcome = V::Outcome::new();
    if let Some(node) = external_docs.as_mut() {
        let other = visitor.visit_external_documentation_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for node in parameters {
        let other = visitor.visit_reference_or_parameter_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = request_body.as_mut() {
        let other = visitor.visit_reference_or_request_body_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_responses_mut(responses);
    outcome = V::Outcome::reduce(outcome, other);
    for (_, node) in callbacks {
        let other = visitor.visit_callback_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for node in servers {
        let other = visitor.visit_server_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_mut(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_responses_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut Responses,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let Responses {
        default,
        responses,
        extensions,
    } = node;
    let mut outcome = V::Outcome::new();
    if let Some(node) = default.as_mut() {
        let other = visitor.visit_reference_or_response_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in responses {
        let other = visitor.visit_reference_or_response_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_mut(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_reference_or_response_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut ReferenceOr<Response>,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_response_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_response_mut<'openapi, V>(visitor: &mut V, node: &'openapi mut Response) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let Response {
        description: _,
        headers,
        content,
        links,
        extensions,
    } = node;
    let mut outcome = V::Outcome::new();
    for (_, node) in headers {
        let other = visitor.visit_reference_or_header_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in content {
        let other = visitor.visit_media_type_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in links {
        let other = visitor.visit_reference_or_link_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_mut(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_reference_or_link_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut ReferenceOr<Link>,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_link_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_link_mut<'openapi, V>(visitor: &mut V, node: &'openapi mut Link) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let Link {
        description: _,
        operation,
        request_body: _,
        parameters: _,
        server,
        extensions,
    } = node;
    let mut outcome = visitor.visit_link_operation_mut(operation);
    if let Some(node) = server.as_mut() {
        let other = visitor.visit_server_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_mut(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_link_operation_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut LinkOperation,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_reference_or_request_body_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut ReferenceOr<RequestBody>,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_request_body_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_request_body_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut RequestBody,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let RequestBody {
        description: _,
        content,
        required: _,
        extensions,
    } = node;
    let mut outcome = V::Outcome::new();
    for (_, node) in content {
        let other = visitor.visit_media_type_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_mut(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_reference_or_parameter_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut ReferenceOr<Parameter>,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_parameter_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_parameter_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut Parameter,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let mut outcome = V::Outcome::new();
    match node {
        Parameter::Query {
            parameter_data,
            allow_reserved: _,
            style,
            allow_empty_value: _,
        } => {
            let other = visitor.visit_parameter_data_mut(parameter_data);
            outcome = V::Outcome::reduce(outcome, other);
            let other = visitor.visit_query_style_mut(style);
            outcome = V::Outcome::reduce(outcome, other);
        }
        Parameter::Header {
            parameter_data,
            style,
        } => {
            let other = visitor.visit_parameter_data_mut(parameter_data);
            outcome = V::Outcome::reduce(outcome, other);
            let other = visitor.visit_header_style_mut(style);
            outcome = V::Outcome::reduce(outcome, other);
        }
        Parameter::Path {
            parameter_data,
            style,
        } => {
            let other = visitor.visit_parameter_data_mut(parameter_data);
            outcome = V::Outcome::reduce(outcome, other);
            let other = visitor.visit_path_style_mut(style);
            outcome = V::Outcome::reduce(outcome, other);
        }
        Parameter::Cookie {
            parameter_data,
            style,
        } => {
            let other = visitor.visit_parameter_data_mut(parameter_data);
            outcome = V::Outcome::reduce(outcome, other);
            let other = visitor.visit_cookie_style_mut(style);
            outcome = V::Outcome::reduce(outcome, other);
        }
    }
    outcome
}

pub fn visit_cookie_style_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut CookieStyle,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_path_style_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut PathStyle,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_parameter_data_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut ParameterData,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let ParameterData {
        name: _,
        description: _,
        required: _,
        deprecated: _,
        format,
        example: _,
        examples,
        explode: _,
        extensions,
    } = node;
    let mut outcome = V::Outcome::new();
    let other = visitor.visit_parameter_schema_or_content_mut(format);
    outcome = V::Outcome::reduce(outcome, other);
    for (_, node) in examples {
        let other = visitor.visit_reference_or_example_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_mut(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_parameter_schema_or_content_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut ParameterSchemaOrContent,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    match node {
        ParameterSchemaOrContent::Schema(node) => visitor.visit_reference_or_schema_mut(node),
        ParameterSchemaOrContent::Content(node) => {
            let mut outcome = V::Outcome::new();
            for (_, node) in node {
                let other = visitor.visit_media_type_mut(node);
                outcome = V::Outcome::reduce(outcome, other);
            }
            outcome
        }
    }
}

pub fn visit_media_type_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut MediaType,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let MediaType {
        schema,
        example: _,
        examples,
        encoding,
        extensions,
    } = node;
    let mut outcome = V::Outcome::new();
    if let Some(node) = schema.as_mut() {
        let other = visitor.visit_reference_or_schema_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in examples {
        let other = visitor.visit_reference_or_example_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in encoding {
        let other = visitor.visit_encoding_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_mut(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_encoding_mut<'openapi, V>(visitor: &mut V, node: &'openapi mut Encoding) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let Encoding {
        content_type: _,
        headers,
        style,
        explode: _,
        allow_reserved: _,
        extensions,
    } = node;
    let mut outcome = V::Outcome::new();
    for (_, node) in headers {
        let other = visitor.visit_reference_or_header_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = style.as_mut() {
        let other = visitor.visit_query_style_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_mut(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_query_style_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut QueryStyle,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_reference_or_header_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut ReferenceOr<Header>,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_header_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_header_mut<'openapi, V>(visitor: &mut V, node: &'openapi mut Header) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let Header {
        description: _,
        style,
        required: _,
        deprecated: _,
        format,
        example: _,
        examples,
        extensions,
    } = node;
    let mut outcome = visitor.visit_header_style_mut(style);
    let other = visitor.visit_parameter_schema_or_content_mut(format);
    outcome = V::Outcome::reduce(outcome, other);
    for (_, node) in examples {
        let other = visitor.visit_reference_or_example_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_mut(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_header_style_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut HeaderStyle,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_reference_or_example_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut ReferenceOr<Example>,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_example_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_example_mut<'openapi, V>(visitor: &mut V, node: &'openapi mut Example) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_reference_or_schema_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut ReferenceOr<Schema>,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
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
    V: VisitMut<'openapi> + ?Sized,
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
    V: VisitMut<'openapi> + ?Sized,
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
    V: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_schema_kind_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut SchemaKind,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
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
    V: VisitMut<'openapi> + ?Sized,
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
    V: VisitMut<'openapi> + ?Sized,
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
    V: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_array_type_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut ArrayType,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
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
    V: VisitMut<'openapi> + ?Sized,
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
    V: VisitMut<'openapi> + ?Sized,
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
    V: VisitMut<'openapi> + ?Sized,
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
    V: VisitMut<'openapi> + ?Sized,
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
    V: VisitMut<'openapi> + ?Sized,
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
    V: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_number_type_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut NumberType,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
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
    V: VisitMut<'openapi> + ?Sized,
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
    V: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_string_type_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut StringType,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
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
    V: VisitMut<'openapi> + ?Sized,
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
    V: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_external_documentation_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut ExternalDocumentation,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_server_mut<'openapi, V>(visitor: &mut V, node: &'openapi mut Server) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let Server {
        url: _,
        description: _,
        variables,
        extensions,
    } = node;
    let mut outcome = V::Outcome::new();
    if let Some(node) = variables.as_mut() {
        for (_, node) in node {
            let other = visitor.visit_server_variable_mut(node);
            outcome = V::Outcome::reduce(outcome, other);
        }
    }
    let other = visitor.visit_extensions_mut(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_server_variable_mut<'openapi, V>(
    visitor: &mut V,
    node: &'openapi mut ServerVariable,
) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_info_mut<'openapi, V>(visitor: &mut V, node: &'openapi mut Info) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let Info {
        title: _,
        description: _,
        terms_of_service: _,
        contact,
        license,
        version: _,
        extensions,
    } = node;
    let mut outcome = V::Outcome::new();
    if let Some(node) = contact.as_mut() {
        let other = visitor.visit_contact_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = license.as_mut() {
        let other = visitor.visit_license_mut(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_mut(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_contact_mut<'openapi, V>(visitor: &mut V, node: &'openapi mut Contact) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_license_mut<'openapi, V>(visitor: &mut V, node: &'openapi mut License) -> V::Outcome
where
    V: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}
