# Implementing

1. Start by declaring the relevant types for the project.
2. Implement a function to take a string of supposidly XML and return the internal 'Company' type or an error (I may need to add `thiserror` to the libraries if I want easy union error types).
3. Tests for above function.
4. Implement a function to take an ID and try get the XML from github returning errors. Maybe do some error preproccessing here?
5. Test above function.
6. Implement Axum boilerplate to create webserver.
7. Implement GET method that calls the two functions above, does the error processing, and returns the result.

I've decided to not couple the parsing of the XML and the getting of the XML just to keep the two operations separate and individually testable, I may decide to change this later if I think it'll work better together.

The GET method should be able to handle turning the internal 'Company' type into JSON due to Axum.
