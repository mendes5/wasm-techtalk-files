# What Is WebAssembly?
    Bytecode format
    Virtual machine


## Why?
        * JS is too slow for very large projects

        UNITY TARGET: https://beta.unity3d.com/jonas/AngryBots/
        GOOGLE EARTH: https://earth.google.com/web/
        MORE: https://madewithwebassembly.com/
        AUTO FUCKIN CAD https://web.autocad.com/acad/me/drawings/new/editor
        SKETCHUP https://app.sketchup.com/app?hl=en#
        Compression/Decompression https://github.com/drbh/wasm-flate
        FIGMA https://www.figma.com/blog/webassembly-cut-figmas-load-time-by-3x/

# Before (this presentation)
This is not a tutorial
We will not be viewing the webassembly API

This is not a comparation to JS
They both don't really compare... they are better used in their own use cases

# Before (general)
WebAssembly does not replace JS
WebAssembly is only faster than JS if used correctly
Even tough webassembly is intended to be used in webbrowsers, the WebAssembly WV can be run standalone anywhere
 

## How it works?
    What is code?
        The Language Sandbox
        Breaking the sandbox
        Safe execution contexts
        Games and shit

    What is a VM? Isnt this just Java?

    But what about JS








Objective:
Understand the concepts, how it works, use cases


# Demo! Calling C functions from JS without tool chain
https://webassembly.studio/

Simple stuff
Return constant
Call WASM from JS
Call JS from WASM

# Demo! Writting the STDLIB from scratch!
Try to return the value to JS
Fail
Use malloc 
Fail
Implement the malloc
Fail
Fetch the data using dataviews

# Demo! Step-By-Step C Library Setup and Compare

Step 1 NanoSVG
 * Show NanoSVG
    What it is
    Why i choose it
        Already used it before
        It is quite simple of a library
        Use only lib c and stb_image_write

 * Compile it!
 * Add Benchmark code

Step 2 EEMC modifications
 * Jsut compile the glue code and see what happens
 * `#include "emscripten.h"` && `EMSCRIPTEN_KEEPALIVE` Rename main, call via js

Step 3 set up the actual filesystem
 * Rename files
 * Create virtual FS
 * Add flags
 * JS api for the image

Step 4 optimize
 * Fix memory leak
 * Use OPT flag

TODO:: Benchmarks

# Demo! Rust Demo

# Debugging
Literally impossible

# Actual Applications
