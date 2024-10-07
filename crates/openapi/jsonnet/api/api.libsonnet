local utils = import "./utils.libsonnet";
local schema = import "./schema.libsonnet";
local types = import "./types.libsonnet";
local openapi = import "./openapi.libsonnet";

openapi + {
  local this = self,

  ext: types.ext,
  common: types.common,

  types: types,

  schema: schema,

  string_patterns: {
    ExternalReference: '^[A-Za-z0-9-_]{0,50}$',
    Tags: '^[A-Za-z0-9-_:]{0,15}$',
  },

  alias(routes, path):: {
    "$ref": '#/paths/' + std.strReplace(path, '/', '~1'),
  },

  tag(name, description=null)::
    {
      name: name,
      [if description != null then 'description']: description,
    },

  routes(routes)::
    local predicate = function(key, value)
      key != 'models'
      && key != 'filters'
      && !std.startsWith(key, "_")
      && std.length(std.findSubstr('/', key)) == 0;

    utils.flatten(predicate, routes),

  new(name, routes, tags=[])::
    local tag_names = std.map(function(item) item.name, tags);

    local extra = {
      "x-ground-name": name,
      "x-ground-tags": tags,
      "x-ground-external-types": types.external_types,
    };

    // We can include non-route fields to help layering the api.
    local flattened_routes = this.routes(routes);

    std.mapWithKey(
      function(key, item)
        if std.isObject(item) && std.objectHas(item, '$ref') then
          item
        else
          item
          + openapi.path_item.map_ops_mixin(item, function(op) { tags: tag_names }),
      flattened_routes,
    )
    + extra,
}