# Four Bishops
This program finds solution to the 4-bishops problem (or the bishop swap
problem). The problem states that one starts with 4 while and 4 black bishops
on a 5x4 chess board:

```
+---+---+---+---+---+
| W |   |   |   | B |
+---+---+---+---+---+
| W |   |   |   | B |
+---+---+---+---+---+
| W |   |   |   | B |
+---+---+---+---+---+
| W |   |   |   | B |
+---+---+---+---+---+
```

One has to exchange the place of all the bishops, but at no time should a
bishop threaten any bishop of the opposing color:

```
+---+---+---+---+---+
| B |   |   |   | W |
+---+---+---+---+---+
| B |   |   |   | W |
+---+---+---+---+---+
| B |   |   |   | W |
+---+---+---+---+---+
| B |   |   |   | W |
+---+---+---+---+---+
```

This program solves this problem and find the minimum number of moves that can
achieve this (and prints them out).
