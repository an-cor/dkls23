# sanity check
cargo run -p dkls-metrics -r -- dkg --n 3 --t 2

Finished `release` profile [optimized] target(s) in 0.23s
Running `target/release/dkls-metrics dkg --n 3 --t 2`
DKG: traced 21 messages, 329445 total bytes
DKG: N =  3, T =  2, K = 100, t = 204.103382ms, 20.410338292s
DKG: send 19 110247, recv 10 110442

cargo run -p dkls-metrics -r -- dkg --n 3 --t 2 --dsg

Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 3 --t 2 --dsg`
DKG: traced 21 messages, 329445 total bytes
DKG: N =  3, T =  2, K = 100, t = 203.70792ms, 20.370792042s
DKG: send 19 110247, recv 10 110442
DSG: traced 8 messages, 118330 total bytes
DSG: send 9 59345, recv 4 59165
DSG: N =  3, T =  2, K = 100, t = 8.172245ms, 817.224542ms

# runtime measurements 

## dkg baseline n 3 t 2

time cargo run -p dkls-metrics -r -- dkg --n 3 --t 2
time cargo run -p dkls-metrics -r -- dkg --n 3 --t 2
time cargo run -p dkls-metrics -r -- dkg --n 3 --t 2


warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 3 --t 2`
DKG: traced 21 messages, 329445 total bytes
DKG: N =  3, T =  2, K = 100, t = 204.493232ms, 20.449323208s
DKG: send 19 110247, recv 10 110442
cargo run -p dkls-metrics -r -- dkg --n 3 --t 2  20.98s user 0.11s system 100% cpu 21.056 total
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 3 --t 2`
DKG: traced 21 messages, 329445 total bytes
DKG: N =  3, T =  2, K = 100, t = 203.454583ms, 20.345458334s
DKG: send 19 110247, recv 10 110442
cargo run -p dkls-metrics -r -- dkg --n 3 --t 2  20.91s user 0.09s system 100% cpu 20.874 total
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 3 --t 2`
DKG: traced 21 messages, 329445 total bytes
DKG: N =  3, T =  2, K = 100, t = 203.308756ms, 20.330875667s
DKG: send 19 110247, recv 10 110442
cargo run -p dkls-metrics -r -- dkg --n 3 --t 2  20.91s user 0.08s system 100% cpu 20.876 total

### dkg variation n 5 t 2

time cargo run -p dkls-metrics -r -- dkg --n 5 --t 2
time cargo run -p dkls-metrics -r -- dkg --n 5 --t 2

warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 5 --t 2`
DKG: traced 55 messages, 1095015 total bytes
DKG: N =  5, T =  2, K = 100, t = 407.049489ms, 40.704948917s
DKG: send 35 219867, recv 20 220884
cargo run -p dkls-metrics -r -- dkg --n 5 --t 2  42.64s user 0.12s system 103% cpu 41.501 total
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 5 --t 2`
DKG: traced 55 messages, 1095015 total bytes
DKG: N =  5, T =  2, K = 100, t = 405.863077ms, 40.586307791s
DKG: send 35 219867, recv 20 220884
cargo run -p dkls-metrics -r -- dkg --n 5 --t 2  42.57s user 0.12s system 103% cpu 41.386 total


### dkg variation n 5 t 3

time cargo run -p dkls-metrics -r -- dkg --n 5 --t 3
time cargo run -p dkls-metrics -r -- dkg --n 5 --t 3


warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 5 --t 3`
DKG: traced 55 messages, 1096165 total bytes
DKG: N =  5, T =  3, K = 100, t = 406.595907ms, 40.65959075s
DKG: send 35 220097, recv 20 221408
cargo run -p dkls-metrics -r -- dkg --n 5 --t 3  42.66s user 0.13s system 102% cpu 41.571 total
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 5 --t 3`
DKG: traced 55 messages, 1096165 total bytes
DKG: N =  5, T =  3, K = 100, t = 407.166446ms, 40.716644625s
DKG: send 35 220097, recv 20 221408
cargo run -p dkls-metrics -r -- dkg --n 5 --t 3  42.80s user 0.10s system 103% cpu 41.449 total


### dkg variation n 10 t 5

time cargo run -p dkls-metrics -r -- dkg --n 10 --t 5
time cargo run -p dkls-metrics -r -- dkg --n 10 --t 5


warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 10 --t 5`
DKG: traced 210 messages, 4931580 total bytes
DKG: N = 10, T =  5, K = 100, t = 919.390785ms, 91.939078542s
DKG: send 75 495102, recv 45 500526
cargo run -p dkls-metrics -r -- dkg --n 10 --t 5  102.37s user 0.25s system 109% cpu 1:33.67 total
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 10 --t 5`
DKG: traced 210 messages, 4931580 total bytes
DKG: N = 10, T =  5, K = 100, t = 917.66309ms, 91.766309s
DKG: send 75 495102, recv 45 500526
cargo run -p dkls-metrics -r -- dkg --n 10 --t 5  102.52s user 0.21s system 109% cpu 1:33.49 total


### dkg variation n 10 t 6

time cargo run -p dkls-metrics -r -- dkg --n 10 --t 6
time cargo run -p dkls-metrics -r -- dkg --n 10 --t 6


warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 10 --t 6`
DKG: traced 210 messages, 4935530 total bytes
DKG: N = 10, T =  6, K = 100, t = 922.652223ms, 92.265222333s
DKG: send 75 495497, recv 45 501705
cargo run -p dkls-metrics -r -- dkg --n 10 --t 6  102.62s user 0.26s system 109% cpu 1:34.03 total
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 10 --t 6`
DKG: traced 210 messages, 4935530 total bytes
DKG: N = 10, T =  6, K = 100, t = 923.536297ms, 92.35362975s
DKG: send 75 495497, recv 45 501705
cargo run -p dkls-metrics -r -- dkg --n 10 --t 6  102.93s user 0.25s system 109% cpu 1:33.99 total

### dkg variation n 10 t 7

time cargo run -p dkls-metrics -r -- dkg --n 10 --t 7
time cargo run -p dkls-metrics -r -- dkg --n 10 --t 7


warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 10 --t 7`
DKG: traced 210 messages, 4939480 total bytes
DKG: N = 10, T =  7, K = 100, t = 923.080233ms, 92.308023334s
DKG: send 75 495892, recv 45 502884
cargo run -p dkls-metrics -r -- dkg --n 10 --t 7  102.43s user 0.33s system 109% cpu 1:34.12 total
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 10 --t 7`
DKG: traced 210 messages, 4939480 total bytes
DKG: N = 10, T =  7, K = 100, t = 923.007406ms, 92.300740625s
DKG: send 75 495892, recv 45 502884
cargo run -p dkls-metrics -r -- dkg --n 10 --t 7  103.01s user 0.19s system 109% cpu 1:34.14 total



