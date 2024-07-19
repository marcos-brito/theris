# Templates

The templates are read from `~/.config/theris/templates` by default. As you'll see in [Appliers](./appliers.md),
it's also possible to use inline templates in `appliers.yaml`.

Theris uses a template engine named **Tera**. It has various features but you probably need just a basic usage. Here is
an example:

```
bg = {{background}}
fg = {{foreground}}
accent = {{blue}}
font = {{font}}
```

Everything defined in the `colors` or `extra` field can be referenced directly using `{{}}`. For more complex rendering
you should check [Tera's documentation](https://keats.github.io/tera/docs/).
