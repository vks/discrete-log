# discrete-log

[![Build Status](https://travis-ci.org/vks/discrete-log.svg?branch=master)](https://travis-ci.org/vks/discrete-log)

[Discrete logarithm](https://en.wikipedia.org/wiki/Discrete_logarithm)
implementation in Rust that can serve as a benchmark for bigints.

## Building

By default, any bignum crates using GMP are disabled.
You can enable the "gmp" feature if GMP is installed.
You can enable the "rug" feature if you are not using Windows.
(The `rug` crate builds its own version of GMP and MPFR.)

## Example

You can use this to compare the performance of the pure-Rust bigint
implementation in [`num`](https://crates.io/crates/num) with
[GMP](https://gmplib.org/):

```
► cargo build --release
[...]
► perf stat -B target/release/discrete-log num_bigint
375374217830

 Performance counter stats for 'target/release/discrete-log num_bigint':

       3511.817048      task-clock (msec)         #    0.998 CPUs utilized          
                14      context-switches          #    0.004 K/sec                  
                 0      cpu-migrations            #    0.000 K/sec                  
            41,578      page-faults               #    0.012 M/sec                  
    13,616,656,227      cycles                    #    3.877 GHz                    
   <not supported>      stalled-cycles-frontend  
   <not supported>      stalled-cycles-backend   
    34,213,980,451      instructions              #    2.51  insns per cycle        
     5,388,009,008      branches                  # 1534.251 M/sec                  
         3,926,186      branch-misses             #    0.07% of all branches        

       3.519970328 seconds time elapsed

► perf stat -B target/release/discrete-log gmp
375374217830

 Performance counter stats for 'target/release/discrete-log gmp':

        875.194874      task-clock (msec)         #    0.995 CPUs utilized          
                 4      context-switches          #    0.005 K/sec                  
                 0      cpu-migrations            #    0.000 K/sec                  
            33,378      page-faults               #    0.038 M/sec                  
     3,393,158,699      cycles                    #    3.877 GHz                    
   <not supported>      stalled-cycles-frontend  
   <not supported>      stalled-cycles-backend   
     5,075,926,634      instructions              #    1.50  insns per cycle        
       597,719,966      branches                  #  682.956 M/sec                  
         1,616,851      branch-misses             #    0.27% of all branches        

       0.879850380 seconds time elapsed

► perf stat -B target/release/discrete-log ramp
375374217830

 Performance counter stats for 'target/release/discrete-log ramp':

       1357.094735      task-clock (msec)         #    0.995 CPUs utilized          
                 8      context-switches          #    0.006 K/sec                  
                 0      cpu-migrations            #    0.000 K/sec                  
            33,381      page-faults               #    0.025 M/sec                  
     5,240,413,785      cycles                    #    3.861 GHz                    
   <not supported>      stalled-cycles-frontend  
   <not supported>      stalled-cycles-backend   
     9,308,410,772      instructions              #    1.78  insns per cycle        
     1,596,274,651      branches                  # 1176.244 M/sec                  
         1,309,634      branch-misses             #    0.08% of all branches        

       1.363674291 seconds time elapsed

► perf stat -B target/release/discrete-log rug
375374217830

 Performance counter stats for 'target/release/discrete-log rug':

        846.578013      task-clock (msec)         #    0.994 CPUs utilized          
                 5      context-switches          #    0.006 K/sec                  
                 0      cpu-migrations            #    0.000 K/sec                  
            33,380      page-faults               #    0.039 M/sec                  
     3,259,830,382      cycles                    #    3.851 GHz                    
   <not supported>      stalled-cycles-frontend  
   <not supported>      stalled-cycles-backend   
     4,788,026,599      instructions              #    1.47  insns per cycle        
       537,150,703      branches                  #  634.496 M/sec                  
         1,612,462      branch-misses             #    0.30% of all branches        

       0.851276668 seconds time elapsed
```

There is also a micro benchmark that compares the performance of modular
multiplication. Use `cargo bench` to run it:

```
[...]
mulmod/gmp              time:   [305.05 ns 305.88 ns 306.77 ns]
[...]
mulmod/num_bigint       time:   [2.3782 us 2.4034 us 2.4408 us]
[...]
mulmod/ramp             time:   [512.84 ns 513.65 ns 514.57 ns]
[...]
mulmod/rug              time:   [313.31 ns 313.81 ns 314.39 ns]
[...]
```
