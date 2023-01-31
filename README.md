# percviz (CLI Percentage Visualizer)

Print a (static) progress bar for a specified percentage to stdout.

## Possible input designs

``` sh
percviz 0.5
percviz --length 10 -- 0.5       # length of 10 characters (default)
percviz --min 0 --max 100 -- 50  # relative value to min and max instead of percentage
percviz --border="|" -- 0.5
```

## Possible output style options designs
``` sh
percviz --char-full="█" --char-half="▌" --char-empty=" " -- 0.55
█████▌    

percviz --char-full="█" --char-half="▌" --char-empty="░" -- 0.55
█████▌░░░░

percviz --char-full="█" --char-empty="░" -- 0.5
█████░░░░░

percviz --char-full="█" --char-empty="▒" -- 0.5
█████▒▒▒▒▒

percviz --char-full="█" --char-empty="▓" -- 0.5
█████▓▓▓▓▓

percviz --char-full="#" --char-empty="=" -- 0.5
#####=====

percviz --char-full="#" --char-empty="-" -- 0.5
#####-----

percviz --char-full="=" --char-half=">" --char-empty="-" -- 0.55
=====>----

# provide (several) default styles
percviz 0.55
█████▌    

percviz --borders="|" -- 0.5
|█████▌    |
```

## TODO
[ ] think about renaming `--char-full` to `--ascii-full` if we just use ascii anyways (do we use unicode?)
