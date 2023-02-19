<<<<<<< HEAD
# game_data.txt

`game_data.txt` contains self-play games on a 10x10 board using the (Wolve)[https://trmph.com/hexwiki/Wolve.html] program.
There are ~2500 games in the database, with more to come. Each game has several dozen moves each, providing thousands of board positions for you to train on.

## IMPORTANT
The first black & white move for each game was auto generated to ensure that the game playouts were sufficiently different.
DO NOT use them when training your model as they are not based on any sort of search or game strategy.

The format of the data is as follows, where each line is different:
`<color> <move>,<color> <move>,...,<color> <move>,<winner>`
=======
# game_data.txt

`game_data.txt` contains self-play games on a 10x10 board using the (Wolve)[https://trmph.com/hexwiki/Wolve.html] program.
There are 9900. Each game has several dozen moves each, providing thousands of board positions for you to train on.

## IMPORTANT
The first black & white move for each game was auto generated to ensure that the game playouts were sufficiently different.
DO NOT use them when training your model as they are not based on any sort of search or game strategy.

The format of the data is as follows, where each line is different:
`<color> <move>,<color> <move>,...,<color> <move>,<winner>`
>>>>>>> dd5dc4677863c72686be2b600e7f7cc34850a1c3
