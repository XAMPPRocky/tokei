// 26 lines 16 code 6 comments 4 blanks
package GSharp.Example.Tokei

import System

/* A small greeter class that builds
   a greeting from the stored name. */
class Greeter(name string) {
    /// Returns a personalized greeting.
    func Greet() string {
        // Interpolation is sigil-free in G#.
        return "Hello, $name!"
    }

    func Query() string {
        // Raw strings span lines and keep \ and // literally.
        return `
            SELECT *
            FROM people // not a comment
        `
    }
}

let g = Greeter("world")
Console.WriteLine(g.Greet())
Console.WriteLine(g.Query())