### dkg variation n 10 t 8

time cargo run -p dkls-metrics -r -- dkg --n 10 --t 8
time cargo run -p dkls-metrics -r -- dkg --n 10 --t 8

warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 10 --t 8`
DKG: traced 210 messages, 4943430 total bytes
DKG: N = 10, T =  8, K = 100, t = 926.61747ms, 92.661747042s
DKG: send 75 496287, recv 45 504063
cargo run -p dkls-metrics -r -- dkg --n 10 --t 8  103.20s user 0.25s system 109% cpu 1:34.40 total
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 10 --t 8`
DKG: traced 210 messages, 4943430 total bytes
DKG: N = 10, T =  8, K = 100, t = 926.352193ms, 92.635219333s
DKG: send 75 496287, recv 45 504063
cargo run -p dkls-metrics -r -- dkg --n 10 --t 8  103.43s user 0.19s system 109% cpu 1:34.50 total



## dkg + dsg baseline n 3 t 2

time cargo run -p dkls-metrics -r -- dkg --n 3 --t 2 --dsg
time cargo run -p dkls-metrics -r -- dkg --n 3 --t 2 --dsg
time cargo run -p dkls-metrics -r -- dkg --n 3 --t 2 --dsg



warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 3 --t 2 --dsg`
DKG: traced 21 messages, 329445 total bytes
DKG: N =  3, T =  2, K = 100, t = 203.390089ms, 20.339008917s
DKG: send 19 110247, recv 10 110442
DSG: traced 8 messages, 118330 total bytes
DSG: send 9 59345, recv 4 59165
DSG: N =  3, T =  2, K = 100, t = 8.153865ms, 815.386583ms
cargo run -p dkls-metrics -r -- dkg --n 3 --t 2 --dsg  21.77s user 0.07s system 100% cpu 21.730 total
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 3 --t 2 --dsg`
DKG: traced 21 messages, 329445 total bytes
DKG: N =  3, T =  2, K = 100, t = 203.922147ms, 20.392214791s
DKG: send 19 110247, recv 10 110442
DSG: traced 8 messages, 118330 total bytes
DSG: send 9 59345, recv 4 59165
DSG: N =  3, T =  2, K = 100, t = 8.155178ms, 815.517875ms
cargo run -p dkls-metrics -r -- dkg --n 3 --t 2 --dsg  21.85s user 0.06s system 100% cpu 21.794 total
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 3 --t 2 --dsg`
DKG: traced 21 messages, 329445 total bytes
DKG: N =  3, T =  2, K = 100, t = 202.788386ms, 20.278838625s
DKG: send 19 110247, recv 10 110442
DSG: traced 8 messages, 118330 total bytes
DSG: send 9 59345, recv 4 59165
DSG: N =  3, T =  2, K = 100, t = 8.154617ms, 815.461792ms
cargo run -p dkls-metrics -r -- dkg --n 3 --t 2 --dsg  21.74s user 0.06s system 100% cpu 21.701 total


### dkg + dsg variation n 5 t 2

time cargo run -p dkls-metrics -r -- dkg --n 5 --t 2 --dsg
time cargo run -p dkls-metrics -r -- dkg --n 5 --t 2 --dsg

warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 5 --t 2 --dsg`
DKG: traced 55 messages, 1095015 total bytes
DKG: N =  5, T =  2, K = 100, t = 405.525645ms, 40.552564583s
DKG: send 35 219867, recv 20 220884
DSG: traced 8 messages, 118330 total bytes
DSG: send 9 59345, recv 4 59165
DSG: N =  5, T =  2, K = 100, t = 8.140463ms, 814.046333ms
cargo run -p dkls-metrics -r -- dkg --n 5 --t 2 --dsg  43.48s user 0.09s system 103% cpu 42.108 total
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 5 --t 2 --dsg`
DKG: traced 55 messages, 1095015 total bytes
DKG: N =  5, T =  2, K = 100, t = 405.11016ms, 40.511016042s
DKG: send 35 219867, recv 20 220884
DSG: traced 8 messages, 118330 total bytes
DSG: send 9 59345, recv 4 59165
DSG: N =  5, T =  2, K = 100, t = 8.239582ms, 823.958291ms
cargo run -p dkls-metrics -r -- dkg --n 5 --t 2 --dsg  43.41s user 0.09s system 103% cpu 42.224 total


### dkg + dsg variation n 5 t 3

time cargo run -p dkls-metrics -r -- dkg --n 5 --t 3 --dsg
time cargo run -p dkls-metrics -r -- dkg --n 5 --t 3 --dsg


warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 5 --t 3 --dsg`
DKG: traced 55 messages, 1096165 total bytes
DKG: N =  5, T =  3, K = 100, t = 471.052526ms, 47.105252667s
DKG: send 35 220097, recv 20 221408
DSG: traced 18 messages, 354195 total bytes
DSG: send 16 118425, recv 8 118330
DSG: N =  5, T =  3, K = 100, t = 24.837942ms, 2.483794291s
cargo run -p dkls-metrics -r -- dkg --n 5 --t 3 --dsg  51.33s user 0.29s system 102% cpu 50.448 total
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.08s
     Running `target/release/dkls-metrics dkg --n 5 --t 3 --dsg`
DKG: traced 55 messages, 1096165 total bytes
DKG: N =  5, T =  3, K = 100, t = 501.473507ms, 50.14735075s
DKG: send 35 220097, recv 20 221408
DSG: traced 18 messages, 354195 total bytes
DSG: send 16 118425, recv 8 118330
DSG: N =  5, T =  3, K = 100, t = 20.1858ms, 2.018580042s
cargo run -p dkls-metrics -r -- dkg --n 5 --t 3 --dsg  57.25s user 0.17s system 106% cpu 53.788 total

### dkg + dsg variation n 10 t 5

time cargo run -p dkls-metrics -r -- dkg --n 10 --t 5 --dsg
time cargo run -p dkls-metrics -r -- dkg --n 10 --t 5 --dsg


warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 10 --t 5 --dsg`
DKG: traced 210 messages, 4931580 total bytes
DKG: N = 10, T =  5, K = 100, t = 1.103416012s, 110.341601292s
DKG: send 75 495102, recv 45 500526
DSG: traced 50 messages, 1179325 total bytes
DSG: send 30 236585, recv 16 236660
DSG: N = 10, T =  5, K = 100, t = 40.806355ms, 4.080635541s
cargo run -p dkls-metrics -r -- dkg --n 10 --t 5 --dsg  139.66s user 0.40s system 118% cpu 1:58.41 total
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.06s
     Running `target/release/dkls-metrics dkg --n 10 --t 5 --dsg`
DKG: traced 210 messages, 4931580 total bytes
DKG: N = 10, T =  5, K = 100, t = 1.09677224s, 109.677224s
DKG: send 75 495102, recv 45 500526
DSG: traced 50 messages, 1179325 total bytes
DSG: send 30 236585, recv 16 236660
DSG: N = 10, T =  5, K = 100, t = 42.710289ms, 4.271028958s
cargo run -p dkls-metrics -r -- dkg --n 10 --t 5 --dsg  138.15s user 0.24s system 117% cpu 1:57.84 total

### dkg + dsg variation n 10 t 6

time cargo run -p dkls-metrics -r -- dkg --n 10 --t 6 --dsg
time cargo run -p dkls-metrics -r -- dkg --n 10 --t 6 --dsg


warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.20s
     Running `target/release/dkls-metrics dkg --n 10 --t 6 --dsg`
DKG: traced 210 messages, 4935530 total bytes
DKG: N = 10, T =  6, K = 100, t = 905.637837ms, 90.56378375s
DKG: send 75 495497, recv 45 501705
DSG: traced 72 messages, 1768590 total bytes
DSG: send 37 295665, recv 20 295825
DSG: N = 10, T =  6, K = 100, t = 39.171115ms, 3.917111584s
cargo run -p dkls-metrics -r -- dkg --n 10 --t 6 --dsg  104.33s user 0.90s system 109% cpu 1:36.45 total
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.04s
     Running `target/release/dkls-metrics dkg --n 10 --t 6 --dsg`
DKG: traced 210 messages, 4935530 total bytes
DKG: N = 10, T =  6, K = 100, t = 902.950358ms, 90.295035875s
DKG: send 75 495497, recv 45 501705
DSG: traced 72 messages, 1768590 total bytes
DSG: send 37 295665, recv 20 295825
DSG: N = 10, T =  6, K = 100, t = 39.70308ms, 3.970308042s
cargo run -p dkls-metrics -r -- dkg --n 10 --t 6 --dsg  103.98s user 0.77s system 108% cpu 1:36.20 total

### dkg + dsg variation n 10 t 7

time cargo run -p dkls-metrics -r -- dkg --n 10 --t 7 --dsg
time cargo run -p dkls-metrics -r -- dkg --n 10 --t 7 --dsg

warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 10 --t 7 --dsg`
DKG: traced 210 messages, 4939480 total bytes
DKG: N = 10, T =  7, K = 100, t = 907.00969ms, 90.700969041s
DKG: send 75 495892, recv 45 502884
DSG: traced 98 messages, 2475655 total bytes
DSG: send 44 354745, recv 24 354990
DSG: N = 10, T =  7, K = 100, t = 46.9303ms, 4.693030041s
cargo run -p dkls-metrics -r -- dkg --n 10 --t 7 --dsg  105.42s user 0.83s system 109% cpu 1:37.14 total
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 10 --t 7 --dsg`
DKG: traced 210 messages, 4939480 total bytes
DKG: N = 10, T =  7, K = 100, t = 920.852272ms, 92.08522725s
DKG: send 75 495892, recv 45 502884
DSG: traced 98 messages, 2475655 total bytes
DSG: send 44 354745, recv 24 354990
DSG: N = 10, T =  7, K = 100, t = 47.435318ms, 4.743531875s
cargo run -p dkls-metrics -r -- dkg --n 10 --t 7 --dsg  105.91s user 0.88s system 108% cpu 1:38.51 total


### dkg + dsg variation n 10 t 8

time cargo run -p dkls-metrics -r -- dkg --n 10 --t 8 --dsg
time cargo run -p dkls-metrics -r -- dkg --n 10 --t 8 --dsg


warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 10 --t 8 --dsg`
DKG: traced 210 messages, 4943430 total bytes
DKG: N = 10, T =  8, K = 100, t = 911.154484ms, 91.115448417s
DKG: send 75 496287, recv 45 504063
DSG: traced 128 messages, 3300520 total bytes
DSG: send 51 413825, recv 28 414155
DSG: N = 10, T =  8, K = 100, t = 54.815061ms, 5.481506166s
cargo run -p dkls-metrics -r -- dkg --n 10 --t 8 --dsg  106.60s user 0.97s system 109% cpu 1:38.29 total
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 10 --t 8 --dsg`
DKG: traced 210 messages, 4943430 total bytes
DKG: N = 10, T =  8, K = 100, t = 908.184461ms, 90.818446166s
DKG: send 75 496287, recv 45 504063
DSG: traced 128 messages, 3300520 total bytes
DSG: send 51 413825, recv 28 414155
DSG: N = 10, T =  8, K = 100, t = 54.729542ms, 5.472954208s
cargo run -p dkls-metrics -r -- dkg --n 10 --t 8 --dsg  106.30s user 0.84s system 109% cpu 1:38.08 total





# memory measurements

## dkg baseline n 3 t 2

/usr/bin/time -l cargo run -p dkls-metrics -r -- dkg --n 3 --t 2
/usr/bin/time -l cargo run -p dkls-metrics -r -- dkg --n 3 --t 2


warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 3 --t 2`
DKG: traced 21 messages, 329445 total bytes
DKG: N =  3, T =  2, K = 100, t = 203.202275ms, 20.3202275s
DKG: send 19 110247, recv 10 110442
       20.88 real        20.88 user         0.08 sys
             6488064  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                4009  page reclaims
                  43  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
                3722  involuntary context switches
        219288720026  instructions retired
         65646510669  cycles elapsed
             4948992  peak memory footprint
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 3 --t 2`
DKG: traced 21 messages, 329445 total bytes
DKG: N =  3, T =  2, K = 100, t = 202.61065ms, 20.261065083s
DKG: send 19 110247, recv 10 110442
       20.86 real        20.86 user         0.07 sys
             6438912  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                4026  page reclaims
                  43  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
                3047  involuntary context switches
        219176807343  instructions retired
         65559617271  cycles elapsed
             4899840  peak memory footprint

### dkg variation n 5 t 2

/usr/bin/time -l cargo run -p dkls-metrics -r -- dkg --n 5 --t 2
/usr/bin/time -l cargo run -p dkls-metrics -r -- dkg --n 5 --t 2

warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 5 --t 2`
DKG: traced 55 messages, 1095015 total bytes
DKG: N =  5, T =  2, K = 100, t = 395.921277ms, 39.592127791s
DKG: send 35 219867, recv 20 220884
       40.44 real        41.61 user         0.21 sys
            11354112  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                4310  page reclaims
                  43  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
                3335  involuntary context switches
        445680901548  instructions retired
        133953261127  cycles elapsed
             9782144  peak memory footprint
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 5 --t 2`
DKG: traced 55 messages, 1095015 total bytes
DKG: N =  5, T =  2, K = 100, t = 396.219274ms, 39.621927459s
DKG: send 35 219867, recv 20 220884
       40.40 real        41.58 user         0.23 sys
            10928128  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                4295  page reclaims
                  43  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
                3405  involuntary context switches
        445610463469  instructions retired
        133899168869  cycles elapsed
             9356160  peak memory footprint


### dkg variation n 5 t 3

/usr/bin/time -l cargo run -p dkls-metrics -r -- dkg --n 5 --t 3
/usr/bin/time -l cargo run -p dkls-metrics -r -- dkg --n 5 --t 3

warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.06s
     Running `target/release/dkls-metrics dkg --n 5 --t 3`
DKG: traced 55 messages, 1096165 total bytes
DKG: N =  5, T =  3, K = 100, t = 399.493977ms, 39.949397792s
DKG: send 35 220097, recv 20 221408
       40.87 real        41.80 user         0.33 sys
            11091968  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                4327  page reclaims
                  43  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
                5057  involuntary context switches
        447359923555  instructions retired
        134779144058  cycles elapsed
             9520000  peak memory footprint
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 5 --t 3`
DKG: traced 55 messages, 1096165 total bytes
DKG: N =  5, T =  3, K = 100, t = 400.838392ms, 40.08383925s
DKG: send 35 220097, recv 20 221408
       40.91 real        41.83 user         0.30 sys
            11173888  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                4300  page reclaims
                  43  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   1  voluntary context switches
                9566  involuntary context switches
        447021042791  instructions retired
        134674624537  cycles elapsed
             9601920  peak memory footprint


