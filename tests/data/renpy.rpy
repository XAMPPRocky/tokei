# 32 lines 8 code 9 comments 15 blanks

# Declare characters used by this game. The color argument colorizes the
# name of the character.

define e = Character("Eileen")


# The game starts here.

label start:

    # Show a background. This uses a placeholder by default, but you can
    # add a file (named either "bg room.png" or "bg room.jpg") to the
    # images directory to show it.

    scene bg room


    show eileen happy

    # These display lines of dialogue.

    e "You've created a new Ren'Py game."

    e 'Once you add a story, pictures, and music, you can release it to the world!'

    e `Testing, testing`

    # This ends the game.

    return
