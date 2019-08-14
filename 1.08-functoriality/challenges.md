1. 
```haskell
instance Bifunctor Pair where
  bimap f g (Pair a b) = Pair (f a) (g b)
```

2. 

```haskell
map_1: Maybe a -> Maybe' a
map_1 Just a -> a
map_1 Nothing -> ()

map_2: Maybe' a -> Maybe a
map_2 a -> Just a
map_2 () -> Nothing

map_1 (map_2 a) = a
map_1 (map_2 ()) = ()

map_2 (map_1 (Just a)) = Just a
map_2 (map_1 Nothing) = Nothing

-- map_1 . map_2 = id
-- map_2 . map_1 = id
-- map_1 and map_2 are inverses
```

3. 
```haskell
instance Bifunctor PreList where
  bimap f g Nil = Nil
  bimap f g (Cons a b) = Cons (f a) (g b)
```

4. 

```haskell
instance Bifunctor (K2 c) where
  bimap: (a -> a') -> (b -> b') -> K2 c a b -> K2 c a' b'
  bimap _ _ (K2 c) = K2 c

instance Bifunctor Fst where
  bimap: (a -> a') -> (b -> b') -> Fst a b -> Fst a' b'
  bimap f g (Fst a) = Fst (f a)

instance Bifunctor Snd where
  bimap: (a -> a') -> (b -> b') -> Snd a b -> Snd a' b'
  bimap f g (Snd b) = Snd (g b)
```

5. see functoriality/src/main.rs
6. ?
