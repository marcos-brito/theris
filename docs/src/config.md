# Configuration

Theris needs at least two files to work properly:

- `themes.yaml`
- `appliers.yaml`

The default path for both is `~/.config/theris`, but you can provide another using the CLI.
You can also create a `templates` directory within the same path to define your templates.

As theris modifies some possibly important files, it optionally creates backups using `tar` and `gunzip` to keep
you safe. For that you'll need to create `~/.local/share/theris/backup`.

> Some warns will pop on your terminal if something is missing

If you are on a hurry, copy and paste this on your terminal:

```sh
mkdir -p ~/.config/theris/templates ~/.local/share/theris/backup &&
touch ~/.config/theris/themes.yaml ~/.config/theris/appliers.yaml
```