### dkg variation n 10 t 5

/usr/bin/time -l cargo run -p dkls-metrics -r -- dkg --n 10 --t 5
/usr/bin/time -l cargo run -p dkls-metrics -r -- dkg --n 10 --t 5


warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 10 --t 5`
DKG: traced 210 messages, 4931580 total bytes
DKG: N = 10, T =  5, K = 100, t = 897.160109ms, 89.716010958s
DKG: send 75 495102, recv 45 500526
       91.26 real        99.69 user         0.50 sys
            33390592  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                5658  page reclaims
                  43  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
               13870  involuntary context switches
       1057364363165  instructions retired
        319159689976  cycles elapsed
            31851520  peak memory footprint
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.07s
     Running `target/release/dkls-metrics dkg --n 10 --t 5`
DKG: traced 210 messages, 4931580 total bytes
DKG: N = 10, T =  5, K = 100, t = 896.871414ms, 89.687141417s
DKG: send 75 495102, recv 45 500526
       91.32 real        99.80 user         0.48 sys
            32505856  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                5632  page reclaims
                  43  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
                7284  involuntary context switches
       1057517349434  instructions retired
        319176410386  cycles elapsed
            30950336  peak memory footprint

### dkg variation n 10 t 6

/usr/bin/time -l cargo run -p dkls-metrics -r -- dkg --n 10 --t 6
/usr/bin/time -l cargo run -p dkls-metrics -r -- dkg --n 10 --t 6


warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 10 --t 6`
DKG: traced 210 messages, 4935530 total bytes
DKG: N = 10, T =  6, K = 100, t = 908.280781ms, 90.828078125s
DKG: send 75 495497, recv 45 501705
       92.54 real       100.79 user         0.35 sys
            32768000  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                5681  page reclaims
                  43  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
               10372  involuntary context switches
       1059998033102  instructions retired
        319410719714  cycles elapsed
            31212480  peak memory footprint
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 10 --t 6`
DKG: traced 210 messages, 4935530 total bytes
DKG: N = 10, T =  6, K = 100, t = 902.656631ms, 90.265663167s
DKG: send 75 495497, recv 45 501705
       91.99 real       100.21 user         0.43 sys
            32899072  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                5655  page reclaims
                  43  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
               14298  involuntary context switches
       1059722610944  instructions retired
        319544161144  cycles elapsed
            31343552  peak memory footprint

### dkg variation n 10 t 7

/usr/bin/time -l cargo run -p dkls-metrics -r -- dkg --n 10 --t 7
/usr/bin/time -l cargo run -p dkls-metrics -r -- dkg --n 10 --t 7

warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 10 --t 7`
DKG: traced 210 messages, 4939480 total bytes
DKG: N = 10, T =  7, K = 100, t = 902.981625ms, 90.298162542s
DKG: send 75 495892, recv 45 502884
       92.16 real       100.16 user         0.64 sys
            32768000  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                5630  page reclaims
                  43  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
               11941  involuntary context switches
       1062027697310  instructions retired
        320584702483  cycles elapsed
            31212480  peak memory footprint
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 10 --t 7`
DKG: traced 210 messages, 4939480 total bytes
DKG: N = 10, T =  7, K = 100, t = 906.764901ms, 90.676490125s
DKG: send 75 495892, recv 45 502884
       92.23 real       100.34 user         0.79 sys
            32276480  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                5580  page reclaims
                  43  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
               15414  involuntary context switches
       1062889247939  instructions retired
        321499135704  cycles elapsed
            30720960  peak memory footprint


### dkg variation n 10 t 8

/usr/bin/time -l cargo run -p dkls-metrics -r -- dkg --n 10 --t 8
/usr/bin/time -l cargo run -p dkls-metrics -r -- dkg --n 10 --t 8


warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 10 --t 8`
DKG: traced 210 messages, 4943430 total bytes
DKG: N = 10, T =  8, K = 100, t = 908.822297ms, 90.88222975s
DKG: send 75 496287, recv 45 504063
       92.36 real       100.85 user         0.72 sys
            32817152  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                5625  page reclaims
                  43  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
               12460  involuntary context switches
       1065973724243  instructions retired
        322703850894  cycles elapsed
            31278080  peak memory footprint
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 10 --t 8`
DKG: traced 210 messages, 4943430 total bytes
DKG: N = 10, T =  8, K = 100, t = 908.335913ms, 90.833591334s
DKG: send 75 496287, recv 45 504063
       92.49 real       100.80 user         0.73 sys
            33619968  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                5667  page reclaims
                  43  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
               13066  involuntary context switches
       1065384937137  instructions retired
        322386485668  cycles elapsed
            32080896  peak memory footprint


## dkg + dsg baseline n 3 t 2

/usr/bin/time -l cargo run -p dkls-metrics -r -- dkg --n 3 --t 2 --dsg
/usr/bin/time -l cargo run -p dkls-metrics -r -- dkg --n 3 --t 2 --dsg


warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 3 --t 2 --dsg`
DKG: traced 21 messages, 329445 total bytes
DKG: N =  3, T =  2, K = 100, t = 203.230175ms, 20.323017583s
DKG: send 19 110247, recv 10 110442
DSG: traced 8 messages, 118330 total bytes
DSG: send 9 59345, recv 4 59165
DSG: N =  3, T =  2, K = 100, t = 8.666458ms, 866.645875ms
       21.76 real        21.73 user         0.07 sys
             7258112  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                4104  page reclaims
                  43  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
                2593  involuntary context switches
        228943831902  instructions retired
         68220328194  cycles elapsed
             5440512  peak memory footprint
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 3 --t 2 --dsg`
DKG: traced 21 messages, 329445 total bytes
DKG: N =  3, T =  2, K = 100, t = 202.496553ms, 20.249655375s
DKG: send 19 110247, recv 10 110442
DSG: traced 8 messages, 118330 total bytes
DSG: send 9 59345, recv 4 59165
DSG: N =  3, T =  2, K = 100, t = 8.259858ms, 825.985875ms
       21.62 real        21.69 user         0.07 sys
             7176192  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                4075  page reclaims
                  43  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
                1601  involuntary context switches
        228924295098  instructions retired
         68145084174  cycles elapsed
             5555200  peak memory footprint

### dkg + dsg variation n 5 t 2

/usr/bin/time -l cargo run -p dkls-metrics -r -- dkg --n 5 --t 2 --dsg
/usr/bin/time -l cargo run -p dkls-metrics -r -- dkg --n 5 --t 2 --dsg

warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 5 --t 2 --dsg`
DKG: traced 55 messages, 1095015 total bytes
DKG: N =  5, T =  2, K = 100, t = 396.187215ms, 39.6187215s
DKG: send 35 219867, recv 20 220884
DSG: traced 8 messages, 118330 total bytes
DSG: send 9 59345, recv 4 59165
DSG: N =  5, T =  2, K = 100, t = 8.008809ms, 800.880917ms
       41.32 real        42.36 user         0.29 sys
            11304960  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                4335  page reclaims
                  43  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
                3916  involuntary context switches
        455736761073  instructions retired
        136624724576  cycles elapsed
             9667520  peak memory footprint
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 5 --t 2 --dsg`
DKG: traced 55 messages, 1095015 total bytes
DKG: N =  5, T =  2, K = 100, t = 396.155795ms, 39.615579583s
DKG: send 35 219867, recv 20 220884
DSG: traced 8 messages, 118330 total bytes
DSG: send 9 59345, recv 4 59165
DSG: N =  5, T =  2, K = 100, t = 7.921739ms, 792.173917ms
       41.32 real        42.25 user         0.32 sys
            11108352  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                4324  page reclaims
                  43  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
                4394  involuntary context switches
        455317220650  instructions retired
        136424461383  cycles elapsed
             9208704  peak memory footprint

### dkg + dsg variation n 5 t 3

/usr/bin/time -l cargo run -p dkls-metrics -r -- dkg --n 5 --t 3 --dsg
/usr/bin/time -l cargo run -p dkls-metrics -r -- dkg --n 5 --t 3 --dsg


warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 5 --t 3 --dsg`
DKG: traced 55 messages, 1096165 total bytes
DKG: N =  5, T =  3, K = 100, t = 397.726739ms, 39.772673917s
DKG: send 35 220097, recv 20 221408
DSG: traced 18 messages, 354195 total bytes
DSG: send 16 118425, recv 8 118330
DSG: N =  5, T =  3, K = 100, t = 15.768275ms, 1.5768275s
       42.27 real        43.26 user         0.33 sys
            11550720  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                4332  page reclaims
                  43  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
                4203  involuntary context switches
        466886018723  instructions retired
        139652433161  cycles elapsed
             9552832  peak memory footprint
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 5 --t 3 --dsg`
DKG: traced 55 messages, 1096165 total bytes
DKG: N =  5, T =  3, K = 100, t = 397.825725ms, 39.782572584s
DKG: send 35 220097, recv 20 221408
DSG: traced 18 messages, 354195 total bytes
DSG: send 16 118425, recv 8 118330
DSG: N =  5, T =  3, K = 100, t = 15.632665ms, 1.563266583s
       42.24 real        43.29 user         0.29 sys
            11665408  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                4358  page reclaims
                  43  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
                3747  involuntary context switches
        467268646706  instructions retired
        139678835055  cycles elapsed
             9601920  peak memory footprint

### dkg + dsg variation n 10 t 5

/usr/bin/time -l cargo run -p dkls-metrics -r -- dkg --n 10 --t 5 --dsg
/usr/bin/time -l cargo run -p dkls-metrics -r -- dkg --n 10 --t 5 --dsg

warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 10 --t 5 --dsg`
DKG: traced 210 messages, 4931580 total bytes
DKG: N = 10, T =  5, K = 100, t = 900.675867ms, 90.06758675s
DKG: send 75 495102, recv 45 500526
DSG: traced 50 messages, 1179325 total bytes
DSG: send 30 236585, recv 16 236660
DSG: N = 10, T =  5, K = 100, t = 30.975063ms, 3.097506375s
       94.74 real       103.16 user         0.74 sys
            34308096  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                5775  page reclaims
                  43  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
               17408  involuntary context switches
       1097967912610  instructions retired
        330168073307  cycles elapsed
            31622144  peak memory footprint
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 10 --t 5 --dsg`
DKG: traced 210 messages, 4931580 total bytes
DKG: N = 10, T =  5, K = 100, t = 897.466607ms, 89.746660792s
DKG: send 75 495102, recv 45 500526
DSG: traced 50 messages, 1179325 total bytes
DSG: send 30 236585, recv 16 236660
DSG: N = 10, T =  5, K = 100, t = 31.475742ms, 3.147574208s
       94.68 real       102.84 user         0.57 sys
            34324480  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                5774  page reclaims
                  43  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
               13442  involuntary context switches
       1097839008350  instructions retired
        328978037471  cycles elapsed
            31212480  peak memory footprint

### dkg + dsg variation n 10 t 6

/usr/bin/time -l cargo run -p dkls-metrics -r -- dkg --n 10 --t 6 --dsg
/usr/bin/time -l cargo run -p dkls-metrics -r -- dkg --n 10 --t 6 --dsg


warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 10 --t 6 --dsg`
DKG: traced 210 messages, 4935530 total bytes
DKG: N = 10, T =  6, K = 100, t = 899.10821ms, 89.910821084s
DKG: send 75 495497, recv 45 501705
DSG: traced 72 messages, 1768590 total bytes
DSG: send 37 295665, recv 20 295825
DSG: N = 10, T =  6, K = 100, t = 39.058179ms, 3.905817958s
       95.51 real       104.00 user         0.61 sys
            35340288  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                5845  page reclaims
                  43  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
               14133  involuntary context switches
       1111263604560  instructions retired
        332838935791  cycles elapsed
            30983168  peak memory footprint
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.04s
     Running `target/release/dkls-metrics dkg --n 10 --t 6 --dsg`
