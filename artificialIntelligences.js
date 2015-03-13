(function() {
    angular.module('AI',['Utils'])
        .service('PlayRandomService',['InfosService',function(InfosService){
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
                InfosService.addMessage("Move done by "+ player +" with random method in " + Math.floor(ms/1000) + "sec " + (ms % 1000) +"ms " +
                "[" + l + ","+ c + "]\n");
            };
        }])

    .service('PlayPathService',['InfosService','GridService',function(InfosService,GridService){
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
            InfosService.addMessage("Move done by "+ player +" with path method in " + Math.floor(ms/1000) + "sec " + (ms % 1000) +"ms " +
            "[" + cell[0] + ","+ cell[1] + "]\n");

        }
    }]);
})();