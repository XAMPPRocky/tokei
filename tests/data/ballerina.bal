// 40 lines 29 code 6 comments 5 blanks

# Returns Bob's response to someone talking to him.
#
# + input - whatever is said to Bob
# + return - Bob's response
public function hey(string input) returns string {
    string trimmed = input.trim();
    boolean silent = isSilence(trimmed);
    boolean asking = isQuestion(trimmed);
    boolean yelling = isYelling(trimmed);

    match [silent, yelling, asking] {
        [true, _, _] => {
            return "Fine. Be that way!";
        }
        [_, true, true] => {
            return "Calm down, I know what I'm doing!";
        }
        [_, true, false] => {
            return "Whoa, chill out!";
        }
        [_, false, true] => {
            return "Sure.";
        }
        _ => {
            return "Whatever.";
        }
    }
}

isolated function isSilence(string input) returns boolean => input.length() == 0;

isolated function isQuestion(string input) returns boolean => input.endsWith("?");

function isYelling(string input) returns boolean {
    // contains an uppercase letter and does not contain a lowercase letter
    return input.includesMatch(re `\p{Lu}`)
        && !input.includesMatch(re `\p{Ll}`);
}
