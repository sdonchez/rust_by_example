# 3. Custom Types
The "Custom Types" section contains applications that demonstrate the various types of custom datatypes supported by rust, including structures, enums, and constants.

## Contents
See the [top level README](/README.md) for a description of the folder structure contained herein.

|Application|Description|Links|
|---|---|---|
|`Structures`|Explores the use of structs in three varieties for facilitating the storage of complex sets of data - structs comprised of tuples, unit (fieldless) structs, and classic c-style structs.| [src](./src/structures.rs), [bin](./bin/structures), [log](./log/structures.log)|
|`Enums`|A top-level demonstration of rust enums. Rust enums can contain values of any type, and may contain multiple different types.|[src](./src/enums.rs), [bin](./bin/enums), [log](./log/enums.log)|
|`Enums->Use`|A demonstration of the `use` keyword, which allows the ommittal of fully qualified scoping at variable declarations.|[src](./src/enumsUse.rs), [bin](./bin/enumsUse), [log](./log/enumsUse.log)|
|`Enums->C-Like`|A brief example of the use of c-style enums in rust.|[src](./src/enumsCLike.rs), [bin](./bin/enumsUse), [log](./log/enumsUse.log)|
|`Enums->Linked-List`|An implementation of a single-linked list using an enum of two types - either a list (recursive), or `Nil`. Includes functions for prepending, identifying the length, and stringifying the list.|[src](./src/enumsLinkedLists.rs), [bin](./bin/enumsLinkedLists), [log](./log/enumsLinkedLists.log)|
|`Constants`|A (very) brief demonstration of the use of constants, which are either const (fully unchangeable, the most common case), or static (the implications of which aren't obvious).|[src](./src/constants.rs), [bin](./bin/constants), [log](./log/constants.log)|