(function() {
    angular.module('Games',['Utils', 'AI'])
        .service('WinService',['InfosService','GridService', function(InfosService, GridService){
            var alreadyChecked;
            var gridSize;
            var winPath;

            var generateAlreadyChecked = function(){
                alreadyChecked = [];
                for(var i = 0; i < InfosService.grid.length; i++){
                    alreadyChecked.push([]);
                }
            };

            var init = function(){
                generateAlreadyChecked();
                gridSize = GridService.getGridSize();
                winPath = [];
            };

            var cellsAroundUnchecked = function(l,c){
                var res = [];
                if(l-1 > 0){
                    if(!alreadyChecked[l-1][c]){
                        res.push([l-1,c]);
                        alreadyChecked[l-1][c]='checked';
                    }
                    if(c+1 <= gridSize){
                        if(!alreadyChecked[l-1][c+1]){
                            res.push([l-1,c+1]);
                            alreadyChecked[l-1][c+1]='checked';
                        }
                    }
                }
                if(c-1 > 0){
                    if(!alreadyChecked[l][c-1]){
                        res.push([l,c-1]);
                        alreadyChecked[l][c-1]='checked';
                    }
                }
                if(!alreadyChecked[l][c]){
                    res.push([l,c]);
                    alreadyChecked[l][c]='checked';
                }
                if(c+1 <= gridSize){
                    if(!alreadyChecked[l][c+1]){
                        res.push([l,c+1]);
                        alreadyChecked[l][c+1]='checked';
                    }
                }
                if(l+1 <= gridSize){
                    if(c-1 > 0){
                        if(!alreadyChecked[l+1][c-1]){
                            res.push([l+1,c-1]);
                            alreadyChecked[l+1][c-1]='checked';
                        }
                    }
                    if(!alreadyChecked[l+1][c]){
                        res.push([l+1,c]);
                        alreadyChecked[l+1][c]='checked';
                    }
                }
                return res;
            };

            var isPlayerCell = function(l,c,player){
                return InfosService[player+"Played"][l][c] !== undefined;
            };

            var isWinCell = function(i){
                return i === gridSize;
            };

            var hasWon = function(l,c,player,winpth){
                var currentWinPath = angular.copy(winpth);
                var res = false;
                alreadyChecked[l][c]='checked';
                if(isPlayerCell(l,c,player)){
                    currentWinPath.push([l,c]);
                    if(player === 'blue' && isWinCell(c)){
                        res = true;
                        winPath = angular.copy(currentWinPath);
                    } else if(player === 'red' && isWinCell(l)){
                        res = true;
                        winPath = angular.copy(currentWinPath);
                    } else {
                        var cellsAround = cellsAroundUnchecked(l,c);
                        angular.forEach(cellsAround,function(cell){
                            if(!res){
                                res = hasWon(cell[0],cell[1],player,currentWinPath);
                            }
                        });
                    }
                }
                return res;
            };

            var drawWinPath = function(){
                angular.forEach(winPath,function(cell){
                    InfosService.grid[cell[0]][cell[1]]='hex-green';
                });
            };


            this.playerWon = function(player){
                init();
                var resultat = false;
                for(var i=1; i<=gridSize; i++){
                    if(player === 'blue'){
                        if(hasWon(i,1,player,winPath)){
                            resultat = true;
                            InfosService.addMessage("Blue won !!\n");
                            drawWinPath();
                            break;
                        }
                    } else {
                        if(hasWon(1,i,player,winPath)){
                            resultat = true;
                            InfosService.addMessage("Red won !!\n");
                            drawWinPath();
                            break;
                        }
                    }
                }
                return resultat;
            }
        }])

        .service('PlayService',['InfosService','PlayRandomService','GridService','WinService','PlayPathService',
            function(InfosService,PlayRandomService,GridService,WinService,PlayPathService){
                var startDate;
                var endDate;
                var init = function(){
                    startDate = new Date();
                    InfosService.addMessage("Starting playing at " + startDate.toLocaleTimeString() +
                    " and " + startDate.getMilliseconds() + "ms\n");
                };
                var finish = function(){
                    endDate = new Date();
                    var diff = endDate.getTime() - startDate.getTime();
                    InfosService.addMessage("Party end at " + endDate.toLocaleTimeString() +
                    " and " + endDate.getMilliseconds() + "ms");
                    InfosService.addMessage("The party lasted " + Math.floor(diff/10000) + "sec " + (diff % 1000) + "ms\n");
                };
                var randomAgainstRandom = function(firstPlayer){
                    var gridSize = GridService.getGridSize();
                    init();
                    for (var i=1; i<=(gridSize*gridSize); i++){
                        var player;
                        InfosService.addMessage("Move n°" + i);
                        if(firstPlayer === 'blue'){
                            if(isOdd(i)){
                                player = 'blue';
                            } else {
                                player = 'red';
                            }
                        } else {
                            if(isOdd(i)){
                                player = 'red';
                            } else {
                                player = 'blue';
                            }
                        }
                        PlayRandomService.play(player);
                        if(WinService.playerWon(player)){
                            break;
                        }
                    }
                    finish();
                };
                var pathAgainstPath = function(firstPlayer){
                    var gridSize = GridService.getGridSize();
                    init();
                    PlayPathService.init();
                    for (var i=1; i<=(gridSize*gridSize); i++){
                        var player;
                        InfosService.addMessage("Move n°" + i);
                        if(firstPlayer === 'blue'){
                            if(isOdd(i)){
                                player = 'blue';
                            } else {
                                player = 'red';
                            }
                        } else {
                            if(isOdd(i)){
                                player = 'red';
                            } else {
                                player = 'blue';
                            }
                        }
                        PlayPathService.play(player);
                        if(WinService.playerWon(player)){
                            break;
                        }
                    }
                    finish();

                };
                this.play = function(firstPlayer, bluePlayer, redPlayer){
                    if (bluePlayer === 'random' && redPlayer === 'random'){
                        randomAgainstRandom(firstPlayer);
                    }
                    if(bluePlayer === 'path'){
                        if(redPlayer === 'path'){
                            pathAgainstPath(firstPlayer);
                        }
                    }
                };
            }]);
})();