DKG: traced 210 messages, 4935530 total bytes
DKG: N = 10, T =  6, K = 100, t = 896.45321ms, 89.645321s
DKG: send 75 495497, recv 45 501705
DSG: traced 72 messages, 1768590 total bytes
DSG: send 37 295665, recv 20 295825
DSG: N = 10, T =  6, K = 100, t = 38.906706ms, 3.890670625s
       95.27 real       103.76 user         0.45 sys
            36880384  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                5914  page reclaims
                  43  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
               10141  involuntary context switches
       1110748384307  instructions retired
        331970383634  cycles elapsed
            30032832  peak memory footprint

### dkg + dsg variation n 10 t 7

/usr/bin/time -l cargo run -p dkls-metrics -r -- dkg --n 10 --t 7 --dsg
/usr/bin/time -l cargo run -p dkls-metrics -r -- dkg --n 10 --t 7 --dsg


warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 10 --t 7 --dsg`
DKG: traced 210 messages, 4939480 total bytes
DKG: N = 10, T =  7, K = 100, t = 903.18597ms, 90.318597s
DKG: send 75 495892, recv 45 502884
DSG: traced 98 messages, 2475655 total bytes
DSG: send 44 354745, recv 24 354990
DSG: N = 10, T =  7, K = 100, t = 46.811203ms, 4.681120375s
       96.85 real       105.19 user         0.57 sys
            38174720  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                6027  page reclaims
                  43  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
               13837  involuntary context switches
       1125491806739  instructions retired
        336779535395  cycles elapsed
            31523776  peak memory footprint
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 10 --t 7 --dsg`
DKG: traced 210 messages, 4939480 total bytes
DKG: N = 10, T =  7, K = 100, t = 899.490593ms, 89.949059333s
DKG: send 75 495892, recv 45 502884
DSG: traced 98 messages, 2475655 total bytes
DSG: send 44 354745, recv 24 354990
DSG: N = 10, T =  7, K = 100, t = 46.677316ms, 4.667731625s
       96.21 real       105.14 user         0.50 sys
            38862848  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                6014  page reclaims
                  43  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
                8665  involuntary context switches
       1124327383507  instructions retired
        336060520478  cycles elapsed
            31458240  peak memory footprint

