# 13 lines 8 code 3 comments 2 blanks

package application.authz

# Only owner can update the pet's information
# Ownership information is provided as part of OPA's input
default allow = false
allow {
    input.method == "PUT"
    some petid
    input.path = ["pets", petid]
    input.user == input.owner
}
