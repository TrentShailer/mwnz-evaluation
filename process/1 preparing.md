# Preparing

## 1 Understanding the problem

Webapps that handles `GET /companies/{id}`, on an incoming request needs to send a GET request to `https://raw.githubusercontent.com/MiddlewareNewZealand/evaluation-instructions/main/xml-api/{id}.xml`, then transforms the XML recieved into JSON and returns it.

## 2 Considerations

- Non-200 responses to GET request, Invalid IDs should 404, other responses should still 404 to conform to spec.
- Malformed XML from GET requests, unlikely but case should still be handled, to conform with spec it should 404.
- I need to consider how testable my implementation will be.
- I need to be able to easily perform HTTP requests.
- I need to be able to eaily parse XML and handle malformed XML.
- How my implementaion should be deployed.

## 3 Language selection

The two languages I am most confident in for webapps is Rust and JS/TS.

### 3.1 Rust

#### Pros

- Great error handling model and ethos in ecosystem - more confidence in correctly handling all errors.
- Single binary - easy to deploy.
- Usually lower resurse usage (RAM & CPU).
- I really enjoy working with it.
- Strict about correctness.
- Libraries almost always have excellent documentation & examples.

#### Cons

- Less flexible.
- Strict about correctness.
- Harder to hire for - Not a concern for this project but still relevant.

### 3.2 JS/TS

#### Pros

- Super flexible - easy to make changes and quicker to get something working.
- Easy to hire for - Not a concern for this project but still relevant.
- Generally less verbose.
- Don't need to compile for a given target.
- Pretty enjoyable to work with.

#### Cons

- Exception model & nulls requites more cognitive overhead.
- Testing is not a language feature.

### 3.3 Choice

A lot of the pros and cons aren't neccecarily very relevant for this specific problem, both languages have libraries to be able to complete the challenge.

It mostly boils down to I am currently enjoying Rust and Rust has (in my opinion) a clear and explicit error model, this means that I can be confident that I have handled all the errors the libraries may return correctly.

## 4 Library seleciton

### Web Application Framework

[Axum](https://docs.rs/axum/latest/axum/), it is what I am used to, very maintained, built by and works on top of the [Tokio](https://docs.rs/tokio/latest/tokio/) async executor library.

### Parsing

[Serde](https://serde.rs/) is the go to parsing library in Rust, very maintained, BYO data format. Designed to go from a format into a rust type & vice versa. Used by Axum.

However, when looking into XML extensions for serde, it appears that XML is a complex spec and mapping directly to a type with serde is difficult.

> Due to the complexity of the XML standard and the fact that Serde was developed with JSON in mind, not all Serde concepts apply smoothly to XML. This leads to that fact that some XML concepts are inexpressible in terms of Serde derives and may require manual deserialization.
> The most notable restriction is the ability to distinguish between elements and attributes, as no other format used by serde has such a conception.
> Due to that the mapping is performed in a best effort manner.
> From <https://docs.rs/quick-xml/latest/quick_xml/de/>

For the XML the app will be parsing this shouldn't be an issue but it something that should be kept in mind.

Most Rust XML librarys support tree like parsing, with [quick-xml](https://docs.rs/quick-xml/latest/quick_xml/index.html) being the most popular, very maintined, and also supporting Serde (de)serialization.

### HTTP Requests

[Reqwest](https://docs.rs/reqwest/latest/reqwest/) is the go to high level HTTP request library, async, very maintained.

### Testing

Rust has testing capabilities built in, unit testing should be no problem, integration testing the web app should follow <https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs>.

### Other libraries

There are a few other minor librarys found in the `Cargo.toml` for things like level based logging.
