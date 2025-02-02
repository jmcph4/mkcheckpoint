# mkcheckpoint #

`mkcheckpoint` is a program that generates [`amms-rs`](https://github.com/darkforestry/amms-rs)-style checkpoints from CSV data.

## Installation ##

```
$ cargo install mkcheckpoint
```

## Usage ##

```
$ mkcheckpoint -h
A small tool for generating state snapshots of AMM pools

Usage: mkcheckpoint [OPTIONS] <IN> [OUT]

Arguments:
  <IN>   Path to the input CSV file
  [OUT]  Path to write the output checkpoint JSON file to [default: .cfmms-checkpoint.json]

Options:
  -r, --rpc <RPC>  URL to the Ethereum RPC provider [default: https://eth.merkle.io]
  -h, --help       Print help
  -V, --version    Print version
```

```
$ cat a_single_pool.csv
variant,factory,factory_created,pool
UniswapV2,0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f,0,0x0d4a11d5EEaaC28EC3F61d100daF4d40471f1852
$ mkcheckpoint a_single_pool.csv
$ cat .cfmms-checkpoint.json | jq
{
  "timestamp": 1738535405,
  "block_number": 21761814,
  "factories": [
    {
      "UniswapV2Factory": {
        "address": "0x5c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f",
        "creation_block": 0,
        "fee": 300
      }
    }
  ],
  "amms": [
    {
      "UniswapV2Pool": {
        "address": "0x0d4a11d5eeaac28ec3f61d100daf4d40471f1852",
        "token_a": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
        "token_a_decimals": 18,
        "token_b": "0xdac17f958d2ee523a2206206994597c13d831ec7",
        "token_b_decimals": 6,
        "reserve_0": 10999641383329786000000,
        "reserve_1": 31936057989252,
        "fee": 300
      }
    }
  ]
}
```

