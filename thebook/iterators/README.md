## Iterators

### Consuming adaptors

Methods that call next are called `consuming adaptors`, because calling them uses up the iterator.


### Iterator adaptors

`Iterator adaptors` are methods defined on the `Iterator` trait that don’t consume the iterator.
Instead, they produce different iterators by **changing** some aspect of the original iterator.
