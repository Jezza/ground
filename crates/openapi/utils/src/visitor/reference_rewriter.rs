use crate::visitor::{type_visit_mut, visit_mut};
use openapiv3::{
    Callback, Example, Header, Link, Parameter, PathItem, ReferenceOr, RequestBody, Response,
    Schema, SecurityScheme,
};

pub struct ReferenceRewriter<F> {
    func: F,
}

impl<F> ReferenceRewriter<F> {
    pub fn new(func: F) -> Self {
        Self { func }
    }
}

impl<'openapi, F> type_visit_mut::TypeVisitMut<'openapi> for ReferenceRewriter<F>
where
    F: FnMut(&mut String),
{
    type Outcome = ();

    fn visit_reference_or_schema_mut(
        &mut self,
        node: &'openapi mut ReferenceOr<Schema>,
    ) -> Self::Outcome {
        if let ReferenceOr::Reference { reference } = node {
            (self.func)(reference)
        }
        type_visit_mut::visit_reference_or_schema_mut(self, node)
    }

    fn visit_reference_or_box_schema_mut(
        &mut self,
        node: &'openapi mut ReferenceOr<Box<Schema>>,
    ) -> Self::Outcome {
        if let ReferenceOr::Reference { reference } = node {
            (self.func)(reference)
        }
        type_visit_mut::visit_reference_or_box_schema_mut(self, node)
    }
}

impl<'openapi, F> visit_mut::VisitMut<'openapi> for ReferenceRewriter<F>
where
    F: FnMut(&mut String),
{
    fn visit_reference_or_callback_mut(
        &mut self,
        node: &'openapi mut ReferenceOr<Callback>,
    ) -> Self::Outcome {
        if let ReferenceOr::Reference { reference } = node {
            (self.func)(reference)
        }
        visit_mut::visit_reference_or_callback_mut(self, node)
    }
    fn visit_reference_or_security_scheme_mut(
        &mut self,
        node: &'openapi mut ReferenceOr<SecurityScheme>,
    ) -> Self::Outcome {
        if let ReferenceOr::Reference { reference } = node {
            (self.func)(reference)
        }
        visit_mut::visit_reference_or_security_scheme_mut(self, node)
    }
    fn visit_reference_or_path_item_mut(
        &mut self,
        node: &'openapi mut ReferenceOr<PathItem>,
    ) -> Self::Outcome {
        if let ReferenceOr::Reference { reference } = node {
            (self.func)(reference)
        }
        visit_mut::visit_reference_or_path_item_mut(self, node)
    }
    fn visit_reference_or_response_mut(
        &mut self,
        node: &'openapi mut ReferenceOr<Response>,
    ) -> Self::Outcome {
        if let ReferenceOr::Reference { reference } = node {
            (self.func)(reference)
        }
        visit_mut::visit_reference_or_response_mut(self, node)
    }
    fn visit_reference_or_link_mut(
        &mut self,
        node: &'openapi mut ReferenceOr<Link>,
    ) -> Self::Outcome {
        if let ReferenceOr::Reference { reference } = node {
            (self.func)(reference)
        }
        visit_mut::visit_reference_or_link_mut(self, node)
    }
    fn visit_reference_or_request_body_mut(
        &mut self,
        node: &'openapi mut ReferenceOr<RequestBody>,
    ) -> Self::Outcome {
        if let ReferenceOr::Reference { reference } = node {
            (self.func)(reference)
        }
        visit_mut::visit_reference_or_request_body_mut(self, node)
    }
    fn visit_reference_or_parameter_mut(
        &mut self,
        node: &'openapi mut ReferenceOr<Parameter>,
    ) -> Self::Outcome {
        if let ReferenceOr::Reference { reference } = node {
            (self.func)(reference)
        }
        visit_mut::visit_reference_or_parameter_mut(self, node)
    }
    fn visit_reference_or_header_mut(
        &mut self,
        node: &'openapi mut ReferenceOr<Header>,
    ) -> Self::Outcome {
        if let ReferenceOr::Reference { reference } = node {
            (self.func)(reference)
        }
        visit_mut::visit_reference_or_header_mut(self, node)
    }
    fn visit_reference_or_example_mut(
        &mut self,
        node: &'openapi mut ReferenceOr<Example>,
    ) -> Self::Outcome {
        if let ReferenceOr::Reference { reference } = node {
            (self.func)(reference)
        }
        visit_mut::visit_reference_or_example_mut(self, node)
    }
}
