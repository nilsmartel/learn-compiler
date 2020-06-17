# learn-compiler

## Language

The Language is minimal and statiically typed. Ment to be compiled ahead of time.



### Syntax
Here's a sample:
```typescript
function fibonacci(n: uint64) -> uint64 {
    if n == 0 {
        return 0
    }

    if n == 1 {
        return 0
    }

    return fibonacci(n-1) + fibonacci(n-2)
}

function main() {
    fibonacci(42)
}
```

### Types
here's a list of build in types

    bool
    int64
    uint64
    float64
