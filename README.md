# HTTP server implementation in Rust
The purpose of this project was to develop a toy implementation of a HTTP server to learn Rust features. It is partially inspired by the tutorial written in [The Rust Book](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html). However, some additional features were developped (MIME type handler, etc). The server handles GET request, reads html files as well as png or jpg. 


## Example

### Running the server on http://127.0.0.1:5666 

Move to `example` folder and run the server:

```
cd example
cargo run --package http-server --bin http-server 127.0.0.1:5666
```

Then in a web browser, type the following URL: http://127.0.0.1:5666/hello.html. A simple HTML page should be displayed. 

## Authors

* **Romain Desarzens** - *Initial work* - [rdesarz](https://github.com/rdesarz)

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
