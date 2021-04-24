### 1. Clone it
```
git clone --depth=1 git@github.com:memononen/nanosvg.git
```

### 2. Compile example 2

```
cd nanosvg/example
gcc ./example2.c -o test -I ../src -lm
```

* `-I ../src` Include the NanoSVG source files, so `#include "nanosvg.h"` doesn't fails.
* `-lm` Include the math library.

### 3. Render the image
```
ls
./test
```

`svg.png` should now be on the disk.

```
open svg.png
```