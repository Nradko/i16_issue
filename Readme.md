## Bug Report

I have found that use.ink and the contracts-ui.substrate.io have problems decoding returned values from contracts containing i16 type.

To reproduce compile the contract in repo and use it with contracts-ui.substrate.io.

### Current Behavior

#### case1

For the storage:

```
pub struct I16Issue {
    values_i16_tuple: [1,-1],
    values_i16_pair: [1,-1],
}
```

the returned value for getI16Tuples is:

```
[
    [
        '-65,535',
        '-1',
    ],
]
```

the returned value for getI16Pairs is:

```
[
    {
        value1: '-65,535',
        value2: '-1',
    },
]
```

#### case2

For the storage:

```
pub struct I16Issue {
    values_i16_tuple: [0,0],
    values_i16_pair: [0,0],
}
```

the returned value for `getI16Tuples` is:
`Decoding error`

the returned value for `getI16Pairs` is:
`Decoding error`

### Expected Behavior

#### case1

the returned value for `getI16Tuples` is:

```
[
    [
        '1,
        '-1',
    ],
]
```

the returned value for `getI16Pairs` is:

```
[
    {
        value1: '1',
        value2: '-1',
    },
]
```

#### case2

the returned value for `getI16Tuples` is:

```
[
    {
        value1: '0',
        value2: '0',
    },
]
```

the returned value for `getI16Pairs` is:

```
[
    {
        value1: '0',
        value2: '0',
    },
]
```

### Steps To Reproduce

1. copy this repo
2. build with `cargo contract build`
3. deploy on some testnet using https://contracts-ui.substrate.io. (I have used AZERO testnet)

#### case1

4. call `pushI16` with args `value1: 1, value2: -1`
5. call getI16Tuples to get the wrong Return value.

#### case2

4. call `pushI16` with args `value1: 0, value2: -0`
5. call getI16Pair to get the wrong Return value.

### Environment

- rustc 1.69.0 (84c898d65 2023-04-16)
- cargo-contract-contract 3.0.1-unknown-x86_64-unknown-linux-gnu
- Browser: Firefox Browser Developer Edition
- OS: Linux Mint 21
- chain: aleph-testnet
