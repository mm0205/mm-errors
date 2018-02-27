
# mm-errors

Provides functions for error handling.

## Examples

Error chaining.

```rust

#[macro_use] extern crate mm_errors;

use std::result::Result;

use mm_errors::Error;

fn level1() -> Result<u32, Error> {
    let _ = try_wrap!("XXX".parse::<u32>());
    panic!("This line is unreachable");
}

fn level2() -> Result<u32, Error> {
    let v = try_wrap!(level1());
    Ok(v)
}

fn level3() -> Result<u32, Error> {
    let v = try_wrap!(level2());
    Ok(v)
}

fn main() {

    match level3() {
        Err(e) => println!("{}", e),
        Ok(_) => panic!("the function should panic!"),
    };

}

```

The above code outputs following.

```xml
<error>
   <file>src\lib.rs</file>
   <line>20</line>
   <reason>
   <error>
       <file>src\lib.rs</file>
       <line>15</line>
       <reason>
           <error>
               <file>src\lib.rs</file>
               <line>10</line>
               <reason>invalid digit found in string</reason>
           </error>
       </reason>
   </error>
   </reason>
</error>
```

Converts `Option::None` to `Result::Err`.

```rust

#[macro_use] extern crate mm_errors;

use std::result::Result;

use mm_errors::Error;

fn level1() -> Result<u32, Error> {
    let mut v = vec![];
    let _ = try_opt!(v.pop(), "Stack underflow!");
    panic!("This line is unreachable");
}

fn level2() -> Result<u32, Error> {
    let v = try_wrap!(level1());
    Ok(v)
}

fn level3() -> Result<u32, Error> {
    let v = try_wrap!(level2());
    Ok(v)
}

fn main() {

    match level3() {
        Err(e) => println!("{}", e),
        Ok(_) => panic!("the function should panic!"),
    };

}

```

The above code outputs following.

```xml
<error>
    <file>src\lib.rs</file>
    <line>21</line>
    <reason>
    <error>
     <file>src\lib.rs</file>
     <line>16</line>
     <reason>
         <error>
             <file>src\lib.rs</file>
             <line>11</line>
             <reason>Stack underflow!</reason>
         </error>
     </reason>
    </error>
    </reason>
</error>

```

Creates simple error with error message.

```rust

#[macro_use] extern crate mm_errors;

use std::result::Result;

use mm_errors::Error;

fn return_error() -> Result<u32, Error> {
    return new_result!("This function always returns error");
    panic!("This line is unreachable");
}

fn main() {

    match return_error() {
        Err(e) => println!("{}", e),
        Ok(_) => panic!("the function should panic!"),
    };
}

```

The above code outputs following.

```xml
<error>
    <file>src\lib.rs</file>
    <line>10</line>
    <reason>This function always returns error</reason>
</error>
```
