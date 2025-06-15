![
    _/_/_/_/  _/                  _/      _/   
   _/        _/        _/    _/    _/  _/      
  _/_/_/    _/        _/    _/      _/         
 _/        _/        _/    _/    _/  _/        
_/        _/_/_/_/    _/_/_/  _/      _/       
*Wrap. Serve. Flux.*                                           
](./logo.svg)


# FLUX

> **FLUX** = *FLUX Launches Unix Executables*

Expose any CLI program as a secure, authenticated web API using FLUX.

---

## 🚀 Features

- 🔧 **Configure endpoints** that map HTTP requests to CLI arguments and options.
- 🎛️ **Format CLI output** via scripts (e.g. AWK), based on a `format` query param.
- 🔐 **JWT-based auth** with refresh tokens.
- 🧱 **Fast and concurrent** backend built on [Actix-Web](https://actix.rs).
- 💾 **Low-overhead secret storage** 
---

## 📦 Example Use Case

```toml
[endpoint."/ping"]
cli = "/usr/bin/ping"
args = ["-c", "4", "$host"]

[endpoint."/ping".mappings]
"host" = "--host"

[endpoint."/ping".output_formatters]
"json" = "awk -f ./formatters/ping_to_json.awk"
````

Call:

```
curl "http://localhost:8000/ping?host=example.com&format=json" \
  -H "Authorization: Bearer <your-access-token>"
```

---

## 🔧 Configuration

Configuration is managed via TOML files stored in `/config`.

Each endpoint defines:

* `cli`: path to the executable
* `args`: CLI arguments (with placeholders)
* `mappings`: maps HTTP parameters to CLI args
* `output_formatters`: optional format scripts

---

## 🔐 Authentication

FLUX uses **JWT + Refresh Tokens** for access control.

| Endpoint             | Description                   |
| -------------------- | ----------------------------- |
| `POST /auth/login`   | Login (basic or passwordless) |
| `POST /auth/refresh` | Get new access token          |

Access and refresh tokens are signed using HS256 and stored in a TBD
---

## 🛠️ Roadmap

* [ ] Middleware hooks
* [ ] Rate limiting
* [ ] CLI output caching
* [ ] Built-in CLI formatter templates
* [ ] Admin web dashboard

---

## 🧪 Development

Build and run with:

```sh
cargo build
cargo run
```

To run in dev mode with hot-reloading:

```sh
cargo watch -x run
```

---

## 📜 License

**FLUX** is licensed under the [GNU GPL v3](https://www.gnu.org/licenses/gpl-3.0.en.html).

---

## ❤️ Contributions

Pull requests welcome! Please open issues to discuss changes before submitting large patches.

---

Happy fluxing.
