set global lsp_cmd "kak-lsp -s %val{session} -vvv --log kak-lsp.log"

hook global WinSetOption filetype=html %{
    set-option window lsp_servers %{
       [htmx-language-server]
       filetypes = ["html"]
       command = "bash"
       args = ["-c", "(cat stdout &);(cat > stdin)"]
       root_globs = ["*.html"]
    }
}
