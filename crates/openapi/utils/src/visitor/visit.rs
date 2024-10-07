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

pub trait Visit: super::type_visit::TypeVisit {
    fn visit_openapi(&mut self, node: OpenAPI) -> Self::Outcome {
        visit_openapi(self, node)
    }
    fn visit_tag(&mut self, node: Tag) -> Self::Outcome {
        visit_tag(self, node)
    }
    fn visit_components(&mut self, node: Components) -> Self::Outcome {
        visit_components(self, node)
    }
    fn visit_reference_or_callback(&mut self, node: ReferenceOr<Callback>) -> Self::Outcome {
        visit_reference_or_callback(self, node)
    }
    fn visit_callback(&mut self, node: Callback) -> Self::Outcome {
        visit_callback(self, node)
    }
    fn visit_reference_or_security_scheme(
        &mut self,
        node: ReferenceOr<SecurityScheme>,
    ) -> Self::Outcome {
        visit_reference_or_security_scheme(self, node)
    }
    fn visit_security_scheme(&mut self, node: SecurityScheme) -> Self::Outcome {
        visit_security_scheme(self, node)
    }
    fn visit_oauth2_flows(&mut self, node: OAuth2Flows) -> Self::Outcome {
        visit_oauth2_flows(self, node)
    }
    fn visit_oauth2_client_credential_flow(
        &mut self,
        node: ClientCredentialsOAuth2Flow,
    ) -> Self::Outcome {
        visit_oauth2_client_credential_flow(self, node)
    }
    fn visit_oauth2_authorization_code_flow(
        &mut self,
        node: AuthorizationCodeOAuth2Flow,
    ) -> Self::Outcome {
        visit_oauth2_authorization_code_flow(self, node)
    }
    fn visit_oauth2_implicit_flow(&mut self, node: ImplicitOAuth2Flow) -> Self::Outcome {
        visit_oauth2_implicit_flow(self, node)
    }
    fn visit_oauth2_password_flow(&mut self, node: PasswordOAuth2Flow) -> Self::Outcome {
        visit_oauth2_password_flow(self, node)
    }
    fn visit_api_key_location(&mut self, node: APIKeyLocation) -> Self::Outcome {
        visit_api_key_location(self, node)
    }
    fn visit_paths(&mut self, node: Paths) -> Self::Outcome {
        visit_paths(self, node)
    }
    fn visit_reference_or_path_item(&mut self, node: ReferenceOr<PathItem>) -> Self::Outcome {
        visit_reference_or_path_item(self, node)
    }
    fn visit_path_item(&mut self, node: PathItem) -> Self::Outcome {
        visit_path_item(self, node)
    }
    fn visit_operation(&mut self, node: Operation) -> Self::Outcome {
        visit_operation(self, node)
    }
    fn visit_responses(&mut self, node: Responses) -> Self::Outcome {
        visit_responses(self, node)
    }
    fn visit_reference_or_response(&mut self, node: ReferenceOr<Response>) -> Self::Outcome {
        visit_reference_or_response(self, node)
    }
    fn visit_response(&mut self, node: Response) -> Self::Outcome {
        visit_response(self, node)
    }
    fn visit_reference_or_link(&mut self, node: ReferenceOr<Link>) -> Self::Outcome {
        visit_reference_or_link(self, node)
    }
    fn visit_link(&mut self, node: Link) -> Self::Outcome {
        visit_link(self, node)
    }
    fn visit_link_operation(&mut self, node: LinkOperation) -> Self::Outcome {
        visit_link_operation(self, node)
    }
    fn visit_reference_or_request_body(&mut self, node: ReferenceOr<RequestBody>) -> Self::Outcome {
        visit_reference_or_request_body(self, node)
    }
    fn visit_request_body(&mut self, node: RequestBody) -> Self::Outcome {
        visit_request_body(self, node)
    }
    fn visit_reference_or_parameter(&mut self, node: ReferenceOr<Parameter>) -> Self::Outcome {
        visit_reference_or_parameter(self, node)
    }
    fn visit_parameter(&mut self, node: Parameter) -> Self::Outcome {
        visit_parameter(self, node)
    }
    fn visit_cookie_style(&mut self, node: CookieStyle) -> Self::Outcome {
        visit_cookie_style(self, node)
    }
    fn visit_path_style(&mut self, node: PathStyle) -> Self::Outcome {
        visit_path_style(self, node)
    }
    fn visit_parameter_data(&mut self, node: ParameterData) -> Self::Outcome {
        visit_parameter_data(self, node)
    }
    fn visit_parameter_schema_or_content(
        &mut self,
        node: ParameterSchemaOrContent,
    ) -> Self::Outcome {
        visit_parameter_schema_or_content(self, node)
    }
    fn visit_media_type(&mut self, node: MediaType) -> Self::Outcome {
        visit_media_type(self, node)
    }
    fn visit_encoding(&mut self, node: Encoding) -> Self::Outcome {
        visit_encoding(self, node)
    }
    fn visit_query_style(&mut self, node: QueryStyle) -> Self::Outcome {
        visit_query_style(self, node)
    }
    fn visit_reference_or_header(&mut self, node: ReferenceOr<Header>) -> Self::Outcome {
        visit_reference_or_header(self, node)
    }
    fn visit_header(&mut self, node: Header) -> Self::Outcome {
        visit_header(self, node)
    }
    fn visit_header_style(&mut self, node: HeaderStyle) -> Self::Outcome {
        visit_header_style(self, node)
    }
    fn visit_reference_or_example(&mut self, node: ReferenceOr<Example>) -> Self::Outcome {
        visit_reference_or_example(self, node)
    }
    fn visit_example(&mut self, node: Example) -> Self::Outcome {
        visit_example(self, node)
    }
    fn visit_server(&mut self, node: Server) -> Self::Outcome {
        visit_server(self, node)
    }
    fn visit_server_variable(&mut self, node: ServerVariable) -> Self::Outcome {
        visit_server_variable(self, node)
    }
    fn visit_info(&mut self, node: Info) -> Self::Outcome {
        visit_info(self, node)
    }
    fn visit_contact(&mut self, node: Contact) -> Self::Outcome {
        visit_contact(self, node)
    }
    fn visit_license(&mut self, node: License) -> Self::Outcome {
        visit_license(self, node)
    }
}

