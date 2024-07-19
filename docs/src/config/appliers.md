## Appliers

`appliers.yaml` tells theris how to apply the theme for different applications. The file will look like this:

```yaml
- name: kitty
  path: /home/you/.config/kitty/theme.conf
  method: !Template
    template: kitty

- name: eww
  path: /home/you/.config/eww/eww.scss
  method: !Delimiter
    template: eww
    start: "// theris start"
    end: "// theris end"

- name: tmux_hook
  path: ""
  method: !Script
    path: "/home/you/.config/theris/tmux.sh"
```

Every key is needed. Here is a description for each:

- `name`: The applier's name. It's relevant when using the CLI.
- `path`: It's the file that will be modified and consequently the file to be in the backup.
- `method`: This is what actually tells theris how to apply the theme. There are a few options. Each one is described right ahead.

### !Template

It overwrites the whole file specified in `path`.

#### Parameters

- `template`: The file name of the template inside the `templates` directory

### !Delimiter

It overwrites a portion of the file specified in `path` using delimiters. The delimiters are searched line by line and surrounding white space
is ignored.

#### Parameters

- `template`: The file name of the template inside the `templates` directory or a inline template.
- `start`: A string that matches the start of the portion to be replaced
- `end`: A string that matches the end of the portion to be replaced

### !ReplaceText

It replaces a text pattern in `path` with another.

#### Parameters

- `target`: A regular expression that matches the pattern to be replaced
- `replacment`: The text that will replace `target`. It can also be a inline template.

### !Script

It runs a external executable. The theme being applied will be passed as a `json` to `stdin`.

#### Parameters

- `path`: The path to the executable file
