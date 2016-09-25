# Development Environment

## Sublime
For developement I used sublime text.

### Packages

My *Package Control.sublime-settings* (with comments, don't try to use this file directly)

```json
{
    "bootstrapped": true,
    "in_process_packages":
    [
    ],
    "installed_packages":
    [
        "BeautifyRust", - reformat rust code
        "BracketHighlighter", - highlight selected bracket pair
        "Filter Lines", - for searching large files
        "GitGutter", - show changed lines
        "Google Spell Check", - spell checking
        "HexViewer", - for viewing binary files
        "HTML-CSS-JS Prettify", - for reformatting ugly html/css/js files
        "LaTeXing", - latex editing
        "LessImproved", - for editing less files
        "Libsass Build", - for building sass files
        "MarkdownEditing", - for eding markdown files
        "Monokai Extended", - color scheme
        "Package Control", - sublime package manager
        "Pandoc", - converting documents
        "Protocol Buffer Syntax",
        "Restructured Text (RST) Snippets",
        "Rust", - rust editing and compiling
        "RustAutoComplete", - rust autocompletion
        "Schemr", - allow switching color schemes
        "SideBarEnhancements", - more options in the sidebar
        "SublimeGDB", - debugging using gdb
        "GitSavvy", - fantastic git gui
        "Table Editor", - fantastic table editor for md and rst files
        "Terminal", - open a terminal (I had to rebind some keys for this)
        "Theme - Soda", - sublime theme
        "Themr", - allow switching themes
        "TOML" - for editing rust package settings
    ]
}
```

### Build Script
My build script *cargo_run.sublime-build*:

```json
{
    "shell_cmd": "cd $project_path && cargo run"
}

```

### Project File

My default rust project file looks like this (with comments, don't try to use this file directly). *oldnav* is the name of the executable.

```json
{
    "folders":
    [
        {
            "path": "."
        }
    ],

    "settings":
    {
         "sublimegdb_executables": - settings for launching sublimegdb plugin
         {
             "oldnav":
             {
                 "workingdir": "${folder:${project_path:target/debug/oldnav}}",
                 "commandline": "rust-gdb --interpreter=mi ./oldnav",
                 "run_after_init": false,
             },
         }
    }
}
```