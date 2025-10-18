# 12 lines 9 code 1 comments 2 blanks
*** Settings ***
Resource    robot.resource

*** Variables ***
${TAB TEXT}         "Pull requests"

*** Test Cases ***
Go To Pull Requests
    Wait Until Element Is Visible       xpath=//a[contains(text(), "${TAB TEXT}")]
    Click Element                       xpath=//a[contains(text(), "${TAB TEXT}")]
    Wait Until Element Is Visisble      xpath=//span[text() = "New pull request"]