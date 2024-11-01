# 4. Variable Bindings
The "Variable Bindings" section contains applications that explore variable mutability and scope.

## Contents
See the [top level README](/README.md) for a description of the folder structure contained herein.

|Application|Description|Links|
|---|---|---|
|`Variable Bindings`|Provides an introduction to variable declarations and definition, as well as rustc compiler warnings about unused variables and means of suppressing them.| [src](./src/variable_bindings.rs), [bin](./bin/variable_bindings), [log - warning](./log/variable_bindings_unfixed_rustc_warn.log) [log - fixed](./log/variable_bindings.log)|
|`Scope`|Introduces the concept of variable scopes, wherein bindings persist only within a block internal to a function|[src](./src/scope.rs), [bin](./bin/scope), [log - error](./log/scope_unfixed_rustc_error), [log - fixed](./log/scope.log)|
|`Variable Shadowing`|Introduces the concept of variable shadowing, wherein a variable can  be re-bound within a nested scope of the scope of its initial binding, or within the same scope.|[src](./src/variable_shadowing.rs), [bin](./bin/variable_shadowing), [log](./log/variable_shadowing.log)|
|`Declare First`|A brief example of separation of variable declaration and initialization, and rustc's ability to detect attempts to use uninitalized variables at compile time.|[src](./src/declare_first.rs), [bin](./bin/declare_first), [log - error](./log/declare_first_rustc_error.log), [log - fixed](./log/declare_first.log)|
|`Freezing`|Demonstrates the concept of variable "freezing", in which a mutable variable is shadowed by itself, but without the `mut` modifier, which prevents modification within the shadowed scope.|[src](./src/freezing.rs), [bin](./bin/freezing), [log - error](./log/freezing_rustc_error.log), [log - fixed](./log/freezing.log)|