## Summary

Let's play pretend and imagine this is actually NASA.
We can safely assume the input would be mostly correct, considering rovers are very expensive so I'm assuming the input is valid when the rover receives it. I'm assuming the instructions are either loaded onto the rovers or coming from a Buffer/Stream, but as we're writing an implementation for moving the rovers based on a set of instructions, how those instructions are received by the rover is out of scope for a smaller coding test, hence there is no rigorous error handling on the input. The errors are instead boxed and bubbled up to whatever the client implementation might be, which is idiomatic Rust, it'll just gracefully exit instead of hard panic the thread provided the errors are accounted for on a higher level. We're writing code for a rover though, so if an instruction is incorrect, there's probably something really wrong with the higher level implementation that actually sends the input.

About Rust, I chose Rust for this because it's exactly the type of thing that would run on a rover. The binary is just 142kb (if you were to drop the init test case in main.rs), it's also exactly 142 LOC excluding tests.

Also accounted for is world boundary, although I didn't have the time to check whether the initial rover position is within world bounds.

In terms of code style, we're following idiomatic Rust by returning `Result<..., Err>` and we're using structs and enums correctly. Some corners are cut such as making some things public where they maybe shouldn't be, but this is for the sake of finishing the test on time and it doesn't cause any major issues for the scope of this test. We're using `Clone/Copy` a little liberally which I wouldn't do on larger projects in Rust but for a coding test it's fine, in case you aren't familiar with rust, memory management is sort of manual, so you're passing pointer references and these are moved, not cloned. Ideally, when you derive `Clone/Copy`, it should be for types that are smaller than a pointer in bytes. Still though, even though we're implicitly cloning (something things like node and python do constantly anyway), we're still blazing fast because we get to have this much control over memory.

## How to run

It runs on latest stable and nightly Rust releases.

Install rust:
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Or for Windows, check rustup site for installer.

Run `cargo test` in root to run test cases


## Test results

Input:

```
5 5
1 2 N
LMLMLMLMM
3 3 E
MMRMMRMRRM
```
Output:
```
> Executing task: cargo test --package mars-rover --test rover -- test_rover_init --nocapture <


Finished test [unoptimized + debuginfo] target(s) in 0.01s
Running target\debug\deps\rover-1b046641f7729bdd.exe     

running 1 test
Position { x: 1, y: 3 }
North
Position { x: 5, y: 1 }
East
test test_rover_init ... ok
```