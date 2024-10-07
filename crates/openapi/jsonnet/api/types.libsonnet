local schema = import "./schema.libsonnet";

local errors = {
  local this = self,

  MissingField: schema.struct('error.MissingField', {
    obj_ty: schema.string(),
    field: schema.string(),
  }) + schema.external(),

  UnknownField: schema.struct('error.UnknownField', {
    field_type: schema.string(),
    field: schema.string(),
  }) + schema.external(),

  UnknownVariant: schema.struct('error.UnknownVariant', {
    variant_type: schema.string(),
    variant: schema.string(),
  }) + schema.external(),

  InvalidType: schema.struct('error.InvalidType', {
    expected: schema.string(),
    value: schema.string(),
  }) + schema.external(),

  InvalidValue: schema.struct('error.InvalidValue', {
    value_type: schema.string(),
    value: schema.string(),
  }) + schema.external(),

  MissingQueryArgument: schema.struct('error.MissingQueryArgument') + schema.external(),
  InvalidQueryArgument: schema.struct('error.InvalidQueryArgument') + schema.external(),
  MissingHeader: schema.struct('error.MissingHeader') + schema.external(),
  InvalidHeader: schema.struct('error.InvalidHeader') + schema.external(),
  MissingPathArgument: schema.struct('error.MissingPathArgument') + schema.external(),
  InvalidPathArgument: schema.struct('error.InvalidPathArgument', {
    key: schema.string(),
  }) + schema.external(),

  NotFound: schema.struct('error.NotFound', {
    entity: schema.string(),
    value:: schema.string(),
  }) + schema.external(),

  Internal: schema.struct('error.Internal') + schema.external(),
  Unauthorized: schema.struct('error.Unauthorized') + schema.external(),
  FailedPrecondition: schema.struct('error.FailedPrecondition') + schema.external(),
  BadGateway: schema.struct('error.BadGateway') + schema.external(),
  ServiceUnavailable: schema.struct('error.ServiceUnavailable') + schema.external(),
  UnprocessableEntity: schema.struct('error.UnprocessableEntity') + schema.external(),
  Forbidden: schema.struct('error.Forbidden') + schema.external(),
  Conflict: schema.struct('error.Conflict') + schema.external(),
  NotImplemented: schema.struct('error.NotImplemented') + schema.external(),
  BadRequest: schema.struct('error.BadRequest') + schema.external(),
  MisdirectedRequest: schema.struct('error.MisdirectedRequest') + schema.external(),

  Reason: schema.tagged('error.ApiErrorReason', {
    MISSING_FIELD: this.MissingField,
    UNKNOWN_FIELD: this.UnknownField,
    UNKNOWN_VARIANT: this.UnknownVariant,
    INVALID_TYPE: this.InvalidType,
    INVALID_VALUE: this.InvalidValue,
    MISSING_QUERY_ARGUMENT: this.MissingQueryArgument,
    INVALID_QUERY_ARGUMENT: this.InvalidQueryArgument,
    MISSING_HEADER: this.MissingHeader,
    INVALID_HEADER: this.InvalidHeader,
    MISSING_PATH_ARGUMENT: this.MissingPathArgument,
    INVALID_PATH_ARGUMENT: this.InvalidPathArgument,
    NOT_FOUND: this.NotFound,
    INTERNAL: this.Internal,
    UNAUTHORIZED: this.Unauthorized,
    FAILED_PRECONDITION: this.FailedPrecondition,
    BAD_GATEWAY: this.BadGateway,
    SERVICE_UNAVAILABLE: this.ServiceUnavailable,
    UNPROCESSABLE_ENTITY: this.UnprocessableEntity,
    FORBIDDEN: this.Forbidden,
    CONFLICT: this.Conflict,
    NOT_IMPLEMENTED: this.NotImplemented,
    BAD_REQUEST: this.BadRequest,
    MISDIRECTED_REQUEST: this.MisdirectedRequest,
  }, tag='type') + schema.external(),
};

local externals = {
  local this = self,

  ApiError: schema.struct('error.ApiError', {
    location:: schema.string(),
    message: schema.string(),
    reason: errors.Reason,
  }) + schema.external(),
};

local into_references(table, filter=[]) = {
  [key]: if 'x-ground-name' in table[key] && (std.length(filter) == 0 || std.member(filter, key)) then
  schema.ref('components/schemas', table[key])
  + (if 'type' in table[key] then { type: table[key].type } else {})
  + { "x-ground-name": table[key]['x-ground-name'] }
else
  table[key],
  for key in std.objectFields(table)
};

// Exported via api.libsonnet
{
  ext: into_references(externals),
  // This is used by the api generator in order to document the underlying external types.
  external_types: externals,
}
