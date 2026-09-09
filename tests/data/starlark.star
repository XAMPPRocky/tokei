# 20 lines 13 code 3 comments 4 blanks

# Load a dependency
load("@rules_go//go:def.bzl", "go_binary")

# Define a build rule
def my_binary(name, srcs):
    go_binary(
        name = name,
        srcs = srcs,
        deps = [
            "//lib:mylib",
        ],
    )

"""
This is a docstring.
"""

my_binary(name = "hello", srcs = ["main.go"])