### dkg + dsg variation n 10 t 8

/usr/bin/time -l cargo run -p dkls-metrics -r -- dkg --n 10 --t 8 --dsg
/usr/bin/time -l cargo run -p dkls-metrics -r -- dkg --n 10 --t 8 --dsg

warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 10 --t 8 --dsg`
DKG: traced 210 messages, 4943430 total bytes
DKG: N = 10, T =  8, K = 100, t = 899.986791ms, 89.998679166s
DKG: send 75 496287, recv 45 504063
DSG: traced 128 messages, 3300520 total bytes
DSG: send 51 413825, recv 28 414155
DSG: N = 10, T =  8, K = 100, t = 54.265985ms, 5.426598542s
       97.08 real       106.17 user         0.40 sys
            42123264  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                6254  page reclaims
                  43  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
               11469  involuntary context switches
       1138172709355  instructions retired
        339382196930  cycles elapsed
            31491008  peak memory footprint
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/crates/dkls-metrics/Cargo.toml
workspace: /Users/angelcoronel/Documents/CUNY/research/tushar/dkls23/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/dkls-metrics dkg --n 10 --t 8 --dsg`
DKG: traced 210 messages, 4943430 total bytes
DKG: N = 10, T =  8, K = 100, t = 903.478737ms, 90.34787375s
DKG: send 75 496287, recv 45 504063
DSG: traced 128 messages, 3300520 total bytes
DSG: send 51 413825, recv 28 414155
DSG: N = 10, T =  8, K = 100, t = 54.941208ms, 5.494120833s
       97.45 real       106.51 user         0.52 sys
            41648128  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                6220  page reclaims
                  43  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
                9091  involuntary context switches
       1138946116571  instructions retired
        340446482399  cycles elapsed
            30917632  peak memory footprint



# runtime tables

## dkg averages

|  n |  t | Avg Time (total) | Avg DKG Time | Messages | Bytes   | Send        | Recv        |
| -: | -: | ---------------- | ------------ | -------- | ------- | ----------- | ----------- |
|  3 |  2 | 20.935s          | 20.375s      | 21       | 329445  | 19 / 110247 | 10 / 110442 |
|  5 |  2 | 41.444s          | 40.646s      | 55       | 1095015 | 35 / 219867 | 20 / 220884 |
|  5 |  3 | 41.510s          | 40.688s      | 55       | 1096165 | 35 / 220097 | 20 / 221408 |
| 10 |  5 | 93.580s          | 91.853s      | 210      | 4931580 | 75 / 495102 | 45 / 500526 |
| 10 |  6 | 94.010s          | 92.310s      | 210      | 4935530 | 75 / 495497 | 45 / 501705 |
| 10 |  7 | 94.130s          | 92.305s      | 210      | 4939480 | 75 / 495892 | 45 / 502884 |
| 10 |  8 | 94.450s          | 92.649s      | 210      | 4943430 | 75 / 496287 | 45 / 504063 |

## dkg + dsg averages table

