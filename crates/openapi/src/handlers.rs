use crate::error::ApiError;
use http::header::Entry;
use std::collections::HashMap;
use std::str::FromStr;

pub fn handle_raw_path_error(err: axum::extract::rejection::RawPathParamsRejection) -> ApiError {
    use axum::extract::rejection::RawPathParamsRejection;
    match err {
        RawPathParamsRejection::InvalidUtf8InPathParam(err) => {
            // Client sent us invalid UTF-8.
            ApiError::invalid_path_argument(err.body_text(), None)
        }
        RawPathParamsRejection::MissingPathParams(_) => {
            // The route wasn't mounted with path arguments, so it's an internal error.
            ApiError::internal("No paths parameters found for matched route")
        }
        _ => {
            // Non-exhaustive, so we return some generic error message.
            ApiError::internal("Malformed path route.")
        }
    }
}

pub fn handle_json_rejection_error(err: crate::extract::JsonRejectionError) -> ApiError {
    tracing::warn!("{}", err);
    err.into()
}

pub fn parse_segment<T>(value: &str) -> Result<T, ApiError>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    parse(value)
}

pub fn into_header_value<T>(key: &str, value: T) -> Result<http::HeaderValue, ApiError>
where
    T: std::fmt::Display,
{
    let value = value.to_string();
    http::HeaderValue::from_str(&value).map_err(|err| {
        let msg = format!("Unable to convert `{}` into a header value", key);
        tracing::error!("{}: {}", msg, err);
        ApiError::internal(msg)
    })
}

pub fn parse_header<T>(
    headers: &mut http::HeaderMap,
    key: &'static str,
) -> Result<Option<T>, ApiError>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    let Entry::Occupied(entry) = headers.entry(key) else {
        return Ok(None);
    };

    let (_name, mut values) = entry.remove_entry_mult();

    let Some(value) = values.next() else {
        // This shouldn't happen, as the entry would have been vacant.
        return Ok(None);
    };

    if values.next().is_some() {
        return Err(ApiError::invalid_header(format!(
            "Duplicate header: `{}`",
            key
        )));
    }
    let value = value.to_str().map_err(|_| {
        ApiError::invalid_header(format!("Header contains invalid UTF-8: `{}`", key))
    })?;

    let value = parse(value)?;

    Ok(Some(value))
}

// The method relies on that all header parsing removes used keys.
pub fn check_unused_headers(headers: &http::HeaderMap) -> Result<(), ApiError> {
    // There's no error for unknown headers.
    // Although we could consider it, the issue lies in that nginx adds headers to the requests.
    // So we might be rejecting perfectly valid requests from the users because our own proxies
    // are inserting their own headers.
    for key in headers.keys() {
        tracing::trace!("Unexpected header found: {}", key);
    }

    Ok(())
}

pub fn parse<T: FromStr>(value: &str) -> Result<T, ApiError>
where
    <T as FromStr>::Err: std::fmt::Display,
{
    T::from_str(value).map_err(move |err| {
        let msg = err.to_string();
        // @TODO jezza - 07 Oct 2024: Add a TypeName trait
        let type_name = std::any::type_name::<T>();
        ApiError::invalid_value(msg, type_name, String::from(value))
    })
}

pub fn parse_query_array_opt<T>(
    query_map: &mut HashMap<String, Vec<String>>,
    field_name: &'static str,
) -> Result<Vec<T>, ApiError>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    Ok(query_map
        .remove(field_name)
        .unwrap_or_default()
        .into_iter()
        .map(|value| parse(&value))
        .collect::<Result<_, _>>()?)
}

pub fn parse_query_array<T>(
    query_map: &mut HashMap<String, Vec<String>>,
    field_name: &'static str,
) -> Result<Vec<T>, ApiError>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    let values = query_map.remove(field_name).ok_or_else(|| {
        let msg = format!("Query argument `{}` was not provided.", field_name);
        ApiError::missing_query_argument(msg)
    })?;

    if values.is_empty() {
        let msg = format!("Query argument `{}` was not provided.", field_name);
        return Err(ApiError::missing_query_argument(msg));
    }

    Ok(values
        .into_iter()
        .map(|value| parse(&value))
        .collect::<Result<_, _>>()?)
}

pub fn parse_query_opt<T>(
    query_map: &mut HashMap<String, Vec<String>>,
    field_name: &'static str,
) -> Result<Option<T>, ApiError>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    let value = if let Some(values) = query_map.remove(field_name) {
        let mut values = values;
        let value = values.remove(0);
        if !values.is_empty() {
            let msg = format!(
                "Query argument `{}` was provided multiple times.",
                field_name
            );
            return Err(ApiError::invalid_query_argument(msg));
        }
        let value = parse(&value)?;
        Some(value)
    } else {
        None
    };
    Ok(value)
}

pub fn parse_query<T>(
    query_map: &mut HashMap<String, Vec<String>>,
    field_name: &'static str,
) -> Result<T, ApiError>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    let values = query_map.remove(field_name).ok_or_else(|| {
        let msg = format!("Query argument `{}` was not provided.", field_name);
        ApiError::missing_query_argument(msg)
    })?;

    if values.is_empty() {
        let msg = format!("Query argument `{}` was not provided.", field_name);
        return Err(ApiError::missing_query_argument(msg));
    }

    let mut values = values;
    let value = values.remove(0);
    if !values.is_empty() {
        let msg = format!(
            "Query argument `{}` was provided multiple times.",
            field_name
        );
        return Err(ApiError::invalid_query_argument(msg));
    }

    Ok(parse(&value)?)
}

// The method relies on that all parse functions remove used keys.
// pub fn check_unused_parameters(query_map: &HashMap<String, Vec<String>>) -> Result<(), ApiError> {
//     if let Some((key, values)) = query_map.iter().next() {
//         // Check a case where we have 'key>' (our parser eats '=') 'value'.
//         // The original argument is 'key>=value' but should be 'key=>=value' (and same for
//         // 'key<=value').
//         if let Some(value) = values.first() {
//             if (key.ends_with('>') || key.ends_with('<')) && !value.is_empty() {
//                 // most probably the client wants '>=' or '<='
//                 let mut field = key.clone();
//                 let operation = field
//                     .pop()
//                     .expect("key ends with a char, so it must have the char");
//                 let msg = format!(
//                     "Probably malformed inequality operation. Do you mean: {field}={operation}={value}?"
//                 );
//                 return Err(ApiError::invalid_query_argument(msg));
//             }
//         }
//
//         // In case of '>' or '<' we have no '=' in the argument at all, so our parser can't split
//         // it by key-value and the entire argument goes to the key.
//         let res = {
//             let le = key.split_once('<').map(|(k, v)| (k, v, "<"));
//             if le.is_some() {
//                 le
//             } else {
//                 key.split_once('>').map(|(k, v)| (k, v, ">"))
//             }
//         };
//         if let Some((k, v, operation)) = res {
//             let msg = format!(
//                 "Probably malformed inequality operation. Do you mean: {k}={operation}{v}?"
//             );
//             return Err(ApiError::invalid_query_argument(msg));
//         }
//
//         // Ok, let's just report about unused argument.
//         let msg = format!("Invalid query argument: `{}`", key);
//         return Err(ApiError::invalid_query_argument(msg));
//     }
//
//     Ok(())
// }
