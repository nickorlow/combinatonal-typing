# Combinational Typing

Type without leaving the home row!

## How?

This works by using modifier keys to remap other keys on the fly. This is similar 
to how the shift key works. Pressing any key in the home row will type like normal. 
If you hold down A and then press a key on the opposite side of the home row, it will 
be mapped to the key above it. For example if you do: `press down A, press down L, release 
L, release A`, then O will be typed. The full list of remappings are:

```
    If you hold A, then these keys are remapped H -> Y, J -> U, K -> I, L -> O, ; -> P
    If you hold S, then these keys are remapped J -> B, K -> N, L -> M
    If you hold ;, then these keys are remapped A -> Q, S -> W, D -> E, F -> R, G -> T
    If you hold L, then these keys are remapped A -> Z, S -> X, D -> C, F -> V
    EXAMPLE: H, ;+D, L, L, A+O       PRINTS: HELLO
```

Backspace and Space work as usual. No other keys work.
