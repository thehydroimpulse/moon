# Moon [![status](http://img.shields.io/badge/stage-alpha-orange.svg)]() [![version](http://img.shields.io/badge/version-0.0.2-blue.svg)]()

Moon is a small Lisp-dialect written purely in Rust. This should allow Rust programs (including games) to have access to a high-quality scripting language that's also safe. Moon is made to work extremely well with Rust or even C.

===

```lisp
(defn shoot
    [player bullet]
    ...)

;; The core render loop for the scripting engine.
;; On every tick, draw the ship.
(defn render
    [inter]
    (draw-ship inter))
```

![Repl](../master/repl.png?raw=true)

![Repl 2](../master/repl2.png?raw=true)

## Installing

Clone the repo:

```
git clone git@github.com:TheHydroImpulse/moon.git
cd moon/
```

Build the project:

```
make moon
```

Access the Repl:

```
./target/moon
```

## Testing

```
make test
```

## License

The MIT License (MIT)

Copyright (c) 2014 Daniel Fagnan <dnfagnan@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.

