# C++ Include Walker

The idea is to parse all of the `#include` statements in a set of `.cpp`, `.cxx`, `.c`, `.h`, `.hpp`, `.hxx` files, build a *depencency forest* and provide some functionality, such as
 - finding circular dependencies, 
 - finding superfluous `#include` statements *(superfluous in the sense of "include only what you need)*
 - sort header files topologically.
 
In the long run, it would be awesome if we could use this to automatically generate `.i` files for swig using this. 

## Note

Other tools use a compiler to print aut the AST.

 - **Con**:
 Additional dependency, how would you do this platform-independently? 
 - **Pro**: This would be useful to catch platform dependent includes and not make assumptions about include guards.