# C++ Include Walker

Its probably really stupid to write a C++ dependency walker with rust, but hey!

The idea is to parse all of the `#include` statements in a set of `.cpp`, `.cxx`, `.c`, `.h`, `.hpp`, `.hxx` files, build a *depencency forest* and provide some functionality, such as
 - finding circular dependencies, 
 - finding superfluous `#include` statements
 - sort header files topologically.
 
In the long run, it would be awesome if we could automatically generate `.i` files for swig using this.