Hex game
=======

Le jeu de hex est un jeu de société pour deux joueurs, il se joue sur un plateau en forme de losange.
Le but est de joindre les bords, deux à deux. Ainsi un joueur doit joindre les bords droite et gauche, et l'autre les
bords haut et bas.

John Nash a étudié ce jeu et prouvé que ce jeu ne peut avoir de match nul.

Chaque joueur joue chacun son tour, en plaçant un pion sur le plateau.

## Notre implémentation

Dans cette implémentation, au lancement, il faut choisir une taille de grille entre 3 et 255
(limite technique pour des raisons de performances. Ceci permet d'utiliser des unsigned integer de 8 bits.)
Ensuite, il faut entrer, pour chaque joueur, un nom de joueur et son type d'implémentation (Humain, "Random IA",
"Path IA", "Mind path IA", "bruteforce IA"). Ces types d'implémentation seront détaillées dans l'autre manuel.
Pour cela, il faut entrer la première lettre de l'implémentation.
