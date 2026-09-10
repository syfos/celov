# 1. Easy go to implementation based search engine  for a function, method, struct or enum.

Though we can grep exact name of the the function, method, struct or even enum by word but this gets painful as you qre navigating in bigger files with multiple occurences of the same word.

Design :

```txt
// cmdline
// :type name

say for a function -> 
:fn wrap

---> cursor lands at the implementation 
```
