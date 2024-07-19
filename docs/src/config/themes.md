## Themes

`themes.yaml` is where you list your themes. Each one of them has:

- A name
- A list of colors
- Some extra data (optional)

The file can look something like this:

```yaml
- name: "gruvbox"
  colors:
    background: "#141617"
    foreground: "#ddc7a1"
    blue: "#7daea3"
  extra:
    wallpaper: "/home/you/.config/hypr/wallpapers/gruvbox.png"

- name: "catppuccin"
  colors:
    background: "#181724"
    foreground: "#ffffff"
    blue: "#89b4fa"
  extra:
    wallpaper: "/home/you/.config/hypr/wallpapers/catppuccin.png"
```

The keys for both the colors and extra data are up to you. One can define something like the following and proceed with no problems:

```yaml
- name: "gruvbox"
  colors:
    joe: "#141617"
    doe: "#ddc7a1"
  extra:
    foo: "/home/you/.config/hypr/wallpapers/gruvbox.png"
    bar: "Hello world"
```

Be aware that the only difference between `colors` and `extra` for theris is the way they will be showed in the terminal
by the `list` command. Using the same keys can arise unexpected behavior.
