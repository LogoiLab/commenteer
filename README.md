# commenteer

Need to add a header to all of your code files that specifies info about you or the program? This tool can do it for you, without the need for tons of copy-pasting.

# Warning this program is currently just a personal project. It may eat the files you tell it to modify. As such make a commit before using this tool.

## Usage

```
Commenteer 0.2.0-1
Chad Baxter <cbaxter@mail.umw.edu>
Add comment headers to code.

USAGE:
    commenteer [FLAGS] [OPTIONS] --input <input>...

FLAGS:
    -h, --help       Prints help information
    -r, --recurse    Add header to every text file in the specified path.
    -V, --version    Prints version information
    -v, --verbose    Show verbose information about the operation

OPTIONS:
    -c, --comment-file <comment-file>    A path to a file containing the header you would like to add.
    -t, --comment-text <comment-text>    A string containing the header you would like to add. Can contain control
                                         characters that use backslashes(\n, \t etc).
    -n, --ignore <ignore>...             Paths to files you would like to exclude from being modified
    -i, --input <input>...               List of paths or files to add a header to.

```
