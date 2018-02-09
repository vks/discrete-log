# discrete-log

[![Build Status](https://travis-ci.org/vks/discrete-log.svg?branch=master)](https://travis-ci.org/vks/discrete-log)

[Discrete logarithm](https://en.wikipedia.org/wiki/Discrete_logarithm) implementation in Rust that can serve as a benchmark for bigints.

## Building

Make sure GMP is installed. Then `cargo build` should work.

## Example

You can use this to compare the performance of the pure-Rust bigint
implementation in [`num`](https://crates.io/crates/num) with
[GMP](https://gmplib.org/):

```
► cargo build --release
[...]
► perf stat -B cargo run --release num_bigint
     Running `target/release/discrete-log num_bigint`
375374217830

 Performance counter stats for 'cargo run --release num_bigint':

       5088.166488      task-clock (msec)         #    0.968 CPUs utilized          
               609      context-switches          #    0.120 K/sec                  
                16      cpu-migrations            #    0.003 K/sec                  
            47,703      page-faults               #    0.009 M/sec                  
    19,700,825,207      cycles                    #    3.872 GHz                    
   <not supported>      stalled-cycles-frontend  
   <not supported>      stalled-cycles-backend   
    53,075,970,364      instructions              #    2.69  insns per cycle        
     9,602,794,480      branches                  # 1887.280 M/sec                  
        14,076,794      branch-misses             #    0.15% of all branches        

       5.255022572 seconds time elapsed

► perf stat -B cargo run --release gmp
     Running `target/release/discrete-log gmp`
375374217830

 Performance counter stats for 'cargo run --release gmp':

       1034.907032      task-clock (msec)         #    0.874 CPUs utilized          
               420      context-switches          #    0.406 K/sec                  
                19      cpu-migrations            #    0.018 K/sec                  
            44,346      page-faults               #    0.043 M/sec                  
     3,848,978,476      cycles                    #    3.719 GHz                    
   <not supported>      stalled-cycles-frontend  
   <not supported>      stalled-cycles-backend   
     6,381,317,237      instructions              #    1.66  insns per cycle        
       731,825,686      branches                  #  707.141 M/sec                  
         4,660,441      branch-misses             #    0.64% of all branches        

       1.184163507 seconds time elapsed

```

There is also a micro benchmark that compares the performance of modular
multiplication. Use `cargo bench` to run it:

```
test bigint_mulmod ... bench:       2,001 ns/iter (+/- 157)
test int_mulmod    ... bench:         409 ns/iter (+/- 16)
test mpz_mulmod    ... bench:         272 ns/iter (+/- 16)
test rugint_mulmod ... bench:         267 ns/iter (+/- 4)
```