pub fn visit_openapi<V>(visitor: &mut V, node: OpenAPI) -> V::Outcome
where
    V: Visit + ?Sized,
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
    let mut outcome = visitor.visit_info(info);
    for node in servers {
        let other = visitor.visit_server(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_paths(paths);
    outcome = V::Outcome::reduce(outcome, other);
    if let Some(node) = components {
        let other = visitor.visit_components(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for node in tags {
        let other = visitor.visit_tag(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = external_docs {
        let other = visitor.visit_external_documentation(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_tag<V>(visitor: &mut V, node: Tag) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let Tag {
        name: _,
        description: _,
        external_docs,
        extensions,
    } = node;
    let mut outcome = V::Outcome::new();
    if let Some(node) = external_docs {
        let other = visitor.visit_external_documentation(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_components<V>(visitor: &mut V, node: Components) -> V::Outcome
where
    V: Visit + ?Sized,
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
        let other = visitor.visit_reference_or_security_scheme(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in responses {
        let other = visitor.visit_reference_or_response(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in parameters {
        let other = visitor.visit_reference_or_parameter(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in examples {
        let other = visitor.visit_reference_or_example(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in request_bodies {
        let other = visitor.visit_reference_or_request_body(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in headers {
        let other = visitor.visit_reference_or_header(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in schemas {
        let other = visitor.visit_reference_or_schema(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in links {
        let other = visitor.visit_reference_or_link(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in callbacks {
        let other = visitor.visit_reference_or_callback(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_reference_or_callback<V>(visitor: &mut V, node: ReferenceOr<Callback>) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_callback(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_callback<V>(visitor: &mut V, node: Callback) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let mut outcome = V::Outcome::new();
    for (_, node) in node {
        let other = visitor.visit_path_item(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_reference_or_security_scheme<V>(
    visitor: &mut V,
    node: ReferenceOr<SecurityScheme>,
) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_security_scheme(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_security_scheme<V>(visitor: &mut V, node: SecurityScheme) -> V::Outcome
where
    V: Visit + ?Sized,
{
    match node {
        SecurityScheme::APIKey {
            location,
            name: _,
            description: _,
            extensions,
        } => {
            let outcome = visitor.visit_api_key_location(location);
            let other = visitor.visit_extensions(extensions);
            V::Outcome::reduce(outcome, other)
        }
        SecurityScheme::HTTP {
            scheme: _,
            bearer_format: _,
            description: _,
            extensions,
        } => visitor.visit_extensions(extensions),
        SecurityScheme::OAuth2 {
            flows,
            description: _,
            extensions,
        } => {
            let outcome = visitor.visit_oauth2_flows(flows);
            let other = visitor.visit_extensions(extensions);
            V::Outcome::reduce(outcome, other)
        }
        SecurityScheme::OpenIDConnect {
            open_id_connect_url: _,
            description: _,
            extensions,
        } => visitor.visit_extensions(extensions),
    }
}

pub fn visit_oauth2_flows<V>(visitor: &mut V, node: OAuth2Flows) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let OAuth2Flows {
        implicit,
        password,
        client_credentials,
        authorization_code,
        extensions,
    } = node;
    let mut outcome = V::Outcome::new();
    if let Some(node) = implicit {
        let other = visitor.visit_oauth2_implicit_flow(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = password {
        let other = visitor.visit_oauth2_password_flow(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = client_credentials {
        let other = visitor.visit_oauth2_client_credential_flow(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = authorization_code {
        let other = visitor.visit_oauth2_authorization_code_flow(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_oauth2_client_credential_flow<V>(
    visitor: &mut V,
    node: ClientCredentialsOAuth2Flow,
) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_oauth2_authorization_code_flow<V>(
    visitor: &mut V,
    node: AuthorizationCodeOAuth2Flow,
) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_oauth2_implicit_flow<V>(visitor: &mut V, node: ImplicitOAuth2Flow) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_oauth2_password_flow<V>(visitor: &mut V, node: PasswordOAuth2Flow) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_api_key_location<V>(visitor: &mut V, node: APIKeyLocation) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_paths<V>(visitor: &mut V, node: Paths) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let Paths { paths, extensions } = node;
    let mut outcome = V::Outcome::new();
    for (_, node) in paths {
        let other = visitor.visit_reference_or_path_item(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_reference_or_path_item<V>(visitor: &mut V, node: ReferenceOr<PathItem>) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_path_item(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_path_item<V>(visitor: &mut V, node: PathItem) -> V::Outcome
where
    V: Visit + ?Sized,
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
    if let Some(node) = get {
        let other = visitor.visit_operation(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = put {
        let other = visitor.visit_operation(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = post {
        let other = visitor.visit_operation(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = delete {
        let other = visitor.visit_operation(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = options {
        let other = visitor.visit_operation(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = head {
        let other = visitor.visit_operation(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = patch {
        let other = visitor.visit_operation(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = trace {
        let other = visitor.visit_operation(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for node in servers {
        let other = visitor.visit_server(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for node in parameters {
        let other = visitor.visit_reference_or_parameter(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_operation<V>(visitor: &mut V, node: Operation) -> V::Outcome
where
    V: Visit + ?Sized,
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
    if let Some(node) = external_docs {
        let other = visitor.visit_external_documentation(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for node in parameters {
        let other = visitor.visit_reference_or_parameter(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = request_body {
        let other = visitor.visit_reference_or_request_body(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_responses(responses);
    outcome = V::Outcome::reduce(outcome, other);
    for (_, node) in callbacks {
        let other = visitor.visit_callback(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for node in servers {
        let other = visitor.visit_server(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_responses<V>(visitor: &mut V, node: Responses) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let Responses {
        default,
        responses,
        extensions,
    } = node;
    let mut outcome = V::Outcome::new();
    if let Some(node) = default {
        let other = visitor.visit_reference_or_response(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in responses {
        let other = visitor.visit_reference_or_response(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_reference_or_response<V>(visitor: &mut V, node: ReferenceOr<Response>) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_response(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_response<V>(visitor: &mut V, node: Response) -> V::Outcome
where
    V: Visit + ?Sized,
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
        let other = visitor.visit_reference_or_header(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in content {
        let other = visitor.visit_media_type(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in links {
        let other = visitor.visit_reference_or_link(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_reference_or_link<V>(visitor: &mut V, node: ReferenceOr<Link>) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_link(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_link<V>(visitor: &mut V, node: Link) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let Link {
        description: _,
        operation,
        request_body: _,
        parameters: _,
        server,
        extensions,
    } = node;
    let mut outcome = visitor.visit_link_operation(operation);
    if let Some(node) = server {
        let other = visitor.visit_server(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_link_operation<V>(visitor: &mut V, node: LinkOperation) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_reference_or_request_body<V>(
    visitor: &mut V,
    node: ReferenceOr<RequestBody>,
) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_request_body(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_request_body<V>(visitor: &mut V, node: RequestBody) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let RequestBody {
        description: _,
        content,
        required: _,
        extensions,
    } = node;
    let mut outcome = V::Outcome::new();
    for (_, node) in content {
        let other = visitor.visit_media_type(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_reference_or_parameter<V>(visitor: &mut V, node: ReferenceOr<Parameter>) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_parameter(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_parameter<V>(visitor: &mut V, node: Parameter) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let mut outcome = V::Outcome::new();
    match node {
        Parameter::Query {
            parameter_data,
            allow_reserved: _,
            style,
            allow_empty_value: _,
        } => {
            let other = visitor.visit_parameter_data(parameter_data);
            outcome = V::Outcome::reduce(outcome, other);
            let other = visitor.visit_query_style(style);
            outcome = V::Outcome::reduce(outcome, other);
        }
        Parameter::Header {
            parameter_data,
            style,
        } => {
            let other = visitor.visit_parameter_data(parameter_data);
            outcome = V::Outcome::reduce(outcome, other);
            let other = visitor.visit_header_style(style);
            outcome = V::Outcome::reduce(outcome, other);
        }
        Parameter::Path {
            parameter_data,
            style,
        } => {
            let other = visitor.visit_parameter_data(parameter_data);
            outcome = V::Outcome::reduce(outcome, other);
            let other = visitor.visit_path_style(style);
            outcome = V::Outcome::reduce(outcome, other);
        }
        Parameter::Cookie {
            parameter_data,
            style,
        } => {
            let other = visitor.visit_parameter_data(parameter_data);
            outcome = V::Outcome::reduce(outcome, other);
            let other = visitor.visit_cookie_style(style);
            outcome = V::Outcome::reduce(outcome, other);
        }
    }
    outcome
}

pub fn visit_cookie_style<V>(visitor: &mut V, node: CookieStyle) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_path_style<V>(visitor: &mut V, node: PathStyle) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_parameter_data<V>(visitor: &mut V, node: ParameterData) -> V::Outcome
where
    V: Visit + ?Sized,
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
    let other = visitor.visit_parameter_schema_or_content(format);
    outcome = V::Outcome::reduce(outcome, other);
    for (_, node) in examples {
        let other = visitor.visit_reference_or_example(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_parameter_schema_or_content<V>(
    visitor: &mut V,
    node: ParameterSchemaOrContent,
) -> V::Outcome
where
    V: Visit + ?Sized,
{
    match node {
        ParameterSchemaOrContent::Schema(node) => visitor.visit_reference_or_schema(node),
        ParameterSchemaOrContent::Content(node) => {
            let mut outcome = V::Outcome::new();
            for (_, node) in node {
                let other = visitor.visit_media_type(node);
                outcome = V::Outcome::reduce(outcome, other);
            }
            outcome
        }
    }
}

pub fn visit_media_type<V>(visitor: &mut V, node: MediaType) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let MediaType {
        schema,
        example: _,
        examples,
        encoding,
        extensions,
    } = node;
    let mut outcome = V::Outcome::new();
    if let Some(node) = schema {
        let other = visitor.visit_reference_or_schema(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in examples {
        let other = visitor.visit_reference_or_example(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    for (_, node) in encoding {
        let other = visitor.visit_encoding(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_encoding<V>(visitor: &mut V, node: Encoding) -> V::Outcome
where
    V: Visit + ?Sized,
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
        let other = visitor.visit_reference_or_header(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = style {
        let other = visitor.visit_query_style(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_query_style<V>(visitor: &mut V, node: QueryStyle) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_reference_or_header<V>(visitor: &mut V, node: ReferenceOr<Header>) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_header(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_header<V>(visitor: &mut V, node: Header) -> V::Outcome
where
    V: Visit + ?Sized,
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
    let mut outcome = visitor.visit_header_style(style);
    let other = visitor.visit_parameter_schema_or_content(format);
    outcome = V::Outcome::reduce(outcome, other);
    for (_, node) in examples {
        let other = visitor.visit_reference_or_example(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_header_style<V>(visitor: &mut V, node: HeaderStyle) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_reference_or_example<V>(visitor: &mut V, node: ReferenceOr<Example>) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let mut outcome = V::Outcome::new();
    if let Item(node) = node {
        let other = visitor.visit_example(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    outcome
}

pub fn visit_example<V>(visitor: &mut V, node: Example) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_reference_or_schema<V>(visitor: &mut V, node: ReferenceOr<Schema>) -> V::Outcome
where
    V: Visit + ?Sized,
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
    V: Visit + ?Sized,
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
    V: Visit + ?Sized,
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
    V: Visit + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_schema_kind<V>(visitor: &mut V, node: SchemaKind) -> V::Outcome
where
    V: Visit + ?Sized,
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
    V: Visit + ?Sized,
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
    V: Visit + ?Sized,
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
    V: Visit + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_array_type<V>(visitor: &mut V, node: ArrayType) -> V::Outcome
where
    V: Visit + ?Sized,
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
    V: Visit + ?Sized,
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
    V: Visit + ?Sized,
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
    V: Visit + ?Sized,
{
    match node {
        AdditionalProperties::Any(_) => V::Outcome::new(),
        AdditionalProperties::Schema(node) => visitor.visit_reference_or_schema(*(node)),
    }
}

pub fn visit_integer_type<V>(visitor: &mut V, node: IntegerType) -> V::Outcome
where
    V: Visit + ?Sized,
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
    V: Visit + ?Sized,
{
    match node {
        VariantOrUnknownOrEmpty::Item(node) => visitor.visit_integer_format(node),
        VariantOrUnknownOrEmpty::Unknown(_) => V::Outcome::new(),
        VariantOrUnknownOrEmpty::Empty => V::Outcome::new(),
    }
}

pub fn visit_integer_format<V>(visitor: &mut V, node: IntegerFormat) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_number_type<V>(visitor: &mut V, node: NumberType) -> V::Outcome
where
    V: Visit + ?Sized,
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
    V: Visit + ?Sized,
{
    match node {
        VariantOrUnknownOrEmpty::Item(node) => visitor.visit_number_format(node),
        VariantOrUnknownOrEmpty::Unknown(_) => V::Outcome::new(),
        VariantOrUnknownOrEmpty::Empty => V::Outcome::new(),
    }
}

pub fn visit_number_format<V>(visitor: &mut V, node: NumberFormat) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_string_type<V>(visitor: &mut V, node: StringType) -> V::Outcome
where
    V: Visit + ?Sized,
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
    V: Visit + ?Sized,
{
    match node {
        VariantOrUnknownOrEmpty::Item(node) => visitor.visit_string_format(node),
        VariantOrUnknownOrEmpty::Unknown(_) => V::Outcome::new(),
        VariantOrUnknownOrEmpty::Empty => V::Outcome::new(),
    }
}

pub fn visit_string_format<V>(visitor: &mut V, node: StringFormat) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_external_documentation<V>(visitor: &mut V, node: ExternalDocumentation) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_server<V>(visitor: &mut V, node: Server) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let Server {
        url: _,
        description: _,
        variables,
        extensions,
    } = node;
    let mut outcome = V::Outcome::new();
    if let Some(node) = variables {
        for (_, node) in node {
            let other = visitor.visit_server_variable(node);
            outcome = V::Outcome::reduce(outcome, other);
        }
    }
    let other = visitor.visit_extensions(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_server_variable<V>(visitor: &mut V, node: ServerVariable) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_info<V>(visitor: &mut V, node: Info) -> V::Outcome
where
    V: Visit + ?Sized,
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
    if let Some(node) = contact {
        let other = visitor.visit_contact(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    if let Some(node) = license {
        let other = visitor.visit_license(node);
        outcome = V::Outcome::reduce(outcome, other);
    }
    let other = visitor.visit_extensions(extensions);
    V::Outcome::reduce(outcome, other)
}

pub fn visit_contact<V>(visitor: &mut V, node: Contact) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}

pub fn visit_license<V>(visitor: &mut V, node: License) -> V::Outcome
where
    V: Visit + ?Sized,
{
    let (_, _) = (visitor, node);
    V::Outcome::new()
}
