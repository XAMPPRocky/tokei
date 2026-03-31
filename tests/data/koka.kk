// 29 lines 11 code 12 comments 6 blanks

/*---------------------------------------------------------------------------
  Copyright 2020-2021, Microsoft Research, Daan Leijen.

  This is free software; you can redistribute it and/or modify it under the
  terms of the Apache License, Version 2.0. A copy of the License can be
  found in the LICENSE file at the root of this distribution.
---------------------------------------------------------------------------*/

/* Run processes.
*/
module std/os/process

import std/os/path

extern import {
  c file "process-inline.c"
}

// Run a command in the shell and return its output as a string.
pub extern run-system-read( cmd : string ) : io error<string> {
  c "kk_os_run_command_error"
}

// Run a command in the shell
pub extern run-system( cmd : string ) : io int {
  c "kk_os_run_system_prim"
}
