# 1. Hello World
The "Hello World" section contains applications that explore the basic concepts of comments and formatted output.

## Contents
See the [top level README](/README.md) for a description of the folder structure contained herein.

|Application|Description|Links|
|---|---|---|
|`Hello`|The ubiquitous "Hello World" application (plus a second line).| [src](./src/hello.rs), [bin](./bin/hello), [log](./log/hello.log)|
|`Comments`|A demonstration of difference between block and line comments.|[src](./src/comments.rs), [bin](./bin/comments), [log](./log/comments.log)|
|`Formatted Print`|Examples of the various ways that `std:fmt` can be used to display formated numbers within the `println!` macro.|[src](./src/formattedPrint.rs), [bin](./bin/formattedPrint), [log](./log/formattedPrint.log)|
|`Debug`| A demonstration of the `Debug` trait, which allows for no-frills printing of types without any further work setting up the formatter.|[src](./src/debug.rs), [bin](./bin/debug), [log](./log/debug.log)|
|`Display`| An introduction to implementing the `Display` trait, which allows the implementer to customize the format of output for a type.|[src](./src/display.rs), [bin](./bin/display), [log](./log/display.log)|
|`List`| Demonstrates how to use the `Display` trait with lists to iterate through elements.|[src](./src/list.rs), [bin](./bin/list), [log](./log/list.log)|
|`Formatting`| Demonstrates the use of various formatters with the `Display` trait to allow hexadecimal and other number formats.|[src](./src/formatting.rs), [bin](./bin/formatting), [log](./log/formatting.log)|