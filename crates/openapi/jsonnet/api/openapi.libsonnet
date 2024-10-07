local types = import "./types.libsonnet";
local utils = import "./utils.libsonnet";
local schema = import "./schema.libsonnet";

local response = {
  local this = self,

  new(body, description='TODO: response description', headers={}, mixin={})::
    {
      description: description,
      [if body != null && std.length(body) > 0 then 'content']+: {
        // Only support application/json for now
        'application/json'+: {
          schema+: body,
        },
      },
      [if std.length(headers) > 0 then 'headers']+: headers,
    } + mixin,

  header(name, ty, description='TODO: response header description', required=true)::
    local where = 'response header';
    assert utils.is_schema(ty, ['string', 'number', 'integer', 'boolean', 'array']) : '%s type must be string|number|integer|boolean' % [where];
    {
      [name]: {
      schema: ty,
      description: description,
      [if required then 'required']: required,
    },
    },
};

local responses = {
  local this = self,

  new_from_ref(code, ref)::
    {
      responses+: {
        [std.toString(code)]+: ref,
      },
    },

  new(code, body, description='TODO: response description', headers={}, mixin={})::
    local resp = response.new(body, description, headers, mixin);
    this.new_from_ref(code, resp),

  header:: response.header,
};

local known_errors =
  local ty = types.ext.ApiError;
  {
    responses+: {
      default: response.new(ty, importstr "./docs/api_error.md"),
    },
  };

local op = {
  local this = self,

  parameter: {
    new(name, where, type, description='TODO: parameter description')::
      {
        name: name,
        "in": where,
        description: description,
        required: true,
        schema: type,
      },

    schema(ty):: {
      schema: ty,
    },
    schema_mixin(ty):: {
      schema+: ty,
    },
  },

  parameters: {
    // Don't confuse parameters and parameter.
    // This is just a shorthand, as the headers, segments, etc, all create them, and it reads easier this way.
    new(name, where, type, description='TODO: parameter description', mixin={})::
      assert utils.is_schema(type, ['string', 'number', 'integer', 'boolean', 'array']) : '%s type must be string|number|integer|boolean' % [where];
      this.parameters.add(this.parameter.new(name, where, type, description) + mixin),

    add(parameter):: {
      parameters+: [
        parameter,
      ],
    },

    include(parameters):: {
      parameters+: parameters,
    },
  },

  new(id, request_body=null, description='TODO: body description', summary=null):: {
    operationId+: id,
    [if summary != null then 'summary']: summary,
    [if request_body != null then 'requestBody']: {
      description: description,
      content: {
        "application/json": {
          schema: request_body,
        },
      },
    },
  } + known_errors,

  summary(text):: {
    summary: text,
  },
  description(text):: {
    description: text,
  },

  query(name, type, description='TODO: query description', mixin={})::
    this.parameters.new(name, 'query', type, description, mixin),

  segment(name, type, description='TODO: segment description', mixin={})::
    this.parameters.new(name, 'path', type, description, mixin),

  header(name, type, description='TODO: header description', mixin={})::
    this.parameters.new(name, 'header', type, description, mixin),

  //  queries: {
  //    group(name, parameters, mixin_fn=function(prop) {}, ty_fn=function(ty) ty)::
  //      local docs = utils.docs(parameters);
  //
  //      local new_parameter(prop) =
  //        local description = std.get(docs, prop, 'TODO: parameter description');
  //        local ty = parameters[prop];
  //        local is_array = std.get(ty, 'type', null) == 'array';
  //
  //        local underlying_ty = ty_fn(ty);
  //
  //        local mixin = {
  //          required: std.objectHas(parameters, prop),
  //        }
  //        + schema.ext('ground-query-group', name)
  //        + (if is_array then schema.ext('ground-query-array', true) else {})
  //        + mixin_fn(prop)
  //        ;
  //
  //        this.query(
  //          prop,
  //          underlying_ty,
  //          description,
  //          mixin,
  //        ).parameters[0];
  //
  //      this.parameters.include(
  //        std.map(
  //          new_parameter,
  //          utils.iter(parameters, hidden=true, preserve_order=false),
  //        )
  //      ),
  //  },

  responses: responses,

  // func: fn(StatusCode, Response)
  map_response(op, func):: [
    local response = op.responses[code];
    func(code, response)
    for code in std.objectFields(op.responses)
  ],

  map_response_mixin(op, func):: {
    responses+: {
      local response = op.responses[code],
      [code]+: func(code, response)
      for code in std.objectFields(op.responses)
    },
  },
};

local methods = ['get', 'put', 'post', 'delete', 'options', 'head', 'patch', 'trace'];

local path_item = {
  map_ops(item, func):: [
    func(item[method]),
    for method in methods
    if std.objectHas(item, method)
  ],

  map_ops_mixin(item, func):: {
    [method]+: func(item[method]),
    for method in methods
    if std.objectHas(item, method)
  },
};

local route = {
  map_ops(routes, func)::
    std.flattenArrays([
      path_item.map_ops(routes[key], func)
      for key in std.objectFields(routes)
    ]),

  map_ops_mixin(routes, func)::
    {
      [key]+: path_item.map_ops_mixin(routes[key], func)
      for key in std.objectFields(routes)
    },
};

{
  methods: methods,

  path_item: path_item,
  route: route,

  op: op,

  response: response,
  responses: responses,
}