# Scrolling 
Must be row based.

## Indexing
Let:

1. so = Scroll offset,
2. cr = Cursor row,
3. bot = botline,
4. rows = bot - so,
4. delta = rows/2,

Note: Denote current by placing 1 at last of so, cr or bot such as: bot1, so1, cr1

## Scrolling for Normal/Visual/Insert -> 

1. <C-d>
2  <C-u>

## Scroll type:
Half screen(rows),

## Scroll down <C-d>:
so1 + delta
cr1 + delta
bot + delta


## Scroll Up <C-d>:
so1 - delta
cr1 - delta
bot - delta

## Hence :

so2 = so1  ± delta
cr2 = cr1  ± delta
bot = bot1 ± delta
