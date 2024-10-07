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

pub trait VisitRef<'openapi>: super::type_visit_ref::TypeVisitRef<'openapi> {
    fn visit_openapi_ref(&mut self, node: &'openapi OpenAPI) -> Self::Outcome {
        visit_openapi_ref(self, node)
    }
    fn visit_tag_ref(&mut self, node: &'openapi Tag) -> Self::Outcome {
        visit_tag_ref(self, node)
    }
    fn visit_components_ref(&mut self, node: &'openapi Components) -> Self::Outcome {
        visit_components_ref(self, node)
    }
    fn visit_reference_or_callback_ref(
        &mut self,
        node: &'openapi ReferenceOr<Callback>,
    ) -> Self::Outcome {
        visit_reference_or_callback_ref(self, node)
    }
    fn visit_callback_ref(&mut self, node: &'openapi Callback) -> Self::Outcome {
        visit_callback_ref(self, node)
    }
    fn visit_reference_or_security_scheme_ref(
        &mut self,
        node: &'openapi ReferenceOr<SecurityScheme>,
    ) -> Self::Outcome {
        visit_reference_or_security_scheme_ref(self, node)
    }
    fn visit_security_scheme_ref(&mut self, node: &'openapi SecurityScheme) -> Self::Outcome {
        visit_security_scheme_ref(self, node)
    }
    fn visit_oauth2_flows_ref(&mut self, node: &'openapi OAuth2Flows) -> Self::Outcome {
        visit_oauth2_flows_ref(self, node)
    }
    fn visit_oauth2_client_credential_flow_ref(
        &mut self,
        node: &'openapi ClientCredentialsOAuth2Flow,
    ) -> Self::Outcome {
        visit_oauth2_client_credential_flow_ref(self, node)
    }
    fn visit_oauth2_authorization_code_flow_ref(
        &mut self,
        node: &'openapi AuthorizationCodeOAuth2Flow,
    ) -> Self::Outcome {
        visit_oauth2_authorization_code_flow_ref(self, node)
    }
    fn visit_oauth2_implicit_flow_ref(
        &mut self,
        node: &'openapi ImplicitOAuth2Flow,
    ) -> Self::Outcome {
        visit_oauth2_implicit_flow_ref(self, node)
    }
    fn visit_oauth2_password_flow_ref(
        &mut self,
        node: &'openapi PasswordOAuth2Flow,
    ) -> Self::Outcome {
        visit_oauth2_password_flow_ref(self, node)
    }
    fn visit_api_key_location_ref(&mut self, node: &'openapi APIKeyLocation) -> Self::Outcome {
        visit_api_key_location_ref(self, node)
    }
    fn visit_paths_ref(&mut self, node: &'openapi Paths) -> Self::Outcome {
        visit_paths_ref(self, node)
    }
    fn visit_reference_or_path_item_ref(
        &mut self,
        node: &'openapi ReferenceOr<PathItem>,
    ) -> Self::Outcome {
        visit_reference_or_path_item_ref(self, node)
    }
    fn visit_path_item_ref(&mut self, node: &'openapi PathItem) -> Self::Outcome {
        visit_path_item_ref(self, node)
    }
    fn visit_operation_ref(&mut self, node: &'openapi Operation) -> Self::Outcome {
        visit_operation_ref(self, node)
    }
    fn visit_responses_ref(&mut self, node: &'openapi Responses) -> Self::Outcome {
        visit_responses_ref(self, node)
    }
    fn visit_reference_or_response_ref(
        &mut self,
        node: &'openapi ReferenceOr<Response>,
    ) -> Self::Outcome {
        visit_reference_or_response_ref(self, node)
    }
    fn visit_response_ref(&mut self, node: &'openapi Response) -> Self::Outcome {
        visit_response_ref(self, node)
    }
    fn visit_reference_or_link_ref(&mut self, node: &'openapi ReferenceOr<Link>) -> Self::Outcome {
        visit_reference_or_link_ref(self, node)
    }
    fn visit_link_ref(&mut self, node: &'openapi Link) -> Self::Outcome {
        visit_link_ref(self, node)
    }
    fn visit_link_operation_ref(&mut self, node: &'openapi LinkOperation) -> Self::Outcome {
        visit_link_operation_ref(self, node)
    }
    fn visit_reference_or_request_body_ref(
        &mut self,
        node: &'openapi ReferenceOr<RequestBody>,
    ) -> Self::Outcome {
        visit_reference_or_request_body_ref(self, node)
    }
    fn visit_request_body_ref(&mut self, node: &'openapi RequestBody) -> Self::Outcome {
        visit_request_body_ref(self, node)
    }
    fn visit_reference_or_parameter_ref(
        &mut self,
        node: &'openapi ReferenceOr<Parameter>,
    ) -> Self::Outcome {
        visit_reference_or_parameter_ref(self, node)
    }
    fn visit_parameter_ref(&mut self, node: &'openapi Parameter) -> Self::Outcome {
        visit_parameter_ref(self, node)
    }
    fn visit_cookie_style_ref(&mut self, node: &'openapi CookieStyle) -> Self::Outcome {
        visit_cookie_style_ref(self, node)
    }
    fn visit_path_style_ref(&mut self, node: &'openapi PathStyle) -> Self::Outcome {
        visit_path_style_ref(self, node)
    }
    fn visit_parameter_data_ref(&mut self, node: &'openapi ParameterData) -> Self::Outcome {
        visit_parameter_data_ref(self, node)
    }
    fn visit_parameter_schema_or_content_ref(
        &mut self,
        node: &'openapi ParameterSchemaOrContent,
    ) -> Self::Outcome {
        visit_parameter_schema_or_content_ref(self, node)
    }
    fn visit_media_type_ref(&mut self, node: &'openapi MediaType) -> Self::Outcome {
        visit_media_type_ref(self, node)
    }
    fn visit_encoding_ref(&mut self, node: &'openapi Encoding) -> Self::Outcome {
        visit_encoding_ref(self, node)
    }
    fn visit_query_style_ref(&mut self, node: &'openapi QueryStyle) -> Self::Outcome {
        visit_query_style_ref(self, node)
    }
    fn visit_reference_or_header_ref(
        &mut self,
        node: &'openapi ReferenceOr<Header>,
    ) -> Self::Outcome {
        visit_reference_or_header_ref(self, node)
    }
    fn visit_header_ref(&mut self, node: &'openapi Header) -> Self::Outcome {
        visit_header_ref(self, node)
    }
    fn visit_header_style_ref(&mut self, node: &'openapi HeaderStyle) -> Self::Outcome {
        visit_header_style_ref(self, node)
    }
    fn visit_reference_or_example_ref(
        &mut self,
        node: &'openapi ReferenceOr<Example>,
    ) -> Self::Outcome {
        visit_reference_or_example_ref(self, node)
    }
    fn visit_example_ref(&mut self, node: &'openapi Example) -> Self::Outcome {
        visit_example_ref(self, node)
    }
    fn visit_server_ref(&mut self, node: &'openapi Server) -> Self::Outcome {
        visit_server_ref(self, node)
    }
    fn visit_server_variable_ref(&mut self, node: &'openapi ServerVariable) -> Self::Outcome {
        visit_server_variable_ref(self, node)
    }
    fn visit_info_ref(&mut self, node: &'openapi Info) -> Self::Outcome {
        visit_info_ref(self, node)
    }
    fn visit_contact_ref(&mut self, node: &'openapi Contact) -> Self::Outcome {
        visit_contact_ref(self, node)
    }
    fn visit_license_ref(&mut self, node: &'openapi License) -> Self::Outcome {
        visit_license_ref(self, node)
    }
}

