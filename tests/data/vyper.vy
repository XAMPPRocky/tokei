# 49 lines 31 code 10 comments 8 blanks
# ```vyper
# @external
# def main():
#     # Comment
#
#     log Hello(message="Hello World!")
# ```
# pragma version ~=0.4.0

event Hello:
    message: String[32]

owner: public(address)

@deploy
def __init__():
    start: String[32] = "###\""
    # comment
    self.owner = msg.sender

@internal
def _call1():
    return

@internal
def _call2():
    return

@external
def foo(name: String[32]) -> String[64]:
    this_ends: String[32] = "a \"test#."
    self._call1()
    self._call2()
    this_does_not: String[64] = "# nested # phrase \" #"
    return this_does_not

@external
def foobar() -> bool:
    does_not_start: String[64] = "until here, test# test"  # a quote: "
    also_doesnt_start: String[64] = "until here, test,# test"  # another quote: "
    return True

@external
def foo_again() -> uint256:
    a: uint256 = 4  # #
    b: uint256 = 5
    c: uint256 = 6  # #
    return a + b + c
