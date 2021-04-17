# Kafka producer in Rust

This is a simple example of a kafka producer that pushes string messages
to a topic. It's based on this [example](https://github.com/fede1024/rust-rdkafka/blob/master/examples/simple_producer.rs)
from `rdkafka` docs.

To execute it you can use binary directly like this:

```
./producer -b <broker-url> -t <topic>
```

You can also debug the executable using the target *"Debug executable 'producer'"*
on vscode.

### How to consume the data

To consume the data generated from this producer you can
use [this project](https://github.com/DanielKneipp/rust-kafka-consumer).

### Todos:

- [ ] Make usage of exactly once semantics (EOS)
- [ ] Push images (binary data)