pub fn visit_openapi_ref<'openapi, V>(visitor: &mut V, node: &'openapi OpenAPI) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
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
    let mut outcome = visitor.visit_info_ref(info);
    for node in servers {
        let other = visitor.visit_server_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_paths_ref(paths);
    outcome = V::Outcome::reduce(outcome, other);
    if let Some(node) = components.as_ref() {
        let other = visitor.visit_components_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for node in tags {
        let other = visitor.visit_tag_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = external_docs.as_ref() {
        let other = visitor.visit_external_documentation_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_ref(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_tag_ref<'openapi, V>(visitor: &mut V, node: &'openapi Tag) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let Tag {
        name: _,
        description: _,
        external_docs,
        extensions,
    } = node;
    let mut outcome = V::Outcome::new();
    if let Some(node) = external_docs.as_ref() {
        let other = visitor.visit_external_documentation_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_ref(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_components_ref<'openapi, V>(visitor: &mut V, node: &'openapi Components) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
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
        let other = visitor.visit_reference_or_security_scheme_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in responses {
        let other = visitor.visit_reference_or_response_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in parameters {
        let other = visitor.visit_reference_or_parameter_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in examples {
        let other = visitor.visit_reference_or_example_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in request_bodies {
        let other = visitor.visit_reference_or_request_body_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in headers {
        let other = visitor.visit_reference_or_header_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in schemas {
        let other = visitor.visit_reference_or_schema_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in links {
        let other = visitor.visit_reference_or_link_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in callbacks {
        let other = visitor.visit_reference_or_callback_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_ref(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_reference_or_callback_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi ReferenceOr<Callback>,
) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_callback_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_callback_ref<'openapi, V>(visitor: &mut V, node: &'openapi Callback) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let mut outcome = V::Outcome::new();
    for (_, node) in node {
        let other = visitor.visit_path_item_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_reference_or_security_scheme_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi ReferenceOr<SecurityScheme>,
) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_security_scheme_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_security_scheme_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi SecurityScheme,
) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    match node {
        SecurityScheme::APIKey {
            location,
            name: _,
            description: _,
            extensions,
        } => {
            let outcome = visitor.visit_api_key_location_ref(location);
            let other = visitor.visit_extensions_ref(extensions);
            V::Outcome::reduce(outcome, other)
        }
        SecurityScheme::HTTP {
            scheme: _,
            bearer_format: _,
            description: _,
            extensions,
        } => visitor.visit_extensions_ref(extensions),
        SecurityScheme::OAuth2 {
            flows,
            description: _,
            extensions,
        } => {
            let outcome = visitor.visit_oauth2_flows_ref(flows);
            let other = visitor.visit_extensions_ref(extensions);
            V::Outcome::reduce(outcome, other)
        }
        SecurityScheme::OpenIDConnect {
            open_id_connect_url: _,
            description: _,
            extensions,
        } => visitor.visit_extensions_ref(extensions),
    }
}

pub fn visit_oauth2_flows_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi OAuth2Flows,
) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let OAuth2Flows {
        implicit,
        password,
        client_credentials,
        authorization_code,
        extensions,
    } = node;
    let mut outcome = V::Outcome::new();
    if let Some(node) = implicit.as_ref() {
        let other = visitor.visit_oauth2_implicit_flow_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = password.as_ref() {
        let other = visitor.visit_oauth2_password_flow_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = client_credentials.as_ref() {
        let other = visitor.visit_oauth2_client_credential_flow_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = authorization_code.as_ref() {
        let other = visitor.visit_oauth2_authorization_code_flow_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_ref(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_oauth2_client_credential_flow_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi ClientCredentialsOAuth2Flow,
) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_oauth2_authorization_code_flow_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi AuthorizationCodeOAuth2Flow,
) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_oauth2_implicit_flow_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi ImplicitOAuth2Flow,
) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_oauth2_password_flow_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi PasswordOAuth2Flow,
) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_api_key_location_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi APIKeyLocation,
) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_paths_ref<'openapi, V>(visitor: &mut V, node: &'openapi Paths) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let Paths { paths, extensions } = node;
    let mut outcome = V::Outcome::new();
    for (_, node) in paths {
        let other = visitor.visit_reference_or_path_item_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_ref(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_reference_or_path_item_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi ReferenceOr<PathItem>,
) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_path_item_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_path_item_ref<'openapi, V>(visitor: &mut V, node: &'openapi PathItem) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
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
    if let Some(node) = get.as_ref() {
        let other = visitor.visit_operation_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = put.as_ref() {
        let other = visitor.visit_operation_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = post.as_ref() {
        let other = visitor.visit_operation_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = delete.as_ref() {
        let other = visitor.visit_operation_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = options.as_ref() {
        let other = visitor.visit_operation_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = head.as_ref() {
        let other = visitor.visit_operation_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = patch.as_ref() {
        let other = visitor.visit_operation_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = trace.as_ref() {
        let other = visitor.visit_operation_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for node in servers {
        let other = visitor.visit_server_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for node in parameters {
        let other = visitor.visit_reference_or_parameter_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_ref(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_operation_ref<'openapi, V>(visitor: &mut V, node: &'openapi Operation) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
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
    if let Some(node) = external_docs.as_ref() {
        let other = visitor.visit_external_documentation_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for node in parameters {
        let other = visitor.visit_reference_or_parameter_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = request_body.as_ref() {
        let other = visitor.visit_reference_or_request_body_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_responses_ref(responses);
    outcome = V::Outcome::reduce(outcome, other);
    for (_, node) in callbacks {
        let other = visitor.visit_callback_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for node in servers {
        let other = visitor.visit_server_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_ref(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_responses_ref<'openapi, V>(visitor: &mut V, node: &'openapi Responses) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let Responses {
        default,
        responses,
        extensions,
    } = node;
    let mut outcome = V::Outcome::new();
    if let Some(node) = default.as_ref() {
        let other = visitor.visit_reference_or_response_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in responses {
        let other = visitor.visit_reference_or_response_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_ref(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_reference_or_response_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi ReferenceOr<Response>,
) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_response_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_response_ref<'openapi, V>(visitor: &mut V, node: &'openapi Response) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
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
        let other = visitor.visit_reference_or_header_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in content {
        let other = visitor.visit_media_type_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in links {
        let other = visitor.visit_reference_or_link_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_ref(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_reference_or_link_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi ReferenceOr<Link>,
) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_link_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_link_ref<'openapi, V>(visitor: &mut V, node: &'openapi Link) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let Link {
        description: _,
        operation,
        request_body: _,
        parameters: _,
        server,
        extensions,
    } = node;
    let mut outcome = visitor.visit_link_operation_ref(operation);
    if let Some(node) = server.as_ref() {
        let other = visitor.visit_server_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_ref(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_link_operation_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi LinkOperation,
) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_reference_or_request_body_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi ReferenceOr<RequestBody>,
) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_request_body_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_request_body_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi RequestBody,
) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let RequestBody {
        description: _,
        content,
        required: _,
        extensions,
    } = node;
    let mut outcome = V::Outcome::new();
    for (_, node) in content {
        let other = visitor.visit_media_type_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_ref(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_reference_or_parameter_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi ReferenceOr<Parameter>,
) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_parameter_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_parameter_ref<'openapi, V>(visitor: &mut V, node: &'openapi Parameter) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let mut outcome = V::Outcome::new();
    match node {
        Parameter::Query {
            parameter_data,
            allow_reserved: _,
            style,
            allow_empty_value: _,
        } => {
            let other = visitor.visit_parameter_data_ref(parameter_data);
            outcome = V::Outcome::reduce(outcome, other);
            let other = visitor.visit_query_style_ref(style);
            outcome = V::Outcome::reduce(outcome, other);
        }
        Parameter::Header {
            parameter_data,
            style,
        } => {
            let other = visitor.visit_parameter_data_ref(parameter_data);
            outcome = V::Outcome::reduce(outcome, other);
            let other = visitor.visit_header_style_ref(style);
            outcome = V::Outcome::reduce(outcome, other);
        }
        Parameter::Path {
            parameter_data,
            style,
        } => {
            let other = visitor.visit_parameter_data_ref(parameter_data);
            outcome = V::Outcome::reduce(outcome, other);
            let other = visitor.visit_path_style_ref(style);
            outcome = V::Outcome::reduce(outcome, other);
        }
        Parameter::Cookie {
            parameter_data,
            style,
        } => {
            let other = visitor.visit_parameter_data_ref(parameter_data);
            outcome = V::Outcome::reduce(outcome, other);
            let other = visitor.visit_cookie_style_ref(style);
            outcome = V::Outcome::reduce(outcome, other);
        }
    }
    outcome
}

pub fn visit_cookie_style_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi CookieStyle,
) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_path_style_ref<'openapi, V>(visitor: &mut V, node: &'openapi PathStyle) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_parameter_data_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi ParameterData,
) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
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
    let other = visitor.visit_parameter_schema_or_content_ref(format);
    outcome = V::Outcome::reduce(outcome, other);
    for (_, node) in examples {
        let other = visitor.visit_reference_or_example_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_ref(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_parameter_schema_or_content_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi ParameterSchemaOrContent,
) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    match node {
        ParameterSchemaOrContent::Schema(node) => visitor.visit_reference_or_schema_ref(node),
        ParameterSchemaOrContent::Content(node) => {
            let mut outcome = V::Outcome::new();
            for (_, node) in node {
                let other = visitor.visit_media_type_ref(node);
                outcome = V::Outcome::reduce(outcome, other);
            }
            outcome
        }
    }
}

pub fn visit_media_type_ref<'openapi, V>(visitor: &mut V, node: &'openapi MediaType) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let MediaType {
        schema,
        example: _,
        examples,
        encoding,
        extensions,
    } = node;
    let mut outcome = V::Outcome::new();
    if let Some(node) = schema.as_ref() {
        let other = visitor.visit_reference_or_schema_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in examples {
        let other = visitor.visit_reference_or_example_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in encoding {
        let other = visitor.visit_encoding_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_ref(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_encoding_ref<'openapi, V>(visitor: &mut V, node: &'openapi Encoding) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
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
        let other = visitor.visit_reference_or_header_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = style.as_ref() {
        let other = visitor.visit_query_style_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_ref(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_query_style_ref<'openapi, V>(visitor: &mut V, node: &'openapi QueryStyle) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_reference_or_header_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi ReferenceOr<Header>,
) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_header_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_header_ref<'openapi, V>(visitor: &mut V, node: &'openapi Header) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
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
    let mut outcome = visitor.visit_header_style_ref(style);
    let other = visitor.visit_parameter_schema_or_content_ref(format);
    outcome = V::Outcome::reduce(outcome, other);
    for (_, node) in examples {
        let other = visitor.visit_reference_or_example_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_ref(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_header_style_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi HeaderStyle,
) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_reference_or_example_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi ReferenceOr<Example>,
) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_example_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_example_ref<'openapi, V>(visitor: &mut V, node: &'openapi Example) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_reference_or_schema_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi ReferenceOr<Schema>,
) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
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
    V: VisitRef<'openapi> + ?Sized,
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
    V: VisitRef<'openapi> + ?Sized,
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
    V: VisitRef<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_schema_kind_ref<'openapi, V>(visitor: &mut V, node: &'openapi SchemaKind) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
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
    V: VisitRef<'openapi> + ?Sized,
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
    V: VisitRef<'openapi> + ?Sized,
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
    V: VisitRef<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_array_type_ref<'openapi, V>(visitor: &mut V, node: &'openapi ArrayType) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
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
    V: VisitRef<'openapi> + ?Sized,
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
    V: VisitRef<'openapi> + ?Sized,
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
    V: VisitRef<'openapi> + ?Sized,
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
    V: VisitRef<'openapi> + ?Sized,
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
    V: VisitRef<'openapi> + ?Sized,
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
    V: VisitRef<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_number_type_ref<'openapi, V>(visitor: &mut V, node: &'openapi NumberType) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
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
    V: VisitRef<'openapi> + ?Sized,
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
    V: VisitRef<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_string_type_ref<'openapi, V>(visitor: &mut V, node: &'openapi StringType) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
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
    V: VisitRef<'openapi> + ?Sized,
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
    V: VisitRef<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_external_documentation_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi ExternalDocumentation,
) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_server_ref<'openapi, V>(visitor: &mut V, node: &'openapi Server) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let Server {
        url: _,
        description: _,
        variables,
        extensions,
    } = node;
    let mut outcome = V::Outcome::new();
    if let Some(node) = variables.as_ref() {
        for (_, node) in node {
            let other = visitor.visit_server_variable_ref(node);
            outcome = V::Outcome::reduce(outcome, other);
        }
    }
    let other = visitor.visit_extensions_ref(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_server_variable_ref<'openapi, V>(
    visitor: &mut V,
    node: &'openapi ServerVariable,
) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_info_ref<'openapi, V>(visitor: &mut V, node: &'openapi Info) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
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
    if let Some(node) = contact.as_ref() {
        let other = visitor.visit_contact_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = license.as_ref() {
        let other = visitor.visit_license_ref(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions_ref(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_contact_ref<'openapi, V>(visitor: &mut V, node: &'openapi Contact) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_license_ref<'openapi, V>(visitor: &mut V, node: &'openapi License) -> V::Outcome
where
    V: VisitRef<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}
