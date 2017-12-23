# Tagged Unions

[![Travis build status](https://travis-ci.org/Michael-F-Bryan/tagged_unions.svg?branch=master)](https://travis-ci.org/Michael-F-Bryan/tagged_unions)
[![Appveyor build status](https://ci.appveyor.com/api/projects/status/i3gxy3o7pxqiux5n?svg=true)](https://ci.appveyor.com/project/Michael-F-Bryan/tagged-unions)



The format Rust uses to lay out enums in memory is currently unspecified, as 
such in order to use Rust unions in a FFI context you need to go via some sort
of tagged union. This crate exposes a custom derive for making this process more
ergonomic.
