/**
 * Created by Robin on 06/03/2015.
 */
(function(){

    function isOdd(n)
    {
        return isNumber(n) && (Math.abs(n) % 2 == 1);
    }

    function isNumber(n)
    {
        return n == parseFloat(n);
    }

    var hex = angular.module('hex',['luegg.directives']);

    hex.factory('InfosService',[function(){
        var service = {
            message: "\n---- Logs ----\n\n",
            unplayedGrid: [],
            grid: [],
            redPlayed: [],
            bluePlayed: []
        };

        service.addMessage = function(msg){
            service.message += msg + "\n";
        };

        service.generateUnplayedGrid = function(size){
            service.unplayedGrid = [];
            for (var i = 0; i < size; i++){
                for (var j = 0; j < size; j++){
                    service.unplayedGrid.push([i+1,j+1]);
                }
            }
        };

        return service;
    }]);

    hex.service('GridService',['InfosService', function(InfosService){
        var gridSize;

        this.setGridSize = function(size){
            gridSize = size;
            InfosService.generateUnplayedGrid(size);
        };

        this.getGridSize = function(){
            return gridSize;
        };

        var init = function(){
            InfosService.grid = [];
            InfosService.redPlayed = [];
            InfosService.bluePlayed = [];
            for(var i=0; i<=gridSize+1;i++){
                InfosService.redPlayed.push([]);
                InfosService.bluePlayed.push([]);
            }
        };

        this.makeGrid = function(){
            init();
            var totalSize = gridSize + 2;
            for (var l = 0; l < totalSize; l++){
                InfosService.grid.push([]);
                for(var c = 0; c < (totalSize); c++){
                    InfosService.grid[l].push([]);
                    if (l === 0 && c === 0 || l === 0 && c === totalSize - 1 ||
                        l === totalSize - 1 && c === 0 || l === totalSize - 1 && c === totalSize - 1){
                        InfosService.grid[l][c] = 'hex-default';
                    } else if (l == 0 || l == totalSize - 1){
                        InfosService.grid[l][c] = 'hex-default-red';
                    } else if (c == 0 || c == totalSize - 1){
                        InfosService.grid[l][c] = 'hex-default-blue';
                    } else {
                        InfosService.grid[l][c] = 'hex-default';
                    }
                }
            }
        };
    }]);

    hex.service('PlayRandomService',['InfosService',function(InfosService){
        this.play = function(player){
            var startDate = new Date();
            var random = Math.floor((Math.random() * InfosService.unplayedGrid.length));
            var l = InfosService.unplayedGrid[random][0];
            var c = InfosService.unplayedGrid[random][1];
            InfosService.grid[l][c] = 'hex-' + player;
            InfosService.unplayedGrid.splice(random,1);
            InfosService[player+"Played"][l][c]="played";
            var endDate = new Date();
            var ms = endDate.getTime() - startDate.getTime();
            InfosService.addMessage("Move done by "+ player +" in " + Math.floor(ms/1000) + "sec " + (ms % 1000) +"ms " +
            "[" + l + ","+ c + "]\n");
        };
    }]);

    hex.service('PlayPathService',['InfosService','GridService',function(InfosService,GridService){
        var bluePreviousCells = [];
        var redPreviousCells = [];

        this.init = function(){
          bluePreviousCells = [];
            redPreviousCells = [];
        };

        var isPlaybable = function(l,c){
            var res = false;
            angular.forEach(InfosService.unplayedGrid, function(unplayedCell){
                if(unplayedCell[0] == l && unplayedCell[1]==c){
                    res = true
                }
            });
            return res;
        };

        var bestCells = function(l,c,player){
            var res = [];
            if(player === 'blue'){
                if(isPlaybable(l-1,c+1)){
                    res.push([l-1,c+1])
                }
                if(isPlaybable(l,c+1)){
                    res.push([l,c+1])
                }
            } else {
                if(isPlaybable(l+1,c-1)){
                    res.push([l+1,c-1])
                }
                if(isPlaybable(l+1,c)){
                    res.push([l+1,c])
                }
            }
            return res;
        };

        var otherCells = function(l,c,player){
            var res = [];
            if(player === 'blue'){
                if(isPlaybable(l-1,c)){
                    res.push([l-1,c])
                }
                if(isPlaybable(l+1,c)){
                    res.push([l+1,c])
                }
            } else {
                if(isPlaybable(l,c-1)){
                    res.push([l,c-1])
                }
                if(isPlaybable(l,c+1)){
                    res.push([l,c+1])
                }
            }
            return res;
        };

        var defaultCells = function(l,c,player){
            var res = [];
            if(player === 'blue'){
                if(isPlaybable(l,c-1)){
                    res.push([l,c-1])
                }
                if(isPlaybable(l+1,c-1)){
                    res.push([l+1,c-1])
                }
            } else {
                if(isPlaybable(l-1,c)){
                    res.push([l-1,c])
                }
                if(isPlaybable(l-1,c+1)){
                    res.push([l-1,c+1])
                }
            }
            return res;
        };

        var run = function(player){
            var cell;
            if(player === 'blue'){
                if(bluePreviousCells.length == 0){
                    var startCell = [Math.floor(GridService.getGridSize()/2),1];
                    if(isPlaybable(startCell)){
                        cell=startCell;
                    } else {
                        cell=[startCell[0]+1,startCell[1]];
                    }
                } else {
                    for(var i=bluePreviousCells.length - 1; i >= 0; i--){
                        var currentCell = bluePreviousCells[i];
                        var l = currentCell[0];
                        var c =  currentCell[1];
                        var bests = bestCells(l,c,player);
                        var others = otherCells(l,c,player);
                        var defaults = defaultCells(l,c,player);
                        if(bests.length != 0){
                            random = Math.floor((Math.random() * bests.length));
                            cell = [bests[random][0],bests[random][1]];
                        } else if(others.length != 0){
                            random = Math.floor((Math.random() * others.length));
                            cell = [others[random][0],others[random][1]];
                        } else if(defaults.length != 0){
                            random = Math.floor((Math.random() * defaults.length));
                            cell = [defaults[random][0],defaults[random][1]];
                        } else {
                            random = Math.floor((Math.random() * InfosService.unplayedGrid.length));
                            cell=[InfosService.unplayedGrid[random][0],InfosService.unplayedGrid[random][1]];
                        }
                    }
                }
                bluePreviousCells.push(cell);
            } else {
                if(redPreviousCells.length == 0){
                    startCell = [1,Math.floor(GridService.getGridSize()/2)];
                    if(isPlaybable(startCell)){
                        cell=startCell;
                    } else {
                        cell=[startCell[0],startCell[1]+1];
                    }
                } else {
                    for(i=redPreviousCells.length - 1; i >= 0; i--){
                        currentCell = redPreviousCells[i];
                        l = currentCell[0];
                        c =  currentCell[1];
                        bests = bestCells(l,c,player);
                        others = otherCells(l,c,player);
                        defaults = defaultCells(l,c,player);
                        if(bests.length != 0){
                            random = Math.floor((Math.random() * bests.length));
                            cell = [bests[random][0],bests[random][1]];
                        } else if(others.length != 0){
                            random = Math.floor((Math.random() * others.length));
                            cell = [others[random][0],others[random][1]];
                        } else if(defaults.length != 0){
                            random = Math.floor((Math.random() * defaults.length));
                            cell = [defaults[random][0],defaults[random][1]];
                        } else {
                            random = Math.floor((Math.random() * InfosService.unplayedGrid.length));
                            cell=[InfosService.unplayedGrid[random][0],InfosService.unplayedGrid[random][1]];
                        }
                    }
                }
                redPreviousCells.push(cell);
            }
            return cell;
        };

        var indexUnplayedGridCell = function(l,c){
            var res;
            angular.forEach(InfosService.unplayedGrid, function(unplayedCell, index){
                if(unplayedCell[0] == l && unplayedCell[1]==c){
                    res = index;
                }
            });
            return res;
        };

        this.play = function(player){
            var startDate = new Date();
            var cell = run(player);
            InfosService.grid[cell[0]][cell[1]] = 'hex-' + player;
            var index = indexUnplayedGridCell(cell[0],cell[1]);
            InfosService.unplayedGrid.splice(index,1);
            InfosService[player+"Played"][cell[0]][cell[1]]="played";
            var endDate = new Date();
            var ms = endDate.getTime() - startDate.getTime();
            InfosService.addMessage("Move done by "+ player +" in " + Math.floor(ms/1000) + "sec " + (ms % 1000) +"ms " +
            "[" + cell[0] + ","+ cell[1] + "]\n");

        }
    }]);

    hex.service('PlayService',['InfosService','PlayRandomService','GridService','WinService','PlayPathService',
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

    hex.service('WinService',['InfosService','GridService', function(InfosService, GridService){
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
    }]);

    hex.controller('GameController',['$scope','GridService','InfosService','PlayService',
        function($scope,GridService,InfosService,PlayService){
            me = this;
            me.gridSize=3;
            me.infos=InfosService;
            me.firstPlayer = 'blue';
            me.bluePlayer = 'path';
            me.redPlayer = 'path';

            var init = function(){
                InfosService.message = "\n---- Logs ----\n\n";
                GridService.setGridSize(me.gridSize);
                GridService.makeGrid();
            };

            me.goodToPlay = function(){
                return isOdd(me.gridSize)&&me.gridSize>0;
            };

            me.start = function(){
                init();
                PlayService.play(me.firstPlayer, me.bluePlayer, me.redPlayer);
            };
        }]);
})();