|  n |  t | Avg Total Time | Avg DKG Time | Avg DSG Time | DKG Msg | DSG Msg | DKG Bytes | DSG Bytes |
| -: | -: | -------------- | ------------ | ------------ | ------- | ------- | --------- | --------- |
|  3 |  2 | 21.742s        | 20.337s      | 0.815s       | 21      | 8       | 329445    | 118330    |
|  5 |  2 | 42.166s        | 40.532s      | 0.819s       | 55      | 8       | 1095015   | 118330    |
|  5 |  3 | 52.118s        | 48.626s      | 2.251s       | 55      | 18      | 1096165   | 354195    |
| 10 |  5 | 118.125s       | 110.010s     | 4.176s       | 210     | 50      | 4931580   | 1179325   |
| 10 |  6 | 96.325s        | 90.429s      | 3.944s       | 210     | 72      | 4935530   | 1768590   |
| 10 |  7 | 97.825s        | 91.393s      | 4.719s       | 210     | 98      | 4939480   | 2475655   |
| 10 |  8 | 98.185s        | 90.967s      | 5.477s       | 210     | 128     | 4943430   | 3300520   |

## estimated dsg cost

computed as (DKG+DSG) − (DKG only)

|  n |  t | Est DSG Time | Est DSG Bytes |
| -: | -: | ------------ | ------------- |
|  3 |  2 | ~0.81s       | 118330        |
|  5 |  2 | ~0.82s       | 118330        |
|  5 |  3 | ~2.25s       | 354195        |
| 10 |  5 | ~4.18s       | 1179325       |
| 10 |  6 | ~3.94s       | 1768590       |
| 10 |  7 | ~4.72s       | 2475655       |
| 10 |  8 | ~5.48s       | 3300520       |

## cobined final  - DKLS runtime

|  n |  t | DKG Time (s) | DSG Time (s) | Total Time (s) | DKG Messages | DSG Messages | DKG Bytes | DSG Bytes |
| -: | -: | ------------ | ------------ | -------------- | ------------ | ------------ | --------- | --------- |
|  3 |  2 | 20.375       | 0.815        | 21.742         | 21           | 8            | 329445    | 118330    |
|  5 |  2 | 40.646       | 0.819        | 42.166         | 55           | 8            | 1095015   | 118330    |
|  5 |  3 | 40.688       | 2.251        | 52.118         | 55           | 18           | 1096165   | 354195    |
| 10 |  5 | 91.853       | 4.176        | 118.125        | 210          | 50           | 4931580   | 1179325   |
| 10 |  6 | 92.310       | 3.944        | 96.325         | 210          | 72           | 4935530   | 1768590   |
| 10 |  7 | 92.305       | 4.719        | 97.825         | 210          | 98           | 4939480   | 2475655   |
| 10 |  8 | 92.649       | 5.477        | 98.185         | 210          | 128          | 4943430   | 3300520   |


- DKG is ~15–20× slower than DSG across all configurations

- DKG runtime grows significantly with n due to increased communication

- Communication (messages + bytes) dominates DKG cost

- DSG remains relatively lightweight, even as t increases

- Signing is fast enough to run on edge/IoT devices, while DKG should be done infrequently

# memory tables

## dkg averages

|  n |  t | Avg Real Time | Avg DKG Time | Messages | Bytes   | Avg Max RSS |
| -: | -: | ------------- | ------------ | -------- | ------- | ----------- |
|  3 |  2 | 20.87s        | 20.291s      | 21       | 329445  | 6463488     |
|  5 |  2 | 40.42s        | 39.607s      | 55       | 1095015 | 11141120    |
|  5 |  3 | 40.89s        | 40.017s      | 55       | 1096165 | 11132928    |
| 10 |  5 | 91.29s        | 89.702s      | 210      | 4931580 | 32948224    |
| 10 |  6 | 92.27s        | 90.547s      | 210      | 4935530 | 32833536    |
| 10 |  7 | 92.20s        | 90.487s      | 210      | 4939480 | 32522240    |
| 10 |  8 | 92.43s        | 90.858s      | 210      | 4943430 | 33218560    |

## dkg + dsg averages

|  n |  t | Avg DKG Time | Avg DSG Time | DKG Msg | DSG Msg | DKG Bytes | DSG Bytes |
| -: | -: | ------------ | ------------ | ------- | ------- | --------- | --------- |
| 10 |  6 | 90.564s      | 3.970s       | 210     | 72      | 4935530   | 1768590   |
| 10 |  7 | 91.393s      | 4.719s       | 210     | 98      | 4939480   | 2475655   |
| 10 |  8 | 90.967s      | 5.477s       | 210     | 128     | 4943430   | 3300520   |

## combined final table - DKLS memory

|  n |  t | DKG Time (s) | DSG Time (s) | Messages | Bytes   | Max RSS  |
| -: | -: | ------------ | ------------ | -------- | ------- | -------- |
|  3 |  2 | 20.291       | ~0.815       | 21       | 329445  | 6463488  |
|  5 |  2 | 39.607       | ~0.819       | 55       | 1095015 | 11141120 |
|  5 |  3 | 40.017       | ~2.251       | 55       | 1096165 | 11132928 |
| 10 |  5 | 89.702       | ~4.176       | 210      | 4931580 | 32948224 |
| 10 |  6 | 90.547       | 3.970        | 210      | 4935530 | 32833536 |
| 10 |  7 | 90.487       | 4.719        | 210      | 4939480 | 32522240 |
| 10 |  8 | 90.858       | 5.477        | 210      | 4943430 | 33218560 |

- DKG dominates memory usage, scaling from ~6 MB (n=3) to ~33 MB (n=10)

- Memory cost grows primarily with number of devices (n), not threshold (t)

- DSG adds negligible memory overhead compared to DKG

- Memory requirements suggest DKG is not suitable for low-power microcontrollers

- Raspberry Pi–class devices are realistic targets, while smaller IoT devices may struggle

# final thoughts on testings

- DKG is significantly slower
    - ~15–20× slower depending on configuration
- Communication dominates DKG (high messages + bytes), while DSG is comparatively lightweight

# IoT shopping

## Large

