---
aliases: 
title: Adding <code>--options</code> to Just recipes
slug: adding-options-to-just-recipes
clean_title: Adding --options to Just recipes
status: published
published: 2024-09-13 00:00
tags:
  - just
  - javascript
created: 2024-09-13T09:06:19-04:00
modified: 2024-09-13T10:11:33-04:00
excerpt: Adding --options to Just recipes to control execution flow
---
# Adding `--options` to Just Recipes

I've been using [Just](https://just.systems) a lot lately to to help run tasks in my various personal and work projects, and wanted to add options to a Just recipe to further control it.

My use case was to make running [Drush](https://www.drush.org/13.x/) on many remote [Pantheon](https://pantheon.io/) environments a bit easier. I wanted to add a `--tag` option to Drush that would execute Drush commands, via [Terminus](https://docs.pantheon.io/terminus), on many sites with the passed tag. The Drush commands should then run in parallel, and optionally sequentially when necessary.

For example, `just drush --tag tag-name --env dev status` to run status on all of the dev sites tagged with `tag-name`.

There's no built-in way to do this with Just currently, but Just is flexible enough to be able to implement the functionality within a recipe.

This is what I came up with:

```makefile
red := `tput setaf 1`
normal := `tput sgr0`
bold := `tput bold`
error := bold + red + "ERROR:" + normal
self := "just --justfile '" + justfile() + "'"
parallel := "parallel --shebang --tag --linebuffer --jobs " + env('JOB_COUNT', num_cpus()) + " -r"
pantheon_org := "the-name-of-the-pantheon-org"

# Run Drush in parallel on site envs, e.g. `just drush -t tag-name -d dev status`. Use `-s` to run sequentially. [options: --tag|-t, --env|-e, --seq|-s]
[positional-arguments]
@drush *args:
    eval "$({{ self }} _build-taggable-command _drush {{ args }})"

# Get a list of sites tagged with `tag`
@sites-with-tag tag:
    echo "$(terminus org:sites "{{ pantheon_org }}" --tag="{{ tag }}" --format=list --field=name)"

# Get a list of sites tagged with `tag` with .env appended, e.g. brownu-dcloud.dev
@site-envs-with-tag tag env="dev":
    sites="$({{ self }} sites-with-tag "{{ tag }}")"; \
    sites="$(echo "$sites" | sed 's/$/.{{ env }}/')"; \
    echo "$sites"

_parallel command args:
    #!/usr/bin/env -S {{ parallel }} {{ command }}
    {{ args }}

@_terminus sites +args: (_parallel ("terminus -- " + args) sites)

@_drush sites +args: (_terminus sites "drush {}" args)
@_drush-by-tag tag env +args:
    sites="$({{ self }} site-envs-with-tag "{{ tag }}" "{{ env }}")"; \
    {{ self }} _drush "$sites" {{ args }}

@_ensure-npm-dependency package:
    if ! npm list -g --depth 1 "{{ package }}" > /dev/null; then npm install -g "{{ package }}"; fi

export NODE_PATH := `npm root -g`
[no-exit-message]
[positional-arguments]
_build-taggable-command recipe *args: (_ensure-npm-dependency "minimist")
    #!/usr/bin/env node
    const argv = require("minimist")(process.argv.slice(3), {
        alias: { seq: "s", tag: "t", env: "e" },
        boolean: ["s", "seq"],
    });

    let command = "";
    if (argv.seq) {
        command += "JOB_COUNT=1 ";
    }
    command += "{{ self }} {{ recipe }}";

    if (argv.tag && argv.env) {
        command += `-by-tag ${argv.tag} ${argv.env}`;
    } else if (argv.tag && !argv.env) {
        console.error("{{ error }} You must use --env|-e when using --tag|-t");
        process.exit(1);
    } else if (!argv.tag && argv.env) {
        console.error("{{ error }} --env|-e is only applicable when using --tag|-t");
        process.exit(2);
    }

    command += ` ${argv._.join(" ")}`;

    console.log(command);
```

Basically, my `drush` recipe runs `just --justfile <the file> _build-taggable-command` which will parse the command line options passed to it, and return a `just` command to run the proper recipe depending on what options were passed.

If `--tag` and `--env` are passed, we append `-by-tag <tag> <env>` to the incoming recipe name (`_drush`). So running `_build-taggable-command _drush --tag the-tag --env dev` would output `just --justfile <the justfile path> _drush-by-tag the-tag dev`. If you hadn't passed `--tag` and `--env`, it would use the `_drush` recipe instead of the `_drush-by-tag` recipe. So now these options can determine which recipe gets used in the end.

`_drush-by-tag` will use other recipes to generate a list of site names with the environment appended, and then run `terminus drush <site>.<env> -- <the drush command to run>` in parallel on all the environments in the tag.
