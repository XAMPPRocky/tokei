::+ 22 lines 12 code 4 comments 6 blanks

@if not defined DEBUG (echo off)
@setlocal DisableDelayedExpansion EnableExtensions
@goto :main

echo /? Line of code - NOT

::+ Print an argument and add a new line to the output
:println #[io] (string = "")
    ::: Echo content to stdout
    echo(%~1

    goto :EOF &@rem Do not set exit code

:main
    set "var=Hello world" This is an inline comment that does not get recognized

    @rem Tokenize contents of the variable
    for /f "usebackq tokens=1,2" %%i in ('%var%') do (
        call :println "%%~i, %%~j!" 2>nul
    )