[Raspberry Pi 5 ](https://www.canakit.com/raspberry-pi-5-1gb.html)
1 GB $45$
2 GB $65$
4 GB $85$
8 GB $125$
16 GB $205$


[Raspberry Pi 4 Model B](https://www.canakit.com/raspberry-pi-4.html)
1 GB $38$
2 GB $55$
4 GB $75$
8 GB $115$

[NVIDIA Jetson Nano Module](https://www.arrow.com/en/products/900-13448-0020-000/nvidia.html)
4 GB $159$

## Medium

[Raspberry Pi Zero W](https://www.canakit.com/raspberry-pi-zero-wireless.html?src=raspberrypi)
Pi Zero Wireless Board Only $15.00$
Pi Zero WH (with Header) Board Only $16.00$
Pi Zero Wireless Basic Kit $39.95$
Pi Zero W Starter MAX Kit $59.95$

[Raspberry Pi Zero 2 W](https://www.canakit.com/raspberry-pi-zero-2-w.html?cid=usd&src=raspberrypi)
Pi Zero 2 W (Board Only) $16.35
Pi Zero 2 W with Pre-Soldered Header (Board Only) $19.60$
Pi Zero 2 W Basic Kit $39.95$
Pi Zero 2 W Starter Kit $64.95$
Pi Zero 2 W Starter MAX Kit $79.95$

## Small

[ESP32-WROOM-32](https://www.amazon.com/ESP-WROOM-32-Development-Microcontroller-Integrated-Compatible/dp/B08D5ZD528?tag=ustxtaddt-20&th=1)

3 pieces $16.79$

[ESP32-S3](https://www.amazon.com/Development-AYWHP-ESP32-S3-DevKitC-WROOM-1-N16R8-Compatible/dp/B0DG8L7MQ9?tag=ustxtaddt-20&th=1)

3 pieces $19.99$

[ESP32-C3 (low power variant)](https://www.amazon.com/ESP32-C3-Development-Bluetooth-Single-Core-Processor/dp/B0D5XYBVKY?tag=ustxtaddt-20&th=1)

3 pieces $11.99$

[Arduino Uno R3](https://www.amazon.com/Arduino-A000066-ARDUINO-UNO-R3/dp/B008GRTSV6?tag=ustxtaddt-20&th=1)

Single $27.60$

[Arduino Nano](https://www.amazon.com/Arduino-A000005-ARDUINO-Nano/dp/B0097AU5OU?tag=ustxtaddt-20)

Single $25.70$

## power meters

[USB-C Power Meter Tester](https://www.amazon.com/YOJOCK-Multimeter-Capacity-Voltmeter-Detector/dp/B0B99Z2GJK/ref=sr_1_7?dib=eyJ2IjoiMSJ9.iybN4vCclQX6xXmsrvw1BcQvo3l7PZwuWdjMlj1JmpX3z1TB2Oh1xwH6C-FWod8otTtXkAzP3CMfo_pVwiZzH750qmmb9zoGFYG40q1s3tXrIddd5s317hIPNh_5yKas4GdWI2_Dashc367g2PNTSJOpc0P6nrok_MsG0hXzNvdHul4pVNaAfTYoz_f3KnADwz8Z1-x_tX9t_BsLHMwLZSEUSkXf3vgwyWYQU8oneo2KsTBueGFqtJxfiJfrf1U23PWPM_a7WTNwV1tuOxr7wFCavMJl87xpAbsXN0ErSGE.vuJYwXse-NXPw8shQ2WeE0fL9Gr0K-j9UxX1DT8RqME&dib_tag=se&keywords=usb%2Bc%2Bpower%2Bmeter&qid=1773892988&sr=8-7&th=1)  $9.97$

[MakerHawk USB Power Meter](https://www.amazon.com/MakerHawk-Multimeter-Voltmeter-Capacity-Resistance/dp/B07DCSNHNB/ref=sr_1_1?crid=ABUC5M80B05J&dib=eyJ2IjoiMSJ9.jWUV7yJtWapaaGdfOR2SqR2f9Eieua-6SgRqorajXvCafvId_HOItvKz51i_fWlsR0sDcxowR-f49cP4PojpiEFmvJY5ZyoJLYVD9ibl-rnIlgJA8mbMZO9QTcJQDNkVOvof0b6JALknE5lyN5N9QjVN3Fdo-WLc1v9AYyYinqd7wtGCRnwmCu7sN9BY98-btGP6CWqUFED0Q9OGXoC_MFsJWOHEbLA6skVc2P5wywEhDjNTgEwAf-pq_FNH7eeeHeWtcsQr5C7781ZpV9TTls0K7vz0ZV_Jw5IJvMiMZdE.7Tqay2v4WKGHjP0Xxvkulo6ChjIYW8tVClDBChTuFGI&dib_tag=se&keywords=MakerHawk%2BUSB%2BPower%2BMeter&qid=1773893053&s=hi&sprefix=makerhawk%2Busb%2Bpower%2Bmeter%2Ctools%2C87&sr=1-1&th=1) $20.99$

[Monsoon Power Monitor - Low Voltage Power Monitor Power Supply](https://www.msoon.com/online-store/Low-Voltage-Power-Monitor-Power-Supply-See-Description-p91565877) $25$

# Multiple signatures for one key gen

## file

examples/sign_repeat.rs
examples/sign_bench.rs

## command

```
cargo build --example sign_repeat
cargo fmt
cargo run --example sign_repeat
```


```
cargo build --example sign_bench
/usr/bin/time -l cargo run --example sign_bench
```

## table mult sign  - DKLS

|  n |  t | Signatures per Keygen | Mode   | DKG Time (s) | Total Sign Time (s) | Avg Sign Time (s) |          Max RSS |
| -: | -: | --------------------: | ------ | -----------: | ------------------: | ----------------: | ---------------: |
|  3 |  2 |                     1 | Fixed  |     2.814252 |            0.578073 |          0.578073 | 13,877,248 bytes |
|  3 |  2 |                     5 | Fixed  |     2.811564 |            2.892892 |          0.578578 | 13,877,248 bytes |
|  3 |  2 |                    10 | Fixed  |     2.820536 |            5.763638 |          0.576364 | 13,877,248 bytes |
|  5 |  3 |                     1 | Fixed  |     5.779648 |            0.822979 |          0.822979 | 13,877,248 bytes |
|  5 |  3 |                     5 | Fixed  |     5.629375 |            4.098759 |          0.819752 | 13,877,248 bytes |
|  5 |  3 |                    10 | Fixed  |     5.619152 |            8.247757 |          0.824776 | 13,877,248 bytes |
|  3 |  2 |                     1 | Random |     2.860074 |            0.580172 |          0.580172 | 13,877,248 bytes |
|  3 |  2 |                     5 | Random |     2.796488 |            2.889829 |          0.577966 | 13,877,248 bytes |
|  3 |  2 |                    10 | Random |     2.830101 |            5.728946 |          0.572895 | 13,877,248 bytes |
|  5 |  3 |                     1 | Random |     5.741547 |            0.821680 |          0.821680 | 13,877,248 bytes |
|  5 |  3 |                     5 | Random |     5.652955 |            4.107222 |          0.821444 | 13,877,248 bytes |
|  5 |  3 |                    10 | Random |     5.639343 |            8.227200 |          0.822720 | 13,877,248 bytes |
