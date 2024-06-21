# Post implementation thoughts

## Fetch Company Error Handling

I am not 100% happy with my error handling here as I just bundle all non-200 status codes into an error. However, handling these better would require a lot more code that I decided was out of scope. This does mean the error message for this case is not good though.

## The OpenApi spec

My initial interpretation of the spec was that I shouldn't modify it and that I should only return error codes that are defined in the spec. As a result of this, all error responses were 404s which often didn't match the errors I was handling. Errors like Reqwest having an internal error should 500, etc. While only 404s make it easy for the end user to manage, it felt really wrong.

Along with this Axum returns 400s on invalid parameters, I can change this to a 404 to comply with the spec, but again, it felt wrong.

So I decided that I should expand the spec to include the 400 and 500 error codes, usually this sort of thing should be discussed with team/client but for this case it is fine. Implementing updates was relatively straight forward besides transforming Axum's internal 400 to return an ApiError.

## Same type for XML and JSON

I decided to use the same type for deserialzing from XML and serialzing into JSON. I had thought this would be a larger issue, however, upon testing, deserializing from XML still passes with additional tags as long as `id`, `name`, and `description` are all present. So this should only be an issue if the JSON schema were to update without the XML schema updating. However, if this does happen, there is a bit of work to split the types.

## No HTTPS

In my implementation I didn't include any TLS capabilities. For most of my personal project I usually just use NGINX infront of my services to handle TLS. However, I have very limited experience around how this is best handled in real-world cases.
