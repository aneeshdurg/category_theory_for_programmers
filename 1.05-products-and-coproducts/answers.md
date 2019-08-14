1. Suppose a, b terminal

```
  f
a -> b
^    |
|___/
  g
```

since fog = idA and gof=idB, f = g^-1 so a,b are isomorphic.

2. The minimum
3. The maximum
4. see `either/src/main.rs`
5. 
```
Int -(Left)-> Either <-(Right)- Bool
|                |                |
\                | f              /
i\               v               / j
  ------------> Int <------------
```


Where:
```
f (Left x) =  i x
f (Right y) = j y
```

6. i and j can't be reversed so can't be factorized
7. It's isomorphic to Either
8. Bool with `f x:int -> false`, `g b:Bool -> b`.
