local iter(obj, hidden=false, preserve_order=true) =
  std.filter(
    function(prop) !std.startsWith(prop, '#'),
    std.objectFieldsEx(obj, hidden=hidden, preserve_order=preserve_order)
  );

// Accepts a predicate and object to recursive flatten those fields into the main object.
// The predicate should return true if the object should be flattened.
// predicate: (key: string, value: any) -> bool,
local flatten(predicate, obj, hidden=false) =
  local object_flat_map(item, first) =
    std.flattenArrays([
      local value = item[key];
      if predicate(key, value) then
        object_flat_map(value, false)
      else if first then
      // We don't want to replace the first layer, as it affects `$` resolution.
      // We append it with the original object to maintain that relation.
        []
      else
        [
          {
            key: key,
            value: value,
          },
        ]
      for key in std.objectFieldsEx(item, hidden=hidden, preserve_order=true)
    ]);
  local entries = object_flat_map(obj, true);
    {
      [entry.key]: entry.value
      for entry in entries
    } + obj;

local docs(properties) =
  {
    [std.substr(prop, 1, std.length(prop))]: properties[prop]
    for prop in std.objectFieldsEx(properties, hidden=true, preserve_order=true)
    if std.startsWith(prop, '#')
  };

local schema_types = [
  'string',
  'number',
  'integer',
  'boolean',
  'array',
  'object',
];

local is_schema(item, type='object') =
  if std.isArray(type) then
    local unknown = [
      ty
      for ty in type
      if !std.member(schema_types, ty)
    ];
    assert std.length(unknown) == 0 : 'is_schema(type=[%s]) must be member(s) of %s.' % [std.join(',', unknown), std.join(',', schema_types)];
    std.isObject(item) && std.member(type, std.get(item, 'type', null))
  else
    assert std.member(schema_types, type) : "is_schema(type='%s') must be a member of %s." % [type, std.join(',', schema_types)];
    std.isObject(item) && std.get(item, 'type', null) == type;

local names(obj) =
  if std.isObject(obj) then
    [
      val['x-ground-name']
      for val in std.objectValues(obj)
      if std.objectHas(val, 'x-ground-name')
    ]
  else if std.isArray(obj) then
    [
      val['x-ground-name']
      for val in obj
      if std.objectHas(val, 'x-ground-name')
    ]
  else
    error 'names(obj=...) must be `object|array`';

{
  names:: names,
  flatten:: flatten,

  docs:: docs,
  iter:: iter,

  schema_types: schema_types,
  is_schema:: is_schema,

  id(item):: item,
}
