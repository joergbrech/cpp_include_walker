# C++ Include Walker

[![Build Status](https://travis-ci.org/joergbrech/cpp_include_walker.svg?branch=master)](https://travis-ci.org/joergbrech/cpp_include_walker)
[![Build status](https://ci.appveyor.com/api/projects/status/u66p43l2g6dqjjk9?svg=true)](https://ci.appveyor.com/project/joergbrech/cpp-include-walker)
[![codecov](https://codecov.io/gh/joergbrech/cpp_include_walker/branch/master/graph/badge.svg)](https://codecov.io/gh/joergbrech/cpp_include_walker)

[**Read the documentation**](https://joergbrech.github.io/cpp_include_walker/)

The idea is to parse all of the `#include` statements in a set of `.cpp`, `.cxx`, `.c`, `.h`, `.hpp`, `.hxx` files, build a *depencency forest* and provide some functionality, such as
 - finding circular dependencies *(either using Johnson's alg, or during topo sort by Kahn's method)*, 
 - finding superfluous `#include` statements *(superfluous in the sense of "include only what you need")*
 - sort header files topologically.
 
In the long run, it would be awesome if this could be used to automatically generate `.i` files for swig. 

## Note

Other tools use a compiler to obtain the AST.

 - **Con**:
 Additional dependency, how would you do this platform-independently? 
 - **Pro**: This would be useful to catch platform dependent includes and not make assumptions about include guards.
