// 17 lines 7 code 6 comments 4 blanks
/*
 * /* Hello! */
 */

/mob
    // I can rely on this file to exist on disk.
    var/some_file = './/dreammaker.dm'

/mob/Login()
    // Not counted. /* */ 
    world << "// Say hello to [src]!"

    src << browse({"
    /*<a href="https://google.com">Link</a>*/
    "}, "window=google")

