# Dir Tree

Prints the target directory and its children in a tree-like format.

## Steps

1. Read target directory string from args, or default to `$PWD`
1. Initialize file/dir counter
1. Iterate over all children of the target directory *(Check for hidden files and filter)*
1. Concatenate the child with the directory parent
1. If the file is the last on of its ancestor's children, print the end mark as prefix and recur
1. Otherwise, print the continue mark as prefix and recur
