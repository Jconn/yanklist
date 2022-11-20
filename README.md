neovim plugin that stores your yank history and outputs it to a buffer, so you can re-yank things you've yanked before

built off of [nvim-oxi](https://github.com/noib3/nvim-oxi). 

In order to run, build the project

move the resulting `target/debug/libyanklist.so` to `lua/yankpast.so`

add the base of the repo to your nvim runtime path
`set runtimepath+=/hdd/projects/pretty_nvim`

and then to open the yank history window 
`:lua require("yankpast").open_window()`
