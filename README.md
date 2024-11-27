# OhmScript

>OhmScript is a interpreted scripting language to help you calculate the equivalent resistance of multiple resistors.

## Syntax

This is an example OhmScript:

```ohmscript
R1 = 1k
R2 = 220
R3 = 300

? = //(R1,R2, R3)

R4 = 100k
? = //(R2, ->(R3, R4), R3)
```

*This prints `112.62798634812287` and `126.76266707517904`*

There are just a few operators in OhmScript:

- `=`: this is the assign operator. It binds a name to a value.
- `?`: this is a special name, everything assigned to it gets evaluated and printed.
- `//`: this is the parallel operator. `//(R1, R2)` is equivalent to $\frac{R_1 \cdot R_2}{R_1 + R_2}$.
- `->`: this is the series operator. `->(R1, R2)` is equivalent to $R_1 + R_2$.

In OhmScript, instead of writing `A = 225000`, you can use the `k` shorthand: `A=225k`.
