# Sonic Channel

Python client for [sonic] search backend.

We recommend you start with the [documentation].


## Installation

```bash
pip install sonic
```


## Example usage

### Search channel

Note: This example requires enabling the `search` feature, enabled by default.

```python
from sonic import IngestChannel, SearchChannel, ControlChannel

querycl = SearchChannel("localhost:1491", "SecretPassword")
print(querycl.ping())
print(querycl.query("wiki", "articles", "for"))
print(querycl.query("wiki", "articles", "love"))
print(querycl.suggest("wiki", "articles", "hell"))
```

### Ingest channel

Note: This example requires enabling the `ingest` feature.

```python
from sonic import IngestChannel, SearchChannel, ControlChannel

ingestcl = IngestChannel("localhost:1491", "SecretPassword")
print(ingestcl.ping())
ingestcl.push("wiki", "articles", "article-1", "for the love of god hell")
ingestcl.push("wiki", "articles", "article-2", "for the love of satan heaven")
ingestcl.push("wiki", "articles", "article-3", "for the love of lorde hello")
ingestcl.push("wiki", "articles", "article-4", "for the god of loaf helmet")
```

### Control channel

Note: This example requires enabling the `control` feature.

```python
from sonic import IngestChannel, SearchChannel, ControlChannel

controlcl = ControlChannel("localhost:1491", "SecretPassword")
print(controlcl.ping())
print(controlcl.consolidate())
```


## Available features

* **default** - ["search", "ingest", "control"]
* **search** - Add sonic search mode with methods
* **ingest** - Add sonic ignite mode with methods
* **control** - Add sonic control mode with methods


[sonic]: https://github.com/valeriansaliou/sonic
[documentation]: https://docs.rs/sonic-channel