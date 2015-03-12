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

    var hex = angular.module('hex',[]);

    hex.factory('InfosService',[function(){
        var service = {
            message: "\n---- Logs ----\n\n",
            unplayedGrid: [],
            grid: []
        };

        service.addMessage = function(msg){
            service.message += msg + "\n";
        };

        service.generateUnplayedGrid = function(size){
            for (var i = 0; i < size; i++){
                for (var j = 0; j < size; j++){
                    service.unplayedGrid.push([i+1,j+1]);
                }
            }
        };

        service.sliceUnplayedGrid = function(index){
            service.unplayedGrid.splice(index,1);
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
            InfosService.sliceUnplayedGrid(random);
            var endDate = new Date();
            var ms = endDate.getTime() - startDate.getTime();
            InfosService.addMessage("Move done in " + Math.floor(ms/1000) + "sec " + (ms % 1000) +"ms\n");
        };
    }]);

    hex.service('PlayService',['InfosService','PlayRandomService','GridService',
        function(InfosService,PlayRandomService,GridService){
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
                    InfosService.addMessage("Move n°" + i);
                    if(firstPlayer === 'blue'){
                        if(isOdd(i)){
                            PlayRandomService.play('blue');
                        } else {
                            PlayRandomService.play('red');
                        }
                    } else {
                        if(isOdd(i)){
                            PlayRandomService.play('red');
                        } else {
                            PlayRandomService.play('blue');
                        }
                    }
                }
                finish();

            };
            this.play = function(firstPlayer, bluePlayer, redPlayer){
                if (bluePlayer === 'random' && redPlayer === 'random'){
                    randomAgainstRandom(firstPlayer);
                }
            };
        }]);

    hex.controller('GameController',['$scope','GridService','InfosService','PlayService',
        function($scope,GridService,InfosService,PlayService){
            me = this;
            me.gridSize=3;
            me.infos=InfosService;
            me.firstPlayer = 'blue';
            me.bluePlayer = 'random';
            me.redPlayer = 'random';

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

            function haveAWinner(){
                var res = false;
                for(var i=1;i<= me.gridSize;i++){
                    var resB = blueCheckArrond(i,1);
                    if(resB){
                        res = true;
                        break;
                    }
                    console.log(i);
                    var resR = redCheckArrond(1,i);
                    if(resR){
                        res = true;
                        break;
                    }
                }
                return res;
            }

            function redCheckArrond(l,c,cellsChecked, winPath){
                if(me.grid[l][c].class !== 'hex-red'){
                    return false;
                }
                if(!cellsChecked){
                    cellsChecked = [];
                    winPath = [];
                }
                winPath.push([l,c]);
                if(l+1 == me.grid.length-1){
                    drawWinPath(winPath);
                    me.infos+="Red wins !! \n";
                    return true;
                }
                if(!cellsChecked[l]){
                    cellsChecked[l]=[];
                }
                cellsChecked[l][c]="done";
                if(c-1 !== 0 ){
                    if(!cellsChecked[l+1] || !cellsChecked[l+1][c-1]){
                        var firstCheck = redCheckArrond(l+1,c-1,cellsChecked, winPath);
                        if(firstCheck){
                            return true;
                        }
                    }
                }
                if(!cellsChecked[l+1] || !cellsChecked[l+1][c]){
                    return redCheckArrond(l+1,c,cellsChecked, winPath)
                }
            }

            function drawWinPath(path){
                angular.forEach(path, function(coor){
                    me.grid[coor[0]][coor[1]].class = 'hex-green';
                })
            }

            function blueCheckArrond(l,c,cellsChecked, winPath){
                if(me.grid[l][c].class !== 'hex-blue'){
                    return false;
                }
                if(!cellsChecked){
                    cellsChecked = [];
                    winPath = [];
                }
                winPath.push([l,c]);
                if(c+1 == me.grid.length-1){
                    drawWinPath(winPath);
                    me.infos+="Blue wins !! \n";
                    return true;
                }
                if(!cellsChecked[l]){
                    cellsChecked[l]=[];
                }
                cellsChecked[l][c]="done";
                if(l-1 !== 0 ){
                    if(!cellsChecked[l-1] || !cellsChecked[l-1][c+1]){
                        var firstCheck = blueCheckArrond(l-1,c+1,cellsChecked, winPath);
                        if(firstCheck){
                            return true;
                        }
                    }
                }
                if(!cellsChecked[l] || !cellsChecked[l][c+1]){
                    return blueCheckArrond(l,c+1,cellsChecked, winPath);
                }
            }

            //function haveAWinner(l,c){
            //    console.log(l,c);
            //    console.log('blue',angular.copy(blueGrid));
            //    console.log('red',angular.copy(redGrid));
            //    var res = false;
            //    if(blueGrid[l]){
            //        console.log('blue length',blueGrid[l].length);
            //        if(blueGrid[l].length == me.gridSize){
            //            me.infos += "Blue wins !! \n";
            //            res = true;
            //        }
            //    }
            //    if(redGrid[c]){
            //        console.log('red length', redGrid[c].length);
            //        if(redGrid[c].length == me.gridSize){
            //            me.infos += "Red wins !! \n";
            //            res = true;
            //        }
            //    }
            //    return res;
            //}

        }]);
})();