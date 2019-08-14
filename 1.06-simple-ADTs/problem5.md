```
a + a === Either a a === (pair, bool) === 2 * a
```

Proof that `Either a a === (bool, a)`
The following isomorphism works:

```
f :: Either a a -> (bool, a)
f (Left x) = (true, x)
f (Right x) = (false, x)

g :: (bool, a) -> Either a a
g (true, x) = Left x
g (false, x) = Right x
```
