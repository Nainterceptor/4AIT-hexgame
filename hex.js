/**
 * Created by Robin on 06/03/2015.
 */
(function(){
    var hex = angular.module('hex',[]);
    hex.controller('GameController',['$scope', function($scope){
        me = this;
        me.gridSize=3;
        me.grid = [];
        me.infos="";
        var randomGrid = [];
        var blueGrid = {};
        var redGrid = {};

        function init(){
            randomGrid = [];
            me.grid = [];
            blueGrid = {};
            redGrid = {};
        }

        function makeGrid(){
            init();
            var grid = [];
            var totalSize = me.gridSize + 2;
            for (var l = 0; l<(totalSize); l++){
                grid.push([]);
                randomGrid.push({value:l,col:[]});
                for(var c = 0; c < (totalSize); c++){
                    grid[l].push({});
                    randomGrid[l].col.push(c);
                    if (l === 0 && c === 0 || l === 0 && c === totalSize - 1 ||
                        l === totalSize - 1 && c === 0 || l === totalSize - 1 && c === totalSize - 1){
                        grid[l][c].class = 'hex-default';
                    } else if (l == 0 || l == totalSize - 1){
                        grid[l][c].class = 'hex-default-red';
                    } else if (c == 0 || c == totalSize - 1){
                        grid[l][c].class = 'hex-default-blue';
                    } else {
                        grid[l][c].class = 'hex-default';
                    }
                }
            }
            me.grid = angular.copy(grid);
        }

        me.goodToPlay = function(){
          return isOdd(me.gridSize)&&me.gridSize>0;
        };

        me.play = function(){
            makeGrid();
            playRandomly();
        };

        function playRandomly (){
            me.infos="Playing \n";
            for (var i=1; i<=(me.gridSize*me.gridSize);i++){
                me.infos += "Count n° " + i + "\n";
                var l = Math.floor((Math.random() * (randomGrid.length - 2)) + 1);
                var c = Math.floor((Math.random() * (randomGrid[l].col.length - 2)) + 1);
                var lign = randomGrid[l].value;
                var col = randomGrid[l].col[c];

                if (isOdd(i)){
                    me.grid[lign][col].class = 'hex-blue';
                    //if(!blueGrid[lign]){
                    //    blueGrid[lign]=[];
                    //}
                    //blueGrid[lign].push(col);
                } else {
                    me.grid[lign][col].class = 'hex-red';
                    //if(!redGrid[col]){
                    //    redGrid[col]=[];
                    //}
                    //redGrid[col].push(lign);
                }
                if(haveAWinner()){
                    break;
                }
                randomGrid[l].col.splice(c,1);
                if (randomGrid[l].col.length === 2){
                    randomGrid.splice(l,1);
                }
            }
        }

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

        function isOdd(n)
        {
            return isNumber(n) && (Math.abs(n) % 2 == 1);
        }

        function isNumber(n)
        {
            return n == parseFloat(n);
        }
    }]);
})();