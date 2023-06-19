### GraphQL Server

This is a graphql server based on ntext and juniper.

## Usage

### server
Build on release mode
```bash
cargo build --release
```

Run graphql server
```
./target/release/graphqlserver
```

### Performance test
Using [bombardier](https://github.com/codesenberg/bombardier), execute the following command:

```
./bombardier.exe -m POST -c 100 -n 1000000 -l -p r http://127.0.0.1:8080/graphql -H "Content-Type: application/json" -b '{"query":"{\n  test {\n    id\n  }\n}\n","variables":null}'
```
The command above performs 1000000 request(s) using 100 connection(s)

* -c number of conect
* -n number of requests