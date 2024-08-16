# 29 lines, 21 code, 3 comments, 5 blanks


def add(x, y):
    '''
    Hello World
    # Real Second line
    Second line
    '''
    string = "Hello World  #\
    "
    y += len(string)
    # Add the two numbers.
    x + y


cdef add2(x, y):
    """
    Hello World
    # Real Second line
    Second line

    Note that docstring lines are counted as code
    """

    string = "Hello World"
    y += len(string)
    # Add the two numbers.
    x + y
