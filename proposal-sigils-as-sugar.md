# High-level overview

We want to move @T syntax into a system library, e.g. GC<T> and we
want to add a placement-new operator syntax.  During a recent
discussion of placement-new syntax [1], 


# Assumptions

* I assme some form of placement new syntax.
  * Here I assume we reserve `new` and introduce `new (Expr) Expr`
  * e.g. lets assume `let a = new (value) Struct(args, ...)` expands
    to something like:
    ```rust
       let a = { let mem_address = value.malloc(sizeof::<Struct>());
                 Type::construct(mem_address, args, ...) };
    ```
    where I am deliberately hand-waving above; the details of that
    expansion are not terribly important.

    (The only reason I mention a particular expansion is that I
     believe we'll want in general for placement-new to take a value
     with certain methods as the allocator argument, and so I want to
     keep in mind our the sugar interacts with (or sidesteps) that.)
    

# Issues

* What scope does sugar have?
  * e.g. file, versus module, (versus module-and-its-children, versus crate)
  
* Is expansion hygenic or non-hygenic?
  * e.g. when one writes `use @ for GC`, is that associating
    `@` with a path `GC` resolved at the module level

# References

[1]: https://github.com/mozilla/rust/wiki/Meeting-weekly-2013-10-29

