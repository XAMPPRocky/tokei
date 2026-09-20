# 11 lines 7 code 2 comments 2 blanks
*** Settings ***
Library     SeleniumLibrary

*** Variables ***
# Setting firefox as browser
${BROWSER}      Firefox

*** Keywords ***
Go To Github Repo
    Open Browser        https://github.com/XAMPPRocky/tokei     ${BROWSER}