# asciibar

*Print an ascii progress bar displaying a percentage to stdout.*

## Possible input designs

``` sh
asciibar 0.5
asciibar --length 10 -- 0.5       # length of 10 characters (default)
asciibar --min 10 --max 90 -- 50  # relative value to min and max instead of percentage
asciibar --border="|" -- 0.5
```

## Possible output style options designs
``` sh
asciibar --char-filled="█" --char-half="▌" --char-empty=" " -- 0.55
█████▌    

asciibar --char-filled="█" --char-half="▌" --char-empty="░" -- 0.55
█████▌░░░░

asciibar --char-filled="█" --char-empty="░" -- 0.5
█████░░░░░

asciibar --char-filled="█" --char-empty="▒" -- 0.5
█████▒▒▒▒▒

asciibar --char-filled="█" --char-empty="▓" -- 0.5
█████▓▓▓▓▓

asciibar --char-filled="#" --char-empty="=" -- 0.5
#####=====

asciibar --char-filled="#" --char-empty="-" -- 0.5
#####-----

asciibar --char-filled="=" --char-half=">" --char-empty="-" -- 0.55
=====>----

# provide (several) default styles
asciibar 0.55
█████▌    

asciibar --borders="|" -- 0.5
|█████▌    |
```
