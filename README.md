# Overapi

Api over any command line application

## Installation

```bash
git clone https://github.com/navinkarkera/overapi.git
cargo build --release
```

## Starting server

Echo the path of cli app that you want an api over.

```bash
echo "cat" | overapi
```

## API Usage

```bash
curl -H "Content-Type: application/json" -X POST -d '{"arguments":"file.txt", "working_directory":"/some/path", "env_vars":{}}' http://127.0.0.1:8080/
```

## TODO
- [ ] Take cli app as an argument

