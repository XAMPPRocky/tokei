# 25 lines 13 code 7 comments 5 blanks
# 
# The context header.
os: windows
os: linux
app: Slack
app: Teams
-

# Activate tag.
tag(): user.tabs

# Adjusts settings.
settings():
    key_wait = 1.5

# Voice commands.
up: key(up)
down: key("down")

insert multiline:
    # multiline string
    """this is a multiline string
    """
