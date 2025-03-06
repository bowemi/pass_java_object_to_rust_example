## Summary

This package provides a basic example of how to use Java to pass a class
instance to a Rust duchess function, then have Rust store that reference
in a background thread that periodically calls it.

## How to Run
```cargo b```

```java -Djava.library.path=target/debug java_test.Test```
