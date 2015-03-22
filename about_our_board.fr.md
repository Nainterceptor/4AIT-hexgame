A propos du déroulement
=======================

## Initialisation du jeu

    First, please answer to some questions...
    Please give us grid size (3 - 255) :
    11
    Your grid have now a size of 11
    Please, give us a name for player 1
    Foo
    Foo will be [h]uman, [r]andom IA, [p]ath IA, [m]ind path IA, [b]ruteforce IA ?
    h
    Please, give us a name for player 2
    Bar
    Bar will be [h]uman, [r]andom IA, [p]ath IA, [m]ind path IA, [b]ruteforce IA ?
    m

## Joueur humain

La position X est verticale, la position Y est horizontale.

    It's your turn (turn 1), Foo (Black (Vertically) - Human)
    Give us X position
    0
    Give us Y position
    3
     -  -  -  B  -  -  -  -  -  -  -
       -  -  -  -  -  -  -  -  -  -  -
         -  -  -  -  -  -  -  -  -  -  -
           -  -  -  -  -  -  -  -  -  -  -
             -  -  -  -  -  -  -  -  -  -  -
               -  -  -  -  -  -  -  -  -  -  -
                 -  -  -  -  -  -  -  -  -  -  -
                   -  -  -  -  -  -  -  -  -  -  -
                     -  -  -  -  -  -  -  -  -  -  -
                       -  -  -  -  -  -  -  -  -  -  -
                         -  -  -  -  -  -  -  -  -  -  -

## Intermédiaire

Nous masquerons une bon nombre d'itérations, où les joueurs alternent leurs choix de jeu, toujours sur le même principe.

## Joueur ordinateur

    It's your turn (turn 22), Bar (White (Horizontally) - Mind path AI)
     -  B  -  B  -  -  -  -  -  -  -
       -  B  -  B  -  -  -  -  -  -  -
         -  -  -  B  -  -  -  -  W  W  W
           -  B  B  -  -  W  W  W  -  -  -
             -  B  -  -  W  -  -  -  -  -  -
               W  W  W  W  -  -  -  -  -  -  -
                 -  -  -  -  -  -  -  -  -  -  -
                   -  -  -  -  -  -  -  -  -  -  -
                     B  -  -  -  -  -  -  -  -  -  B
                       -  -  -  -  -  -  -  -  -  -  -
                         -  -  B  -  -  -  -  -  -  -  -

## Conclusion du jeu

    Congratulations, Bar, you've won.