# Tagged Unions

The format Rust uses to lay out enums in memory is currently unspecified, as 
such in order to use Rust unions in a FFI context you need to go via some sort
of tagged union. This crate exposes a custom derive for making this process more
ergonomic.
