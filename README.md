# OldNav [![Build Status](https://travis-ci.org/kellpossible/oldnav.svg?branch=master)](https://travis-ci.org/kellpossible/oldnav) [![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)



An aviation navigation library. This library was created for developing old
style navigation systems in x-plane with the fantastic
[xplane_plugin](https://github.com/samcrow/rust-x-plane-plugin) crate,  but it
could be used for a variety of other geospatial purposes. A number of the
features will probably eventually be broken out into their own projects if they
mature. Everything is still very much WIP as it has yet to be used in a
serious plugin.

[Documentation](https://kellpossible.github.io/oldnav/oldnav_lib/navdata/index.html)

## Features
+ [Navigation database](https://kellpossible.github.io/oldnav/oldnav_lib/navdata/database/index.html) - capable of loading x-plane's GNS430 data. Optimised for typical
FMS queries.
+ [Integer based geohash](https://kellpossible.github.io/oldnav/oldnav_lib/navdata/geohash/index.html)
+ [Flight plan routes](https://kellpossible.github.io/oldnav/oldnav_lib/navdata/route/struct.Route.html), route statistics, route finding.

## Initial Developement Plan

- [ ] parse/load all information in the GNS430 database
- [ ] basic database queries
- [ ] query acceleration structure based on geohash in a method similar to what is suggested [here](http://gis.stackexchange.com/a/92331)
- [ ] drawing in opengl in x-plane possibly using [nanovg](https://github.com/KevinKelley/nanovg-rs)
- [ ] flight plan parsing (various formats)
- [ ] route finding
- [ ] route statistics
- [ ] sample plugin

## Motivations

### OldNav?

I've a fascination with older navigation tools and systems which require much
more interaction from the navigation and offer more learning, and interactive
content at the expense of convenience and accuracy. I'm developing this library
for use in a few potential projects:

+ [Omega](https://en.wikipedia.org/wiki/Omega_(navigation_system)) navigation system simulation
+ A comprehensive [KNS660](http://www.herrmanninc.com/Herrmanninc/Resume/kns660.htm) simulation
+ A celestial navigation simulation
+ A seperate map application (possibly embedded in x-plane too if this is sensible)
  with plotting tools similar to the [silent hunter](http://www.subsim.com/subsim_files/images05/nomobig.jpg), and with several useful
  navigation map projections
+ Aircraft systems for addon aircraft

### Rust?

Having spent hours getting a basic x-plane c++ plugin to compile, it was a
breath of fresh air to discover the great work of samcrow on the [xplane_plugin](https://github.com/samcrow/rust-x-plane-plugin) crate, which
makes it dead easy to get up and running with x-plane plugin creation.

A common source of bugs and frustration with x-plane plugins are crash to
desktop from segfaults in plugins. Halfway through a 7 hour flight, there's
nothing more annoying. Rust's inherent safety is a potential solution
to this problem, and its performance should put it at an advantage
to the existing lua plugin tools.
