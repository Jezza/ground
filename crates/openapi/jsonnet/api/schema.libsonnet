local utils = import "./utils.libsonnet";

{
  local this = self,
  local schema_types = utils.schema_types,
  type(type)::
    local schema = {
      type: type,
    };
    // Quickest way to check it.
    assert utils.is_schema(schema, type);
    schema,

  ext(key, value):: {
    ['x-ground-' + key]: value,
  },

  name(name):: this.ext('name', name),
  external(value=true):: this.ext('external', value),

  deprecated(value=true):: { deprecated: value },

  nullable(value=true)::
    assert std.isBoolean(value) : error 'nullable(value=...) should be a `bool`';
    { nullable: value },

  opt(ty):: this.all_of([
    ty,
  ]) + this.nullable(true),

  nonnull(ty={}):: this.all_of([
    ty,
  ]) + this.nullable(false),

  enum(values, extensible=false)::
    assert std.isArray(values) || std.isObject(values) : 'enum(values=...) must be an array|object';
    assert std.length(values) > 0 : 'enum(values=...) must be a non-empty array|object';
    if std.isObject(values) then
    // We require order, so if someone omits a description field, the descriptions aren't aligned.
    // We use all of the normal (non-doc) fields, and build the extensions from that,
    // this way we can make sure ordering is preserved (we can use defaults in the case of omissions).
      local names = utils.iter(values);

      local docs = std.map(function(field) std.get(values, '#' + field, ""), names);
      local constants = std.map(function(field) values[field], names);

        self.enum(constants, extensible) + {
          // Check if we actually have any doc strings.
          [if std.any(std.map(function(item) std.length(item) > 0, docs)) then 'x-enum-descriptions']+: docs,
          ['x-enum-varnames']+: names,
        }
    else
      local types = std.set(std.map(std.type, values));
      local type = if std.length(types) == 1 then types[0] else
        error 'Heterogeneous arrays not yet supported.';
      //      oneOf: std.map(function(item) { type: item }, types),
      {
        type: type,
        [if extensible then 'x-extensible-enum' else 'enum']+: values,
      },

  integer(format=false, minimum=false, maximum=false):: this.type('integer') + {
    [if std.isString(format) then 'format']: format,
    [if std.isNumber(minimum) then 'minimum']: minimum,
    [if std.isNumber(maximum) then 'maximum']: maximum,
  },
  i32(minimum=false, maximum=false):: this.integer('int32', minimum, maximum),
  i64(minimum=false, maximum=false):: this.integer('int64', minimum, maximum),

  seconds():: this.integer('date-time'),

  string(format=false, min_length=false, max_length=false, pattern=false):: this.type('string') + {
    [if std.isString(format) then 'format']: format,
    [if std.isNumber(min_length) then 'minLength']: min_length,
    [if std.isNumber(max_length) then 'maxLength']: max_length,
    [if std.isString(pattern) then 'pattern']: pattern,
  },
  date():: this.string('date'),
  date_time():: this.string('date-time'),
  password(min_length=false):: this.string('password') + {
    [if std.isNumber(min_length) then 'minLength']: min_length,
  },
  byte():: this.string('byte'),
  binary():: this.string('binary'),
  decimal():: this.string('decimal'),

  regex(regex):: this.string(null) + {
    [if std.isString(regex) then 'pattern']: regex
  },

  email():: this.string('email'),
  uuid():: this.string('uuid'),
  uri():: this.string('uri'),
  hostname():: this.string('hostname'),
  ipv4():: this.string('ipv4'),
  ipv6():: this.string('ipv6'),

  bool():: this.type('boolean'),
  boolean():: this.bool(),

  one_of(types):: {
    oneOf+: types,
  },
  all_of(types):: {
    allOf+: types,
  },
  any_of(types):: {
    anyOf+: types,
  },
  not(type):: {
    not: type,
  },
  extend(child, parent)::
    this.all_of([parent, child]),

  array(items, min_length=null, max_length=null, unique_items=false)::
    this.type('array') + {
      items: items,
      [if std.isNumber(min_length) then 'minItems']: min_length,
      [if std.isNumber(max_length) then 'maxItems']: max_length,
      [if std.isBoolean(unique_items) && unique_items then 'uniqueItems']: true,
    },

  struct(name, properties={}):: this.name(name) + this.object(properties),

  tagged(name, variant_mapping, tag='kind', title=function(variant, schema) schema['x-ground-name'])::
    this.name(name) + this.discriminator(variant_mapping, tag, title),

  discriminator(variant_mapping, tag='kind', title=function(variant, schema) schema['x-ground-name'])::
    local docs = utils.docs(variant_mapping);

    assert std.all(
      std.map(
        function(key)
          local item = variant_mapping[key];
            item.type == 'object' && std.objectHas(item, 'x-ground-name'),
        utils.iter(variant_mapping)
      )
    ) : 'All variants must be an object and have a defined name.';

    local build_variant(tag, variant, schema) =
      local variant_title = title(variant, schema);

      this.all_of([
        schema,
        this.object({
          [tag]: this.enum([variant]),
        })
        + (if std.objectHas(docs, variant) then this.description(docs[variant]) else {})
        + this.title(variant_title),
      ]);

    local variants = [
      build_variant(tag, variant, variant_mapping[variant])
      for variant in utils.iter(variant_mapping)
      if !std.startsWith(variant, '#')
    ];

    // We're assuming that it'll be added later to the this.
    local mapping = {
      [variant]: '#/components/schemas/' + variant_mapping[variant]['x-ground-name'],
      for variant in utils.iter(variant_mapping)
    };

      {
        discriminator+: {
          propertyName: tag,
          mapping+: mapping,
        },
      } + this.one_of(variants),

  object(properties={})::
    local docs = utils.docs(properties);

    local required = utils.iter(properties);
    local all = utils.iter(properties, hidden=true);

    local build_prop(key) =
      local prop = properties[key];

      // We want to explicit mark things that are required as non-nullable and vice-versa.
      local nullable = if !std.member(required, key) && !std.objectHas(prop, 'nullable') then this.nullable(true) else {};

      // The field's description.
      local description = if std.objectHas(docs, key) then this.description(docs[key]) else {};

      local mixin = nullable + description;

      // Do we need to promote the object schema to an allOf
      local schema = if std.length(mixin) > 0 && !std.objectHas(prop, 'allOf') then
        this.all_of([
          prop,
        ])
      else
        prop;

        schema + mixin;

    local props = {
      [key]: build_prop(key)
      for key in all
    };

      this.type('object') + {
        properties+: props,
        [if std.length(required) > 0 then 'required']+: required,
      },

  map(value_type={}, min_properties=null, max_properties=null):: this.type('object') + {
    [if std.isObject(value_type) then 'additionalProperties']: value_type,  // Validate `value_type`
    [if std.isNumber(min_properties) then 'minProperties']: min_properties,
    [if std.isNumber(max_properties) then 'maxProperties']: max_properties,
  },

  ref(path, obj=null)::
    local prefix = if obj != null && std.isObject(obj) && std.get(obj, 'x-ground-external', false) then
      'external:'
    else
      "";
    local name = if obj != null && std.isObject(obj) then
      local name = std.get(obj, 'x-ground-name', error "Type doesn't have an explicit name");
        '/' + name
    else
      "";
    {
      "$ref": prefix + '#/' + path + name,
    },

  any():: {},

  description(value):: { description+: value },
  example(value):: { example: value },
  title(value):: { title: value },
}
