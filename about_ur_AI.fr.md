Nos implémentations
===================

Dans notre cas, nous avons focalisés sur le fait qu'une IA doit être un moyen pour un joueur de gagner.

## Humain

Pour rester cohérent, vis à vis de la machine, on peut voir du coup l'humain comme une implémentation. En effet,
la machine "requête" l'humain en vue d'obtenir la coordonnée suivante. Ainsi, dans cette implémentation, à chaque tour,
l'utilisateur devra entrer une coordonnée, X et Y. De simples contrôles vérifient que la coordonnée est bien libre
et dans la matrice.

Cette "IA" est probablement la plus rapide et pertinente de par sont utilisation de l'intelligence humaine.

## Random IA

L'IA la plus simple peut être contre intuitive. En effet, d'après les études de John Nash, il ne peut y avoir d'égalité
au jeu de hex. Donc, si on rempli de manière aléatoire une grille, il y aura toujours un gagnant. Il suffit donc de
faire une liste des cases vides, de la mélanger, et en prenant le premier résultat, nous aurons un choix aléatoire
de case. Cette méthode est la meilleure pour avoir un tirage sans remise, et éviter de gaspiller aléatoirement
des ressources en prenant des coordonnées X et Y au hazard.

Cette méthode est rapide mais peu pertinente. Elle sera très probablement perdante face à toute autre IA, et gagnante
uniquement contre elle même.

Cette IA utilise uniquement la grille.

### Benchmark

    test tests::bench::bench_random_ia... bench:  27 943 525 ns/iter (+/- 5603072)
    test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured

## Path IA

Cette méthode est une amélioration de la Random IA. il s'agit de construire un chemin. On prend au hazard un point
au hazard sur la "ligne de départ", ensuite, à chaque tour, on part du dernier coup joué pour déterminer,
par un système de poids, le prochain coup à jouer. En effet, quand on se rapproche du but, le poids est de +1,
quand on évolue parallèlement, le poids est de 0 et quand on recule, le poids est de -1.
Le choix ayant le poids le plus élevé est priorisé. Quand deux choix ont le même poids, étant donné que cette IA n'a
aucune capacité d'anticipation, un choix est pris au hazard. Si elle se trouve dans une impasse, elle fallback
sur la Random IA pour le coup suivant.

Cette IA utilise la grille + la liste des coups joués.

Cette méthode est la plus rapide mais également peu pertinente. Elle sera perdante contre toute IA qui lui coupe le passage.

### Benchmark

    test tests::bench::bench_path_ia... bench:   5 899 152 ns/iter (+/- 2247232)
    test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured

## Bruteforce IA

Cette méthode est probablement la plus pertinente et la moins prévisible pour le joueur qu'elle affronte. Cependant,
elle est aussi la plus lente. Pour tenir une partie contre un humain, il faudrait une forte puissance de calcul.
L'idée est simple. On fait une liste de choix possibles aléatoire (CF Random IA). Pour chacun de ces choix, on vérifie
tous les chemins possibles de la grille. chaque chemin est pondéré de 0 ou 1 comme précédemment.
Quand une case est prise par l'adversaire, elle n'apparait pas dans les cellules connexes.
Ainsi, il est possible de trouver le chemin avec le plus bas coùt. Le choix avec le chemin ayant le plus bas coùt est
donc forcement le meilleur choix.

Cet algorithme se base uniquement sur la grille + n simulations de grille

Des pistes d'amélioration ont été envisagées mais non implémentées : Ignorer les chemins trop long(Problématique :
A partir de quel poids un chemin est trop long ?), Prioriser la recherche sur un "cone" en direction du but.

### Benchmark

Ce bench a été fait sur une grille réduite de 4x4

    test tests::bench::bench_bruteforce_ia... bench: 524 053 195 ns/iter (+/- 69966562)
    test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured


## Mind Path IA

Cette méthode est une amélioration de la Path IA. Le principe reste le même sauf que l'IA garde un objectif en mémoire
qu'elle doit remplir.

A l'initialisation de la partie, un chemin est calcul au hazard comme le Path IA. Ce chemin est gardé en mémoire. Il
constitue l'objectif. A chaque tour de jeu, une case vide de ce chemin est remplie. Si jamais ce chemin est franchi par
un pion adverse, il est alors recalculé à partir du dernier pion placé. Le calcul est proche du bruteforce IA,
mais pas aussi coùteux de par le nombre restreint de vérifications. Si le chemin est obstrué, on retire de la mémoire
le dernier pion pour recommencer le calcul.

Cet algorithme se base donc sur : La grille + Le chemin previsionnel

Cette méthode est directement issue du fonctionnement d'un joueur humain. Elle pourrait être améliorée en ajoutant
une sensitivité autour du chemin

### Benchmark

    test tests::bench::bench_mind_path_ia... bench:   9 733 885 ns/iter (+/- 8422082)
    test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured

## Autres IA

D'autres IA ont été envisagées lors de notre recherche préliminaire. Principalement basées sur le placement de poids
déterminant le prochain choix. Chaque placement mettant à jour les poids du plateau de jeu.