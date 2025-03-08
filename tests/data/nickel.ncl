# 21 lines 15 code 3 comments 3 blanks

# A comment
let log = m%"
if log:
    print("log:", s)
"% in

{
    # A list with normal strings
    args = ["-c", "-v"],

    derivations = nix-s%"
      pkgs.writeShellApplication {
        runtimeInputs = with pkgs; [ python3 ];
          text = ''
            %{log}
          '';
      };
    "%,
}
