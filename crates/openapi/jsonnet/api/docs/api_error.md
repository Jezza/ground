 * 400 - The server cannot or will not process the request due to something that is perceived to be a client error (for example, malformed request syntax, invalid request message framing, or deceptive request routing). The client should not repeat this request without modification.
 * 401 - The client request has not been completed because it lacks valid authentication credentials for the requested action/resource.
 * 403 - The server understands the request, but refuses to authorize it.
 * 404 - The server cannot find the requested resource.
 * 409 - Indicates a request conflict with the current state of the target resource.
 * 412 - Indicates a conditional request failed to meet its precondition/requirement, and as such, the modification or action cannot be performed.
 * 422 - The server understands the content type of the request entity, and the syntax of the request entity is correct, but it was unable to process the contained instructions. The client should not repeat this request without modification.
 * 429 - The user has sent too many requests in a given amount of time ("rate limiting").
 * 500 - The server encountered an unexpected condition that prevented it from fulfilling the request.
 * 502 - The server, while acting as a gateway or proxy, received an invalid response from the upstream server.
 * 503 - The server is not ready to handle the request.
 