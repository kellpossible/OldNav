# Building with the x-plane sdk

Building the rust binary into a dynamic library and then linking it with he x-plane sdk is expected to be a major hurdle to overcome.

Some valuable links:

- https://doc.rust-lang.org/book/ffi.html 
- http://doc.crates.io/build-script.html
- https://users.rust-lang.org/t/linking-with-custom-c-library/637/4
- http://www.xsquawkbox.net/xpsdk/mediawiki/BuildInstall
- http://www.xsquawkbox.net/xpsdk/mediawiki/Overview
- https://doc.rust-lang.org/1.1.0/book/rust-inside-other-languages.html
- http://siciarz.net/24-days-of-rust-calling-rust-from-other-languages/
- http://siciarz.net/ffi-rust-writing-bindings-libcpuid/
- https://doc.rust-lang.org/book/advanced-linking.html

*Note: there are no import/link-time libraries for Linux; on Linux, plugins
simply leave SDK symbols undefined and they are discovered at runtime.  The 
SDK website explains this process in more detail.*

Possible Issues with linking:
- https://github.com/rust-lang/rust/issues/32996


## C prototype

I'm going to start by creating a simple C example application which calls
a linked library function and attempts to link another C application with this. Then I'll try the rust equivalent.

Then after I've learned a lot doing this, I'll move on to the far more complicated x-plane sdk.


## Process

So it looks like I might need to create a build.rs script. I'm unsure if this is only needed if I want to link another library with this one to run or whether it is also required if I want the rust code to compile as a dynamic library.